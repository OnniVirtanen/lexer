use crate::lexer::token::TokenType;
use crate::lexer::token::Token;
use crate::lexer::util;

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
                lexeme: String::from("EOF"),
            };
        }

        let current_char = self.input.chars().nth(self.position).expect("Index out of bounds");

        if util::isWhitespace(current_char) {
            self.skip_whitespace();
            return self.get_next_token();
        }
        if util::isBeginningOfString(current_char) {
            return self.consume_string();
        }
        if util::isBeginningOfChar(current_char) {
            return self.consume_char();
        }
        if util::isNumber(current_char) {
            return self.consume_number();
        }
        if util::isSeparator(current_char) {
            return self.consume_separator(current_char);
        }
        if util::isOperator(current_char) {
            return self.consume_operator(current_char);
        }

        return Token {
            token_type: TokenType::Illegal,
            lexeme: String::from("Illegal"),
        };
    }

    fn skip_whitespace(&mut self) {
        self.position += 1;
    }
    
    fn consume_separator(&mut self, current_char: char) -> Token {
        self.position += 1;
        match current_char {
            ';' => Token::new(TokenType::Semicolon, String::from(current_char)),
            ',' => Token::new(TokenType::Comma, String::from(current_char)),
            '(' => Token::new(TokenType::Comma, String::from(current_char)),
            ')' => Token::new(TokenType::Comma, String::from(current_char)),
            '[' => Token::new(TokenType::Comma, String::from(current_char)),
            ']' => Token::new(TokenType::Comma, String::from(current_char)),
            '{' => Token::new(TokenType::Comma, String::from(current_char)),
            '}' => Token::new(TokenType::Comma, String::from(current_char)),
            '.' => Token::new(TokenType::Comma, String::from(current_char)),
            ':' => Token::new(TokenType::Comma, String::from(current_char)),
            _ => panic!("No such separator: {}", current_char),
        }
    }

    fn consume_operator(&mut self, current_char: char) -> Token {
        let mut operator = String::from(current_char);

        self.position += 1;
        let next_char = self.input.chars().nth(self.position).expect("Index out of bounds");
        if util::isOperator(next_char) {
            operator.push_str(&String::from(next_char));
            self.position += 1;
        }

        match operator.as_str() {
            "+" => Token::new(TokenType::Addition, operator),
            "-" => Token::new(TokenType::Subtraction, operator),
            "*" => Token::new(TokenType::Multiplication, operator),
            "/" => Token::new(TokenType::Division, operator),
            "%" => Token::new(TokenType::Modulus, operator),
            "&&" => Token::new(TokenType::LogicalAnd, operator),
            "||" => Token::new(TokenType::LogicalOr, operator),
            "!" => Token::new(TokenType::LogicalNot, operator),
            "==" => Token::new(TokenType::EqualTo, operator),
            "!=" => Token::new(TokenType::NotEqualTo, operator),
            "<" => Token::new(TokenType::LessThan, operator),
            ">" => Token::new(TokenType::GreaterThan, operator),
            "<=" => Token::new(TokenType::LessThanOrEqualTo, operator),
            ">=" => Token::new(TokenType::GreaterThanOrEqualTo, operator),
            "=" => Token::new(TokenType::Assignment, operator),
            "+=" => Token::new(TokenType::AddAndAssign, operator),
            "-=" => Token::new(TokenType::SubtractAndAssign, operator),
            "*=" => Token::new(TokenType::MultiplyAndAssign, operator),
            "/=" => Token::new(TokenType::DivideAndAssign, operator),
            "%=" => Token::new(TokenType::ModulusAndAssign, operator),
            _ => panic!("No such operator: {}", operator),
        }
    }

    fn consume_char(&mut self) -> Token {
        self.position += 1;
        let char = self.input.chars().nth(self.position).expect("Index out of bounds");
        return Token {
            token_type: TokenType::CharacterLiteral,
            lexeme: String::from(char),
        };
    }

    fn consume_string(&mut self) -> Token {
        let start_position = self.position;
        self.position += 1;
    
        let mut character = self.input.chars().nth(self.position).expect("Index out of bounds");
        let mut string = String::new();
    
        while character != '"' {
            string.push(character);
            self.position += 1;
    
            if self.position < self.input.len() {
                character = self.input.chars().nth(self.position).expect("Index out of bounds");
            } else {
                panic!("Unexpected end of input");
            }
        }
    
        self.position += 1;
    
        return Token {
            token_type: TokenType::StringLiteral,
            lexeme: string,
        }
    }

    fn consume_number(&mut self) -> Token {
        // todo
    }

    fn consume_illegal(&mut self) -> Token {
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