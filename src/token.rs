#![allow(dead_code)]

use phf::phf_map;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: i32) -> Token {
        Self {
            token_type,
            lexeme: lexeme.to_owned(),
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.token_type {
            TokenType::STRING { literal } => write!(f, "String {:?} {:?}", self.lexeme, literal),
            TokenType::NUMBER { literal } => write!(f, "String {:?} {:?}", self.lexeme, literal),
            _ => write!(f, "{:?} {:?}", self.token_type, self.lexeme),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING { literal: String },
    NUMBER { literal: f64 },

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
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

pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {

    "and" => TokenType::AND,
    "class" => TokenType::CLASS,
    "else" => TokenType::ELSE,
    "false" => TokenType::FALSE,
    "for" => TokenType::FOR,
    "fun" => TokenType::FUN,
    "if" => TokenType::IF,
    "nil" => TokenType::NIL,
    "or" => TokenType::OR,
    "print" => TokenType::PRINT,
    "return" => TokenType::RETURN,
    "super" => TokenType::SUPER,
    "this" => TokenType::THIS,
    "true" => TokenType::TRUE,
    "var" => TokenType::VAR,
    "while" => TokenType::WHILE,


};
