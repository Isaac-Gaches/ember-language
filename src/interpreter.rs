use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use crate::ast::{Comparator, Expression, LogicalExpr, LogicalOp, Operation, Program, Statement};
use crate::error::RuntimeError;

pub struct Interpreter {
    variables: HashMap<String, Variable>,
}
#[derive(Debug)]
struct Variable {
    value: Value,
    mutable: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Self{
            variables: HashMap::new(),
        }
    }

    pub fn run(&mut self,program: Program) -> Result<(), RuntimeError> {
        for stmt in &program.statements {
            self.execute(stmt)?;
        }

        for var in &self.variables {
            println!("{}: {:?}", var.0,var.1);
        }

        Ok(())
    }

    fn execute(&mut self, stmt: &Statement) -> Result<(), RuntimeError> {
        match stmt {
            Statement::Declaration { definition, value } => {
                let mutable = definition.mutable;
                let value = self.evaluate_expr(value)?;

                self.variables.insert(definition.name.clone(), Variable{
                    value,
                    mutable
                });
            },
            Statement::Selection { logic, statements,else_statements } => {
                if self.evaluate_logical_expr(logic)? {
                    for stmt in statements {
                        self.execute(stmt)?;
                    }
                }
                else{
                    for stmt in else_statements {
                        self.execute(stmt)?;
                    }
                }
            }
            Statement::Assignment { name, value } => {
                let value = self.evaluate_expr(value)?;

                match self.variables.get_mut(name) {
                    Some(var) => {
                        if !var.mutable{
                            return Err(RuntimeError{
                                message: "variable is not mutable".to_string(),
                            });
                        }
                        var.value = value;
                    }
                    None => {
                        return Err(RuntimeError{
                            message: "unknown identifier".to_string(),
                        });
                    }
                }
            }
            Statement::Function {..} => {

            }
        }
        Ok(())
    }

    fn evaluate_logical_expr(&mut self, expr: &LogicalExpr) -> Result<bool, RuntimeError> {
        match expr {
            LogicalExpr::Term {left,comparator,right} =>{
                let left = self.evaluate_expr(left)?;
                let right = self.evaluate_expr(right)?;
                match comparator {
                    Comparator::Equal => Ok(left == right),
                    Comparator::Greater => Ok(left > right),
                    Comparator::Less => Ok(left < right),
                }
            }
            LogicalExpr::Binary {left, operation,right} =>{
                let left = self.evaluate_logical_expr(left)?;
                let right = self.evaluate_logical_expr(right)?;
                match operation {
                    LogicalOp::And => Ok(left && right),
                    LogicalOp::Or => Ok(left || right),
                }
            }
        }
    }

    fn evaluate_expr(&mut self, expr: &Expression) -> Result<Value, RuntimeError> {
        match expr {
            Expression::Number(number) => Ok(Value::Int(*number)),
            Expression::Identifier(identifier) => {
                if let Some(variable) = self.variables.get(identifier) {
                    Ok(variable.value)
                }
                else{
                    Err(RuntimeError{
                        message: "unknown identifier".to_string(),
                    })
                }
            }
            Expression::Binary {left,operation,right } =>{
                let left = self.evaluate_expr(left)?;
                let right = self.evaluate_expr(right)?;
                match operation {
                    Operation::Mul => left * right,
                    Operation::Div => left / right,
                    Operation::Add => left + right,
                    Operation::Sub => left - right,
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Value{
    Int(i32)
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Value::Int(num1) => {
                match other {
                    Value::Int(num2) => {
                        num1 == num2
                    }
                }
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Value::Int(num1) => {
                match other {
                    Value::Int(num2) => {
                        Some(num1.cmp(num2))
                    }
                }
            }
        }
    }
}

impl Mul for Value {
    type Output = Result<Value, RuntimeError>;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(num1) => {
                match rhs {
                    Value::Int(num2) => {
                        Ok(Value::Int(num1 * num2))
                    }
                }
            }
        }
    }
}

impl Div for Value {
    type Output = Result<Value, RuntimeError>;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(num1) => {
                match rhs {
                    Value::Int(num2) => {
                        Ok(Value::Int(num1 / num2))
                    }
                }
            }
        }
    }
}

impl Add for Value {
    type Output = Result<Value, RuntimeError>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(num1) => {
                match rhs {
                    Value::Int(num2) => {
                        Ok(Value::Int(num1 + num2))
                    }
                }
            }
        }
    }
}
impl Sub for Value {
    type Output = Result<Value, RuntimeError>;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Value::Int(num1) => {
                match rhs {
                    Value::Int(num2) => {
                        Ok(Value::Int(num1 + num2))
                    }
                }
            }
        }
    }
}
