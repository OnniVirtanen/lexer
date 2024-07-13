use crate::lexer::lexer::Tokenizer;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::TokenType;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_can_lex_identifier() {
        let input = String::from("house");
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].token_type, TokenType::Identifier);
    }

    #[test]
    fn test_can_lex_keyword() {
        let input = String::from("if");
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].token_type, TokenType::If);
    }

    #[test]
    fn test_can_lex_literal() {
        let input = String::from("4.0");
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].token_type, TokenType::FloatingLiteral);
    }

    #[test]
    fn test_can_lex_two_char_operator() {
        let input = String::from("*=");
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].token_type, TokenType::MultiplyAndAssign);
    }

    #[test]
    fn test_can_lex_operator() {
        let input = String::from('=');
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].token_type, TokenType::Assignment);
    }

    #[test]
    fn test_can_lex_separator() {
        let input = String::from('{');
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].token_type, TokenType::LeftCurlyBracket);
    }

    #[test]
    fn test_can_lex_other() {
        let input = String::from("");
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }

    #[test]
    fn test_can_lex_subsequent() {
        let input = String::from("; \"hello\" 4.0 5 'a' bool true null");
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens[0].token_type, TokenType::Semicolon);
        assert_eq!(tokens[1].token_type, TokenType::StringLiteral);
        assert_eq!(tokens[2].token_type, TokenType::FloatingLiteral);
        assert_eq!(tokens[3].token_type, TokenType::IntegerLiteral);
        assert_eq!(tokens[4].token_type, TokenType::CharacterLiteral);
        assert_eq!(tokens[5].token_type, TokenType::Bool);
        assert_eq!(tokens[6].token_type, TokenType::BooleanLiteral);
        assert_eq!(tokens[7].token_type, TokenType::NullLiteral);
        assert_eq!(tokens[8].token_type, TokenType::EOF);
    }

}


