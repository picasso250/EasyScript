use crate::ast::{Block, Expression, LiteralValue};
use crate::environment::Environment;
use crate::value::Value;

// Represents an error that occurs during program execution.
#[derive(Debug, Clone)]
pub struct RuntimeError(pub String);

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime Error: {}", self.0)
    }
}

pub struct Interpreter {
    // The environment for storing global variables.
    // Scoped environments will be added later.
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }

    // The main entry point to run a program (a block of expressions).
    pub fn run(&mut self, block: &Block) -> Result<Value, RuntimeError> {
        let mut last_value = Value::Nil;
        for expr in &block.expressions {
            last_value = self.evaluate(expr)?;
        }
        Ok(last_value)
    }

    // The core evaluation logic that dispatches based on expression type.
    pub fn evaluate(&mut self, expression: &Expression) -> Result<Value, RuntimeError> {
        match expression {
            Expression::Literal(val) => self.evaluate_literal(val),

            Expression::Identifier(name) => self
                .environment
                .get(name)
                .map_err(|e| RuntimeError(e)),

            Expression::Assignment { lvalue, value } => {
                let value_to_assign = self.evaluate(value)?;
                
                // For now, only handle simple identifier assignment.
                match &lvalue {
                    crate::ast::LValue::Identifier(name) => {
                        self.environment.assign(name, value_to_assign.clone());
                        // Assignment evaluates to the assigned value.
                        Ok(value_to_assign)
                    }
                    _ => Err(RuntimeError(
                        "Complex assignments are not yet supported.".to_string(),
                    )),
                }
            }

            // Other expression types like Binary, etc., will be added here.
            _ => Err(RuntimeError(
                "This expression type is not yet supported.".to_string(),
            )),
        }
    }
    
    // Evaluates a literal value from the AST into a runtime Value.
    fn evaluate_literal(&self, literal: &LiteralValue) -> Result<Value, RuntimeError> {
        Ok(match literal {
            LiteralValue::Number(n) => Value::Number(*n),
            LiteralValue::String(s) => Value::String(s.clone()),
            LiteralValue::Boolean(b) => Value::Boolean(*b),
            LiteralValue::Nil => Value::Nil,
            // Lists and Maps are literals but will be implemented later.
            _ => {
                return Err(RuntimeError(
                    "This literal type is not yet supported.".to_string(),
                ))
            }
        })
    }
}
