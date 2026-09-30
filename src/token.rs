#[derive(Debug, PartialEq,Clone)]
pub enum Token{
    Let,
    Var,
    If,
    Else,
    Fn,
    
    Arrow,
    Mut,
    Ampersand,
    Type(Type),

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

#[derive(Debug, PartialEq,Clone)]
pub enum Type{
    Int,
    Float,
    Bool,
    String,
    Char,
    Null,
}





