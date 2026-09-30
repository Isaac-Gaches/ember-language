#[derive(Debug, PartialEq)]
pub enum Token{
    Let,
    Var,
    If,
    Else,

    Identifier(String),
    Int(i32),

    Plus,
    Minus,
    Asterisk,
    Slash,
    Equal,

    EqualEqual,
    Greater,
    Less,
    And,
    Or,

    LeftBrace,
    RightBrace,
    Semicolon,
}





