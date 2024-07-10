use std::fmt;

#[derive(Clone, Debug)]
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

    ASSIGN,
    BANG,
    EQUAL,
    NOTEQUAL,

    LESS,
    LESSEQUAL,
    GREATER,
    GREATEREQUAL,

    SLASH,

    STRING,
    NUMBER,

    IDENTIFIER,

    AND,
    CLASS,
    ELSE,
    FALSE,
    FOR,
    FUN,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

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
            TokenType::ASSIGN => "EQUAL", // difference for testing
            TokenType::BANG => "BANG",
            TokenType::EQUAL => "EQUAL_EQUAL",
            TokenType::NOTEQUAL => "BANG_EQUAL",
            TokenType::LESS => "LESS",
            TokenType::LESSEQUAL => "LESS_EQUAL",
            TokenType::GREATER => "GREATER",
            TokenType::GREATEREQUAL => "GREATER_EQUAL",
            TokenType::SLASH => "SLASH",
            TokenType::STRING => "STRING",
            TokenType::NUMBER => "NUMBER",
            TokenType::IDENTIFIER => "IDENTIFIER",
            TokenType::AND => "AND",
            TokenType::CLASS => "CLASS",
            TokenType::ELSE => "ELSE",
            TokenType::FALSE => "FALSE",
            TokenType::FOR => "FOR",
            TokenType::FUN => "FUN",
            TokenType::IF => "IF",
            TokenType::NIL => "NIL",
            TokenType::OR => "OR",
            TokenType::PRINT => "PRINT",
            TokenType::RETURN => "RETURN",
            TokenType::SUPER => "SUPER",
            TokenType::THIS => "THIS",
            TokenType::TRUE => "TRUE",
            TokenType::VAR => "VAR",
            TokenType::WHILE => "WHILE",
            TokenType::EOF => "EOF",
        };

        write!(f, "{}", token_type_str)
    }
}

#[derive(Clone, Debug)]
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
        let literal = self.literal.as_deref().unwrap_or("null");
        write!(f, "{} {} {}", self.token_type, self.lexeme, literal)
    }
}
