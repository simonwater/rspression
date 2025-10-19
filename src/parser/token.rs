use crate::values::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Percent,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Star,
    StarStar,
    And,
    Or,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Null,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub literal: Option<Value>,
    pub line: usize,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'a str,
        literal: Option<Value>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Self {
            token_type: TokenType::Error,
            lexeme: "",
            literal: None,
            line: 0,
        }
    }
}
