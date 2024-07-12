use crate::lexer::lexer::Tokenizer;
use crate::lexer::lexer::Lexer;
mod lexer;

fn main() {
    let input = String::from("x = 1;");
    let lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    for token in tokens {
        println!("token value: {}", token.lexeme);
    }
}