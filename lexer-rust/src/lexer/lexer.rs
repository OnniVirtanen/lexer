use crate::lexer::token::TokenType;
use crate::lexer::token::Token;

use std::mem;

pub struct Lexer {
    tokens: Vec<Token>,
    input: String,
    position: usize,
    length: usize,
}

pub trait Tokenizer {
    fn tokenize(&mut self) -> Vec<Token>;
}

impl Tokenizer for Lexer {
    fn tokenize(&mut self) -> Vec<Token> {
        let mut token = self.get_next_token();

        while token.token_type != TokenType::EOF {
            self.tokens.push(token);
            token = self.get_next_token();
        }

        self.tokens.push(token);
        return mem::take(&mut self.tokens)
    }
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer {
            tokens: Vec::<Token>::new(),
            input: input.clone(),
            position: 0,
            length: input.len(),
        }
    }

    fn get_next_token(&self) -> Token {
        if self.position >= self.length {
            return Token {
                token_type: TokenType::EOF,
                lexeme: String::from(""),
            };
        }
        return Token {
            token_type: TokenType::Illegal,
            lexeme: String::from(""),
        };
    }

    fn debug(&self) {
        println!("input: {}", self.input);
        println!("position: {}", self.position);
        println!("length: {}", self.length);
    }
}