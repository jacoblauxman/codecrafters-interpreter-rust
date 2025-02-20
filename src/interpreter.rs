use crate::Environment;
use crate::{Expr, Stmt, Token, TokenType};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

#[derive(Debug, thiserror::Error)]
#[error("[line {line}] Error with `{token}`: {message}")]
pub struct RuntimeError {
    pub token: String,
    pub message: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprValue {
    Bool(bool),
    Number(f64),
    String(String),
    Nil,
}

impl Display for ExprValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprValue::Bool(b) => write!(f, "{b}"),
            ExprValue::Number(n) => {
                // todo: handle '.0' decimal?
                write!(f, "{n}")
            }
            ExprValue::String(s) => write!(f, "{s}"),
            ExprValue::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
enum InterpreterStatus {
    #[default]
    Evaluate,
    Run,
}

impl TryFrom<&str> for InterpreterStatus {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "evaluate" => Ok(InterpreterStatus::Evaluate),
            "run" => Ok(InterpreterStatus::Run),
            _ => Err("should only accept `evaluate` and `run` string values".to_string()),
        }
    }
}

#[derive(Default)]
pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
    status: InterpreterStatus,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
            status: InterpreterStatus::Evaluate,
        }
    }

    pub fn set_env(&mut self, environment: Rc<RefCell<Environment>>) {
        self.environment = environment;
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), RuntimeError> {
        for statement in statements.iter() {
            self.execute(statement)?;
        }

        Ok(())
    }

    pub fn set_status(&mut self, status: &str) -> Result<(), String> {
        let status = InterpreterStatus::try_from(status)?;
        self.status = status;

        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(_) => self.eval_expr_stmt(stmt),
            Stmt::Print(_) => self.eval_print_stmt(stmt),
            Stmt::Var(name, initializer) => self.eval_var_stmt(name, initializer),
            Stmt::Block(statements) => self.eval_block_stmt(statements),
        }
    }

    fn eval_block_stmt(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        let prev_env = self.environment.clone();

        self.set_env(Rc::new(RefCell::new(Environment::with_enclosing(
            prev_env.clone(),
        ))));

        let block_eval: Result<(), RuntimeError> = (|| {
            for stmt in statements.iter() {
                self.execute(stmt)?;
            }
            Ok(())
        })();

        self.set_env(prev_env);
        block_eval
    }

    fn eval_expr_stmt(&self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => {
                let stmt = self.evaluate(expr)?;
                if self.status == InterpreterStatus::Evaluate {
                    println!("{}", stmt);
                }
                Ok(())
            }
            _ => unreachable!("use with expression statements only!"),
        }
    }

    fn eval_print_stmt(&self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Print(expr) => {
                let stmt = self.evaluate(expr)?;
                println!("{}", stmt);
                Ok(())
            }
            _ => unreachable!("use with print statements only!"),
        }
    }

    fn eval_var_stmt(&self, name: &Token, initializer: &Expr) -> Result<(), RuntimeError> {
        let expr = self.evaluate(initializer)?;
        self.environment
            .borrow_mut()
            .define(name.lexeme.clone(), expr);
        Ok(())
    }

    pub fn evaluate(&self, expr: &Expr) -> Result<ExprValue, RuntimeError> {
        match expr {
            Expr::Bool(b) => Ok(ExprValue::Bool(*b)),
            Expr::Number(n) => Ok(ExprValue::Number(*n)),
            Expr::String(s) => Ok(ExprValue::String(s.to_owned())),
            Expr::Nil => Ok(ExprValue::Nil),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary { operator, right } => self.evaluate_unary(operator, right),
            Expr::Binary {
                operator,
                right,
                left,
            } => self.evaluate_binary(operator, left, right),
            //
            Expr::Variable(name) => self.environment.borrow().get(name),
            Expr::Assign(name, val) => {
                let val = self.evaluate(val)?;
                self.environment.borrow_mut().assign(name, val.clone())?;
                Ok(val)
            }
        }
    }

    fn evaluate_unary(&self, operator: &Token, right: &Expr) -> Result<ExprValue, RuntimeError> {
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::BANG => Ok(ExprValue::Bool(!self.is_truthy(&right))),
            TokenType::MINUS => {
                let expr_num = self.check_num_operand(operator, &right)?;
                Ok(ExprValue::Number(-expr_num))
            }
            _ => Err(RuntimeError {
                token: operator.to_string(),
                message: "Invalid operator found in unary expression".to_string(),
                line: operator.line,
            }),
        }
    }

    fn evaluate_binary(
        &self,
        operator: &Token,
        left: &Expr,
        right: &Expr,
    ) -> Result<ExprValue, RuntimeError> {
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::GREATER => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Bool(left > right))
            }
            TokenType::GREATEREQUAL => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Bool(left >= right))
            }
            TokenType::LESS => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Bool(left < right))
            }
            TokenType::LESSEQUAL => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Bool(left <= right))
            }
            TokenType::MINUS => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Number(left - right))
            }
            TokenType::PLUS => match (left, right) {
                (ExprValue::Number(left), ExprValue::Number(right)) => {
                    Ok(ExprValue::Number(left + right))
                }
                (ExprValue::String(left), ExprValue::String(right)) => {
                    let expr_val = left + &right;
                    Ok(ExprValue::String(expr_val))
                }
                _ => Err(RuntimeError {
                    token: operator.lexeme.to_string(),
                    message: "Operands must be two numbers or two strings.".to_string(),
                    line: operator.line,
                }),
            },
            TokenType::SLASH => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Number(left / right))
            }
            TokenType::STAR => {
                let (left, right) = self.check_num_operands(operator, &left, &right)?;
                Ok(ExprValue::Number(left * right))
            }
            TokenType::NOTEQUAL => Ok(ExprValue::Bool(!self.is_equal(&left, &right))),
            TokenType::EQUAL => Ok(ExprValue::Bool(self.is_equal(&left, &right))),
            _ => Err(RuntimeError {
                token: operator.lexeme.to_string(),
                message: "Unrecognized binary operator.".to_string(),
                line: operator.line,
            }),
        }
    }

    // helpers
    fn check_num_operand(
        &self,
        operator: &Token,
        expr_val: &ExprValue,
    ) -> Result<f64, RuntimeError> {
        match expr_val {
            ExprValue::Number(n) => Ok(*n),
            _ => Err(RuntimeError {
                token: operator.lexeme.to_string(),
                message: "Operand must be a number.".to_string(),
                line: operator.line,
            }),
        }
    }

    fn check_num_operands(
        &self,
        operator: &Token,
        left: &ExprValue,
        right: &ExprValue,
    ) -> Result<(f64, f64), RuntimeError> {
        match (left, right) {
            (ExprValue::Number(left), ExprValue::Number(right)) => Ok((*left, *right)),
            _ => Err(RuntimeError {
                token: operator.lexeme.to_string(),
                message: "Operands must be numbers".to_string(),
                line: operator.line,
            }),
        }
    }

    fn is_truthy(&self, expr_val: &ExprValue) -> bool {
        match expr_val {
            ExprValue::Nil => false,
            ExprValue::Bool(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, left: &ExprValue, right: &ExprValue) -> bool {
        match (left, right) {
            (ExprValue::Nil, ExprValue::Nil) => true,
            (ExprValue::Bool(a), ExprValue::Bool(b)) => a == b,
            (ExprValue::Number(a), ExprValue::Number(b)) => (a - b).abs() < f64::EPSILON,
            (ExprValue::String(a), ExprValue::String(b)) => a == b,
            _ => false,
        }
    }
}
