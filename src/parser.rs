use crate::ast::{Comparator, Expression, LogicalExpr, LogicalOp, Operation, Program, Statement, TypeDef, VariableDef};
use crate::error::ParserError;
use crate::token::{Token, Type};

pub struct Parser{
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser{
    pub fn new(tokens: Vec<Token>) -> Self{
        Self{
            tokens,
            pos: 0,
        }
    }

    fn peek(&self) -> Option<&Token>{
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) -> Option<Token>{
        let token = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        token
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError>{
        let mut statements = Vec::new();

        while self.peek().is_some(){
            statements.push(self.parse_statement()?);
        }

        Ok(Program{
            statements,
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, ParserError> {
        match self.peek() {
            Some(Token::Let | Token::Var) => self.parse_declaration(),
            Some(Token::If) => self.parse_selection(),
            Some(Token::Identifier(name)) => self.parse_assignment(),
            _ => Err(ParserError {
                pos: self.pos,
                message: "expected declaration or selection".to_string(),
            }),
        }
    }
    fn parse_assignment(&mut self) -> Result<Statement, ParserError> {
        let name = match self.advance().unwrap(){
            Token::Identifier(identifier) => identifier,
            _ => unreachable!()
        };
        
        if self.advance().unwrap() != Token::Equal {
            return Err(ParserError{
                pos: self.pos,
                message: "expected '='".to_string(),
            });
        }

        let value = self.parse_expression()?;

        if self.advance() != Some(Token::Semicolon) {
            return Err(ParserError {
                pos: self.pos,
                message: "expected ';'".to_string(),
            });
        }
        
        Ok(Statement::Assignment {
            name,
            value,
        })
    }
    fn parse_declaration(&mut self) -> Result<Statement, ParserError> {
        let mutable = match self.advance().unwrap() {
            Token::Let => false,
            Token::Var => true,
            _ => unreachable!()
        };
        
        let ty = self.parse_typedef()?;

        let Some(Token::Identifier(name)) = self.advance() else {
            return Err(ParserError {
                pos: self.pos,
                message: "expected identifier".to_string(),
            });
        };

        if self.advance().unwrap() != Token::Equal {
            return Err(ParserError{
                pos: self.pos,
                message: "data must be initialised".to_string(),
            });
        }

        let value = self.parse_expression()?;

        if self.advance() != Some(Token::Semicolon) {
            return Err(ParserError {
                pos: self.pos,
                message: "expected ';'".to_string(),
            });
        }

        Ok(Statement::Declaration {
            definition: VariableDef{
                ty,
                mutable,
                name,
            },
            value,
        })
    }
    
    fn parse_typedef(&mut self) -> Result<TypeDef, ParserError> {
        let (reference_ty,mutable_ty,ty) = match self.advance() { 
            Some(Token::Ampersand) => {
                match self.advance() { 
                    Some(Token::Mut) => {
                        match self.advance() {
                            Some(Token::Type(ty)) => {
                                (true,true,ty)
                            }
                            _ => {
                                return Err(ParserError{
                                    pos: self.pos,
                                    message: "expected type".to_string(),
                                })
                            }
                        }
                    }
                    Some(Token::Type(ty)) => {
                        (true,false,ty)
                    }
                    _ => {
                        return Err(ParserError{
                            pos: self.pos,
                            message: "expected type".to_string(),
                        }) 
                    }
                }
            }
            Some(Token::Type(ty)) => {
                (false,false,ty)
            }
            _ => {
                return Err(ParserError{
                    pos: self.pos,
                    message: "expected type".to_string(),
                })
            }
        };
        
        Ok(TypeDef{
            ty,
            reference: reference_ty,
            mutable: mutable_ty,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<Statement>, ParserError> {
        if self.advance().unwrap() != Token::LeftBrace {
            return Err(ParserError{
                pos: self.pos,
                message: "expected opening brace '{'".to_string(),
            });
        }

        let mut statements = Vec::new();

        while let Some(token) = self.peek() &&
            *token != Token::RightBrace{
            statements.push(self.parse_statement()?)
        }

        if self.advance() != Some(Token::RightBrace) {
            return Err(ParserError {
                pos: self.pos,
                message: "expected closing brace '}'".to_string(),
            });
        }

        Ok(statements)
    }
    fn parse_selection(&mut self) -> Result<Statement, ParserError> {
        self.advance();

        let logic = self.parse_logical_expression()?;
        let statements = self.parse_block()?;
        
        let else_statements = match self.peek() {
            Some(Token::Else) => {
                self.advance();
                self.parse_block()?
            },
            _ => Vec::new(),
        };

        Ok(Statement::Selection {
            logic,
            statements,
            else_statements,
        })
    }

    fn parse_logical_expression(&mut self) -> Result<LogicalExpr, ParserError> {
        let mut left = self.parse_logical_term()?;

        while self.peek_is_logical_op() {
            let operation = self.parse_logical_operation()?;
            let right = self.parse_logical_term()?;

            left = LogicalExpr::Binary {
                left: Box::new(left),
                operation,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logical_term(&mut self) -> Result<LogicalExpr, ParserError> {
        let left = self.parse_expression()?;
        let comparator = self.parse_comparator()?;
        let right = self.parse_expression()?;

        Ok(LogicalExpr::Term {
            left,
            comparator,
            right,
        })
    }

    fn parse_logical_operation(&mut self) -> Result<LogicalOp, ParserError> {
        match self.advance() {
            Some(Token::And) => Ok(LogicalOp::And),
            Some(Token::Or) => Ok(LogicalOp::Or),
            _ => Err(ParserError{
                pos: self.pos,
                message: "expected logical operator".to_string(),
            })
        }
    }

    fn parse_comparator(&mut self) -> Result<Comparator, ParserError> {
        match self.advance() {
            Some(Token::Greater) => Ok(Comparator::Greater),
            Some(Token::Less) => Ok(Comparator::Less),
            Some(Token::EqualEqual) => Ok(Comparator::Equal),
            _ => Err(ParserError{
                pos: self.pos,
                message: "expected comparator".to_string(),
            })
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        let mut left = self.parse_value()?;

        while self.peek_is_operator() {
            let operation = self.parse_operator()?;
            let right = self.parse_value()?;

            left = Expression::Binary {
                left: Box::new(left),
                operation,
                right: Box::new(right),
            }
        }

        Ok(left)
    }

    fn parse_operator(&mut self) -> Result<Operation, ParserError> {
        match self.advance() {
            Some(Token::Plus) => Ok(Operation::Add),
            Some(Token::Minus) => Ok(Operation::Sub),
            Some(Token::Slash) => Ok(Operation::Div),
            Some(Token::Asterisk) => Ok(Operation::Mul),
            _ => Err(ParserError{
                pos: self.pos,
                message: "expected operator".to_string(),
            })
        }
    }

    fn peek_is_operator(&self) -> bool {
        match self.peek() {
            Some(Token::Plus) |
            Some(Token::Minus) |
            Some(Token::Slash) |
            Some(Token::Asterisk) => true,
            _ => false
        }
    }

    fn peek_is_logical_op(&self) -> bool {
        match self.peek() {
            Some(Token::And) |
            Some(Token::Or) => true,
            _ => false
        }
    }
    
    fn consume_token(&mut self, token: Token) -> Result<(), ParserError> {
        if self.advance() != Some(token) {
            return Err(ParserError {
                pos: self.pos,
                message: "expected: todo".to_string(),
            });
        }
        Ok(())
    }

    fn parse_value(&mut self) -> Result<Expression, ParserError> {
        match self.advance() {
            Some(Token::Identifier(name)) => Ok(Expression::Identifier(name)),
            Some(Token::Int(num)) => Ok(Expression::Number(num)),
            _ => Err(ParserError{
                pos: self.pos,
                message: "expected int or identifier".to_string(),
            })
        }
    }
}