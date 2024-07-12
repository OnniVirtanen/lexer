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

    fn get_next_token(&mut self) -> Token {
        if self.position >= self.length {
            return Token {
                token_type: TokenType::EOF,
                lexeme: String::from(""),
            };
        }

        let current_char = self.input.chars().nth(self.position).expect("Index out of bounds");

        println!("The character at index {} is: {}", self.position, current_char);
        self.position += 1;

        match current_char {
            ' ' | '\t' | '\n' | '\r' => {
                self.skip_whitespace();
            }
            _ => {
                println!("{} is not a whitespace character", current_char);
            }
        }

        return Token {
            token_type: TokenType::Illegal,
            lexeme: String::from(""),
        };
    }

    fn skip_whitespace(&self) {
        println!("skipping white space");
    }

    fn debug(&self) {
        println!("input: {}", self.input);
        println!("position: {}", self.position);
        println!("length: {}", self.length);
    }
}