use crate::lexer::{Lexer,Token::*};

#[test]
fn lexer_test(){
    let source =
        "let x = 56;
        var y = 678;

        if x < y {

        }";

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().unwrap();

    assert_eq!(tokens,vec![
        Let,
        Identifier("x".to_string()),
        Equal,
        Int(56),
        Semicolon,
        Var,
        Identifier("y".to_string()),
        Equal,
        Int(678),
        Semicolon,
    ]);
}