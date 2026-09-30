use crate::token::Type;

pub struct Program{
    pub statements: Vec<Statement>,
}

#[derive(PartialEq,Debug)]
pub enum Statement{
    Declaration{
        definition: VariableDef,
        value: Expression,
    },
    Assignment{
        name: String,
        value: Expression,
    },
    Selection{
        logic: LogicalExpr,
        statements: Vec<Statement>,
        else_statements: Vec<Statement>,
    },
    Function{
        name: String,
        arguments: Vec<VariableDef>,
        return_type: Option<TypeDef>,
        statements: Vec<Statement>,
    }
}
#[derive(PartialEq, Debug)]
pub struct VariableDef {
    pub ty: TypeDef,
    pub mutable: bool,
    pub name: String,
}
#[derive(PartialEq, Debug)]
pub struct TypeDef {
    pub ty: Type,
    pub reference: bool,
    pub mutable: bool,
}
#[derive(PartialEq, Debug)]
pub enum Expression{
    Number(i32),
    Identifier(String),
    Binary{
        left: Box<Expression>,
        operation: Operation,
        right: Box<Expression>,
    }
}
#[derive(PartialEq, Debug)]
pub enum Operation{
    Add,
    Sub,
    Mul,
    Div,
}
#[derive(PartialEq, Debug)]
pub enum LogicalExpr{
    Term{
        left: Expression,
        comparator: Comparator,
        right: Expression,
    },
    Binary{
        left: Box<LogicalExpr>,
        operation: LogicalOp,
        right: Box<LogicalExpr>,
    }
}
#[derive(PartialEq, Debug)]
pub enum LogicalOp{
    And,
    Or
}
#[derive(PartialEq, Debug)]
pub enum Comparator{
    Equal,
    Greater,
    Less
}
