use std::convert::TryInto;

use crate::token::Token;
use crate::token::TokenType;

pub struct Scanner {
    tokens: Vec<Token>,
    chars_vec: Vec<char>,
    source: String,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            tokens: Vec::new(),
            chars_vec: source.chars().collect(),
            source: source.to_owned(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, "", self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            ';' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '!' => match self.next_match('=') {
                true => self.add_token(TokenType::BANG_EQUAL),
                false => self.add_token(TokenType::BANG),
            },
            '=' => match self.next_match('=') {
                true => self.add_token(TokenType::EQUAL_EQUAL),
                false => self.add_token(TokenType::EQUAL),
            },
            '<' => match self.next_match('=') {
                true => self.add_token(TokenType::LESS),
                false => self.add_token(TokenType::LESS_EQUAL),
            },
            '>' => match self.next_match('=') {
                true => self.add_token(TokenType::GREATER_EQUAL),
                false => self.add_token(TokenType::GREATER),
            },
            '/' => match self.next_match('/') {
                true => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                false => self.add_token(TokenType::SLASH),
            },

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            c => () 
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
            // PANIC
            return;
        }

        // closing "
        self.advance();

        self.add_token(TokenType::STRING {
            literal: self
                .source
                .get((self.start + 1)..(self.current - 1))
                .expect("No string")
                .to_string(),
        })
    }

    fn peek(&self) -> char {
        match self.is_at_end() {
            true => '\0',
            false => self.chars_vec[self.current],
        }
    }

    fn next_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.chars_vec[self.current] != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn advance(&mut self) -> char {
        let val = self.chars_vec[self.current];
        self.current += 1;
        val
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars_vec.len()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(
            token_type,
            &self
                .source
                .get(self.start..self.current)
                .expect("Source Empty"),
            self.line,
        ))
    }
}
