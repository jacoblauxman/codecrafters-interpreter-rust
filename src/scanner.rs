use crate::{Token, TokenType};

pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub errors: Vec<String>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: vec![],
            errors: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), None, self.line))
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFTPAREN, None),
            ')' => self.add_token(TokenType::RIGHTPAREN, None),
            '{' => self.add_token(TokenType::LEFTBRACE, None),
            '}' => self.add_token(TokenType::RIGHTBRACE, None),
            ',' => self.add_token(TokenType::COMMA, None),
            '.' => self.add_token(TokenType::DOT, None),
            '-' => self.add_token(TokenType::MINUS, None),
            '+' => self.add_token(TokenType::PLUS, None),
            ';' => self.add_token(TokenType::SEMICOLON, None),
            '*' => self.add_token(TokenType::STAR, None),

            '!' => match self.operator_match('=') {
                true => self.add_token(TokenType::NOTEQUAL, None),
                false => self.add_token(TokenType::BANG, None),
            },
            '=' => match self.operator_match('=') {
                true => self.add_token(TokenType::EQUAL, None),
                false => self.add_token(TokenType::ASSIGN, None),
            },
            '<' => match self.operator_match('=') {
                true => self.add_token(TokenType::LESSEQUAL, None),
                false => self.add_token(TokenType::LESS, None),
            },
            '>' => match self.operator_match('=') {
                true => self.add_token(TokenType::GREATEREQUAL, None),
                false => self.add_token(TokenType::GREATER, None),
            },
            '/' => {
                // advance through comment in code
                if self.operator_match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH, None)
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            unknown => self.errors.push(format!(
                "[line {}] Error: Unexpected character: {}",
                self.line, unknown
            )),
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors
                .push(format!("[line {}] Error: Unterminated string.", self.line));
            return;
        }

        self.advance(); // closing '"'

        let literal = &self.source[self.start + 1..self.current - 1];

        self.add_token(TokenType::STRING, Some(literal.to_string()));
    }

    fn operator_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = &self.source[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line))
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
