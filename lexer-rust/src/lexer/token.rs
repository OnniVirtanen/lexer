#[derive(PartialEq)]
#[derive(Debug)]
pub enum TokenType {
    // Identifier
    Identifier,
    // Keyword
    If,
    Else,
    While,
    Int,
    Float,
    Bool,
    Character,
    String,
    Function,
    Return,
    Void,
    // Literal
    IntegerLiteral,
    FloatingLiteral,
    BooleanLiteral,
    CharacterLiteral,
    StringLiteral,
    NullLiteral,
    // Operator
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    LogicalAnd,
    LogicalOr,
    LogicalNot,
    EqualTo,
    NotEqualTo,
    LessThan,
    GreaterThan,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
    Assignment,
    AddAndAssign,
    SubtractAndAssign,
    MultiplyAndAssign,
    DivideAndAssign,
    ModulusAndAssign,
    // Separator
    Semicolon,
    Comma,
    LeftParenthesis,
    RightParenthesis,
    LeftBracket,
    RightBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Period,
    Colon,
    // Other
    EOF,
    Illegal
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String) -> Self {
        Token { token_type, lexeme }
    }
}