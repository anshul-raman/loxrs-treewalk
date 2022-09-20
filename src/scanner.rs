use crate::token::Token;

pub struct Scanner {
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {




        Scanner { tokens: vec![] }
    }

    

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}
