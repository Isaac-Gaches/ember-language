use crate::ast::{Comparator, Expression, LogicalExpr, LogicalOp, Operation, Statement};
use crate::parser::Parser;

#[test]
fn lexer_test(){
    use crate::lexer::Lexer;
    use crate::token::Token::*;

    let source =
        "let x = 56;
        var y = x + 678;

        if x < y + 10 or 5 == 4{

        }".to_string();

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize().ok().unwrap();

    assert_eq!(tokens,vec![
        Let,
        Identifier("x".to_string()),
        Equal,
        Int(56),
        Semicolon,
        Var,
        Identifier("y".to_string()),
        Equal,
        Identifier("x".to_string()),
        Plus,
        Int(678),
        Semicolon,
        If,
        Identifier("x".to_string()),
        Less,
        Identifier("y".to_string()),
        Plus,
        Int(10),
        Or,
        Int(5),
        EqualEqual,
        Int(4),
        LeftBrace,
        RightBrace,
    ]);
}

#[test]
fn parser_test(){
    use crate::parser::Parser;
    use crate::token::Token::*;

    let tokens= vec![
        Let,
        Identifier("x".to_string()),
        Equal,
        Int(56),
        Semicolon,
        Var,
        Identifier("y".to_string()),
        Equal,
        Identifier("x".to_string()),
        Plus,
        Int(678),
        Semicolon,
        If,
        Identifier("x".to_string()),
        Less,
        Identifier("y".to_string()),
        Plus,
        Int(10),
        Or,
        Int(5),
        EqualEqual,
        Int(4),
        LeftBrace,
        RightBrace,
    ];

    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program().ok().unwrap();


  /*  assert_eq!(ast.statements,vec![
        Statement::Declaration{
            name: "x".to_string(),
            declaration_type: DeclarationType::Let,
            value: Expression::Number(56),
        },
        Statement::Declaration{
            name: "y".to_string(),
            declaration_type: DeclarationType::Var,
            value: Expression::Binary {
                left: Box::new(Expression::Identifier("x".to_string())),
                operation: Operation::Add,
                right: Box::new(Expression::Number(678)),
            },
        },
        Statement::Selection {
            logic: LogicalExpr::Binary {
                left: Box::new(LogicalExpr::Term {
                    left: Expression::Identifier("x".to_string()),
                    comparator: Comparator::Less,
                    right: Expression::Binary {
                        left: Box::new(Expression::Identifier("y".to_string())),
                        operation: Operation::Add,
                        right: Box::new(Expression::Number(10)),
                    },
                }),
                operation: LogicalOp::Or,
                right: Box::new(LogicalExpr::Term {
                    left: Expression::Number(5),
                    comparator: Comparator::Equal,
                    right: Expression::Number(4),
                }),
            },
            statements: vec![],
            else_statements: vec![],
        }
    ]);*/
}

#[test]
fn interpreter_test(){

}