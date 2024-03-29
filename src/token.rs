use crate::expr::LiteralType;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
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
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralType>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: impl ToString, line: usize) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            literal: None,
            line,
        }
    }

    pub fn new_literal(
        token_type: TokenType,
        lexeme: impl ToString,
        literal: LiteralType,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            literal: Some(literal),
            line,
        }
    }
}
