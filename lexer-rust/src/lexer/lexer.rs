use crate::lexer::token::TokenType;
use crate::lexer::token::Token;

pub struct Lexer {
    tokens: Vec<Token>,
    input: String,
    position: i64,
    length: usize,
}

pub trait Tokenizer {
    fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizer for Lexer {
    fn tokenize(&self) -> Vec<Token> {
        let mut vec = Vec::<Token>::new();

        let _token = Token {
            token_type: TokenType::Identifier,
            lexeme: String::from("x"),
        };

        vec.push(_token);

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

    pub fn debug(&self) {
        println!("input: {}", self.input);
        println!("position: {}", self.position);
        println!("length: {}", self.length);
    }
}