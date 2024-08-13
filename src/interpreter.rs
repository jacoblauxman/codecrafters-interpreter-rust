use crate::{Expr, Token, TokenType};
use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
#[error("[line {line}] Error with `{token}`: {message}")]
pub struct RuntimeError {
    token: String,
    message: String,
    line: usize,
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

pub struct Interpreter;

impl Interpreter {
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
        }
    }

    fn evaluate_unary(&self, operator: &Token, right: &Expr) -> Result<ExprValue, RuntimeError> {
        let right = self.evaluate(right)?;

        match operator.token_type {
            TokenType::BANG => Ok(ExprValue::Bool(self.is_truthy(&right))),
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
