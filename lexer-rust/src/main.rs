mod lexer {

    use core::fmt;

    pub struct Lexer {
        tokens: Vec<Token>,
        input: String,
        position: i64,
        length: usize,
    }

    pub trait Tokenizer {
        fn tokenize(&self) -> Vec<Token>;
    }

    pub enum TokenType {
        Identifier,
        Keyword,
        Literal,
        Operator,
        Separator,    
    }

    impl fmt::Display for TokenType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                TokenType::Identifier => write!(f, "identifier"),
                TokenType::Keyword => write!(f, "keyword"),
                TokenType::Literal => write!(f, "literal"),
                TokenType::Operator => write!(f, "operator"),
                TokenType::Separator => write!(f, "separator"),
            }
        }
    }
    
    pub struct Token {
        pub token_type: TokenType,
        pub lexeme: String,
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
    
}

use crate::lexer::Tokenizer;

fn main() {
    let input = String::from("x = 1;");
    let lexer = lexer::Lexer::new(input);
    lexer.debug();
    let tokens = lexer.tokenize();
    for token in tokens {
        println!("token type: {}", token.token_type);
        println!("token value: {}", token.lexeme);
    }
}