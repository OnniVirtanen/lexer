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
            return Token::new(TokenType::EOF, String::from("EOF"));
        }

        let current_char = self.input.chars().nth(self.position).expect("Index out of bounds");

        if util::is_whitespace(current_char) {
            self.skip_whitespace();
            return self.get_next_token();
        }
        if util::is_beginning_of_string(current_char) {
            return self.consume_string();
        }
        if util::is_beginning_of_char(current_char) {
            return self.consume_char();
        }
        if util::is_number(current_char) {
            return self.consume_number();
        }
        if util::is_letter(current_char) {
            return self.consume_letter();
        }
        if util::is_separator(current_char) {
            return self.consume_separator();
        }
        if util::is_operator(current_char) {
            return self.consume_operator();
        }
        
        return Token::new(TokenType::Illegal, String::from("Illegal"));
    }

    fn skip_whitespace(&mut self) {
        self.position += 1;
    }
    
    fn consume_separator(&mut self) -> Token {
        let current_char = self.input.chars().nth(self.position).expect("Index out of bounds");
        self.position += 1;

        let type_of_token = match current_char {
            ';' => TokenType::Semicolon,
            ',' => TokenType::Comma,
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '{' => TokenType::LeftCurlyBracket,
            '}' => TokenType::RightCurlyBracket,
            '.' => TokenType::Period,
            ':' => TokenType::Colon,
            _ => panic!("No such separator: {}", current_char),
        };

        return Token::new(type_of_token, String::from(current_char));
    }

    fn consume_operator(&mut self) -> Token {
        let current_char = self.input.chars().nth(self.position).expect("Index out of bounds");
        let mut operator = String::from(current_char);

        self.position += 1;
        let next_char = self.input.chars().nth(self.position).expect("Index out of bounds");
        if util::is_operator(next_char) {
            operator.push_str(&String::from(next_char));
            self.position += 1;
        }

        let type_of_token = match operator.as_str() {
            "+" => TokenType::Addition,
            "-" => TokenType::Subtraction,
            "*" => TokenType::Multiplication,
            "/" => TokenType::Division,
            "%" => TokenType::Modulus,
            "&&" => TokenType::LogicalAnd,
            "||" => TokenType::LogicalOr,
            "!" => TokenType::LogicalNot,
            "==" => TokenType::EqualTo,
            "!=" => TokenType::NotEqualTo,
            "<" => TokenType::LessThan,
            ">" => TokenType::GreaterThan,
            "<=" => TokenType::LessThanOrEqualTo,
            ">=" => TokenType::GreaterThanOrEqualTo,
            "=" => TokenType::Assignment,
            "+=" => TokenType::AddAndAssign,
            "-=" => TokenType::SubtractAndAssign,
            "*=" => TokenType::MultiplyAndAssign,
            "/=" => TokenType::DivideAndAssign,
            "%=" => TokenType::ModulusAndAssign,
            _ => panic!("No such operator: {}", operator),
        };

        return Token::new(type_of_token, operator);
    }

    fn consume_char(&mut self) -> Token {
        self.position += 1;
        let char = self.input.chars().nth(self.position).expect("Index out of bounds");
        return Token::new(TokenType::CharacterLiteral, String::from(char));
    }

    fn consume_string(&mut self) -> Token {
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
        return Token::new(TokenType::StringLiteral, string);
    }

    fn consume_number(&mut self) -> Token {
        let mut is_float = false;

        let mut character = self.input.chars().nth(self.position).expect("Index out of bounds");
        let mut string = String::new();

        while util::is_number(character) || character == '.' {
            string.push(character);
            if character == '.' {
                is_float = true;
            }
            self.position += 1;

            if self.position < self.input.len() {
                character = self.input.chars().nth(self.position).expect("Index out of bounds");
            } else {
                panic!("Unexpected end of input");
            }
        }

        let type_of_token = if is_float { TokenType::FloatingLiteral } else { TokenType::IntegerLiteral };

        return Token::new(type_of_token, string);
    }

    fn consume_letter(&mut self) -> Token {
        let mut character = self.input.chars().nth(self.position).expect("Index out of bounds");
        let mut string = String::new();
    
        while !util::is_whitespace(character) && !util::is_separator(character) && (self.position < self.length) {
            string.push(character);
            self.position += 1;
    
            if self.position < self.input.len() {
                character = self.input.chars().nth(self.position).expect("Index out of bounds");
            } else {
                panic!("Unexpected end of input");
            }
        }

        if util::is_keyword(string.clone()) {
            return self.consume_keyword(string);
        }
        if string == "true" || string == "false" {
            return Token::new(TokenType::BooleanLiteral, string);
        }
        if string == "null" {
            return Token::new(TokenType::NullLiteral, string);
        }

        return Token::new(TokenType::Identifier, string);
    }

    fn consume_keyword(&mut self, keyword: String) -> Token {
        let type_of_token = match keyword.as_str() {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "while" => TokenType::While,
            "int" => TokenType::Int,
            "float" => TokenType::Float,
            "bool" => TokenType::Bool,
            "char" => TokenType::Character,
            "string" => TokenType::String,
            "fun" => TokenType::Function,
            "return" => TokenType::Return,
            "void" => TokenType::Void,
            _ => panic!("No such keyword: {}", keyword),
        };

        return Token::new(type_of_token, keyword);
    }

}