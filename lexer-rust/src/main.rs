use crate::lexer::lexer::Tokenizer;
use crate::lexer::lexer::Lexer;
mod lexer;

fn main() {
    let input = String::from("; += - \"hello\" 4.0 5 65 0.1  ,");
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    for token in tokens {
        println!("token value: {}", token.lexeme);
    }
}