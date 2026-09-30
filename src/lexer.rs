use crate::error::LexerError;
use crate::token::{Token, Type};

pub struct Lexer{
    source: String,
    pos: usize,
}

impl Lexer{
    pub fn new(source: String) -> Self {
        Self{
            source,
            pos: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>,LexerError>{
        let mut tokens = Vec::new();

        let mut chars = self.source.chars().peekable();

        while let Some(c) = chars.next(){
            let token = if c.is_whitespace() {
                self.pos += 1;
                continue;
            }
            else if c.is_alphabetic(){
                let mut word = String::from(c);
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_'{
                        word.push(next);
                        chars.next();
                        self.pos += 1;
                    }
                    else{
                        break;
                    }
                }

                match word.as_str() {
                    "let" => Token::Let,
                    "var" => Token::Var,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "or" => Token::Or,
                    "and" => Token::And,
                    "mut" => Token::Mut,
                    "int" => Token::Type(Type::Int),
                    "float" => Token::Type(Type::Float),
                    "bool" => Token::Type(Type::Bool),
                    "string" => Token::Type(Type::String),
                    "char" => Token::Type(Type::Char),
                    "null" => Token::Type(Type::Null),
                    "fn" => Token::Fn,
                    _ => Token::Identifier(word),
                }
            }
            else if c.is_digit(10){
                let mut number = String::from(c);
                while let Some(&next) = chars.peek() {
                    if next.is_digit(10){
                        number.push(next);
                        chars.next();
                        self.pos += 1;
                    }
                    else{
                        break;
                    }
                }

                Token::Int(number.parse::<i32>().unwrap())
            }
            else{
                match c {
                    ';' => Token::Semicolon,
                    '{' => Token::LeftBrace,
                    '}' => Token::RightBrace,

                    '+' => Token::Plus,
                    '-' => {
                        if let Some(&next) = chars.peek() && next == '>' {
                            chars.next();
                            Token::Arrow
                        }
                        else{
                            Token::Minus
                        }
                    }
                    '*' => Token::Asterisk,
                    '/' => Token::Slash,
                    
                    '&' => Token::Ampersand,

                    '>' => Token::Greater,
                    '<' => Token::Less,
                    '=' => {
                        if let Some(&next) = chars.peek() && next == '=' {
                            chars.next();
                            Token::EqualEqual
                        }
                        else{
                            Token::Equal
                        }
                    }
                    _ => {
                        return Err(LexerError{
                            pos: self.pos,
                        })
                    }
                }
            };

            tokens.push(token);
        }

        Ok(tokens)
    }
}