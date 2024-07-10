use std::fmt;

pub enum TokenType {
    LEFTPAREN,
    RIGHTPAREN,
    LEFTBRACE,
    RIGHTBRACE,

    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    STAR,

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_type_str = match self {
            TokenType::LEFTPAREN => "LEFT_PAREN",
            TokenType::RIGHTPAREN => "RIGHT_PAREN",
            TokenType::LEFTBRACE => "LEFT_BRACE",
            TokenType::RIGHTBRACE => "RIGHT_BRACE",
            TokenType::COMMA => "COMMA",
            TokenType::DOT => "DOT",
            TokenType::MINUS => "MINUS",
            TokenType::PLUS => "PLUS",
            TokenType::SEMICOLON => "SEMICOLON",
            TokenType::STAR => "STAR",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", token_type_str)
    }
}

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<String>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = self.literal.as_deref().unwrap_or("null");
        write!(f, "{} {} {}", self.token_type, self.lexeme, token_str)
    }
}
