use crate::lexer::token::TokenType;
use crate::lexer::token::Token;

pub struct Lexer {
    tokens: Vec<Token>,
    input: String,
    position: usize,
    length: usize,
}

pub trait Tokenizer {
    fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizer for Lexer {
    fn tokenize(&self) -> Vec<Token> {
        let mut vec = Vec::<Token>::new();
        let mut token = self.get_next_token();

        while token.token_type != TokenType::EOF {
            vec.push(token);
            token = self.get_next_token();
        }

        vec.push(token);
        return vec;
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