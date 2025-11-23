use std::fmt;

pub mod lexer;
pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Plus,
    Minus,
    Star,
    Slash,
    Bang,
    BangEqual,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Or,

    String(String),
    Number(i32),
    True,
    False,

    LeftParen,
    RightParen,

    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    // Comparison
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Equal,
    NotEqual,
    // Logical
    And,
    Or,
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    String(String),
    Bool(bool),

    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },

    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    Grouping(Box<Expr>),
}

#[derive(Debug)]
pub struct SyntaxError {
    pub message: String,
    pub line: u32,
    pub column: u32,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}: {}", self.line, self.column, self.message)
    }
}

impl SyntaxError {
    fn new(message: String, line: u32, column: u32) -> SyntaxError {
        return SyntaxError {
            message,
            line,
            column,
        };
    }
}

impl std::error::Error for SyntaxError {}
