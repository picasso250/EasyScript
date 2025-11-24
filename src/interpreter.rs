use crate::ast::{Block, Expression, LiteralValue};
use crate::environment::{Environment, EnvironmentRef};
use crate::value::Value;
use std::rc::Rc;

// Represents an error that occurs during program execution.
#[derive(Debug, Clone)]
pub struct RuntimeError(pub String);

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Runtime Error: {}", self.0)
    }
}

pub struct Interpreter {
    // The environment is now a reference-counted pointer to a mutable Environment.
    environment: EnvironmentRef,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            // Create the top-level (global) environment.
            environment: Environment::new(),
        }
    }

    /// The main entry point to run a program.
    pub fn run(&mut self, block: &Block) -> Result<Value, RuntimeError> {
        // Clone the Rc to avoid a mutable borrow conflict with self.environment
        let env_clone = Rc::clone(&self.environment);
        self.execute_block(block, &env_clone)
    }

    /// Executes a block of expressions in a given environment.
    /// For nested blocks, a new enclosed environment is created.
    fn execute_block(&mut self, block: &Block, env: &EnvironmentRef) -> Result<Value, RuntimeError> {
        // Temporarily set the interpreter's environment to the new one.
        let previous_env = Rc::clone(&self.environment);
        self.environment = Rc::clone(env);

        let mut last_value = Value::Nil;
        for expr in &block.expressions {
            last_value = self.evaluate(expr)?;
        }

        // Restore the previous environment.
        self.environment = previous_env;
        Ok(last_value)
    }

    /// The core evaluation logic that dispatches based on expression type.
    pub fn evaluate(&mut self, expression: &Expression) -> Result<Value, RuntimeError> {
        match expression {
            Expression::Literal(val) => self.evaluate_literal(val),

            Expression::ListLiteral(expr_list) => {
                let mut values = Vec::new();
                for expr in expr_list {
                    values.push(self.evaluate(expr)?);
                }
                Ok(Value::List(Rc::new(values)))
            }

            Expression::MapLiteral(expr_pairs) => {
                let mut map = std::collections::HashMap::new();
                for (key_expr, value_expr) in expr_pairs {
                    let key = self.evaluate(key_expr)?;
                    let value = self.evaluate(value_expr)?;
                    // For now, assume keys are simple types that implement Hash and Eq.
                    // Value::List and Value::Map are not hashable by default, so inserting them here
                    // would cause a panic or incorrect behavior if they were allowed as keys.
                    map.insert(key, value);
                }
                Ok(Value::Map(Rc::new(map)))
            }

            Expression::Block(block) => {
                // Create a new scope for the block and execute it.
                let new_env = Environment::new_enclosed(&self.environment);
                self.execute_block(block, &new_env)
            }

                        Expression::Identifier(name) => self.environment.borrow().get(name).map_err(|e| RuntimeError(e)),

            

                        Expression::Assignment { lvalue, value } => {

                            let value_to_assign = self.evaluate(value)?;

                            

                            match lvalue {

                                crate::ast::LValue::Identifier(name) => {

                                    self.environment

                                        .borrow_mut()

                                        .assign(name, value_to_assign.clone());

                                    Ok(value_to_assign)

                                }

                                crate::ast::LValue::IndexAccess { target, key } => {
                                    let value_to_assign = self.evaluate(value)?;
                                    let key_val = self.evaluate(key)?;

                                    match &**target {
                                        Expression::Identifier(ref target_name) => {
                                            if let Some(target_env_ref) = Environment::find_environment(&self.environment, target_name) {
                                                let mut target_env = target_env_ref.borrow_mut();
                                                
                                                if let Some(mut existing_val) = target_env.values.remove(target_name.as_str()) {
                                                    let mut modification_err = None;
                                                    match &mut existing_val {
                                                        Value::List(list_rc) => {
                                                            let mut_list = Rc::make_mut(list_rc);
                                                            if let Value::Number(idx_float) = key_val {
                                                                let index = idx_float as usize;
                                                                if index < mut_list.len() {
                                                                    mut_list[index] = value_to_assign.clone();
                                                                } else {
                                                                    modification_err = Some(RuntimeError(format!("List index out of bounds for assignment: {}", idx_float)));
                                                                }
                                                            } else {
                                                                modification_err = Some(RuntimeError(format!("List index must be a number for assignment. Got: {}", key_val)));
                                                            }
                                                        }
                                                        Value::Map(map_rc) => {
                                                            let mut_map = Rc::make_mut(map_rc);
                                                            mut_map.insert(key_val, value_to_assign.clone());
                                                        }
                                                        _ => {
                                                            modification_err = Some(RuntimeError(format!("Cannot index non-list/map variable '{}'", target_name)));
                                                        }
                                                    }
                                                    
                                                    target_env.values.insert(target_name.clone(), existing_val);
                                                    
                                                    if let Some(err) = modification_err {
                                                        return Err(err);
                                                    }
                                                    Ok(value_to_assign)
                                                } else {
                                                    Err(RuntimeError(format!("Internal error: Variable '{}' found but could not be removed for mutation.", target_name)))
                                                }
                                            } else {
                                                Err(RuntimeError(format!("Undefined variable '{}' in index assignment.", target_name)))
                                            }
                                        }
                                        _ => Err(RuntimeError("Nested accessor assignment (e.g., obj.prop[idx]) not yet supported.".to_string())),
                                    }
                                }
                                crate::ast::LValue::DotAccess { target, property_name } => {
                                    let value_to_assign = self.evaluate(value)?;
                                    match &**target {
                                        Expression::Identifier(ref target_name) => {
                                            if let Some(target_env_ref) = Environment::find_environment(&self.environment, target_name) {
                                                let mut target_env = target_env_ref.borrow_mut();
                                                
                                                if let Some(mut existing_val) = target_env.values.remove(target_name.as_str()) {
                                                    let mut modification_err = None;
                                                    match &mut existing_val {
                                                        Value::Map(map_rc) => {
                                                            let mut_map = Rc::make_mut(map_rc);
                                                            mut_map.insert(Value::String(property_name.clone()), value_to_assign.clone());
                                                        }
                                                        _ => {
                                                            modification_err = Some(RuntimeError(format!("Cannot use dot access on non-map variable '{}'", target_name)));
                                                        }
                                                    }

                                                    target_env.values.insert(target_name.clone(), existing_val);

                                                    if let Some(err) = modification_err {
                                                        return Err(err);
                                                    }
                                                    Ok(value_to_assign)
                                                } else {
                                                    Err(RuntimeError(format!("Internal error: Variable '{}' found but could not be removed for mutation.", target_name)))
                                                }
                                            } else {
                                                Err(RuntimeError(format!("Undefined variable '{}' in dot assignment.", target_name)))
                                            }
                                        }
                                        _ => Err(RuntimeError("Nested accessor assignment (e.g., obj[idx].prop) not yet supported.".to_string())),
                                    }
                                }

                            }

                        }

            

                        Expression::Accessor { target, access } => {

                            let target_val = self.evaluate(target)?;

            

                            match access {

                                crate::ast::AccessType::Index(key_expr) => {

                                    let key_val = self.evaluate(key_expr)?;

                                    match target_val {

                                        Value::List(list) => {

                                            if let Value::Number(idx_float) = key_val {

                                                let index = idx_float as usize; // Cast to usize for list indexing

                                                if let Some(val) = list.get(index) {

                                                    Ok(val.clone())

                                                } else {

                                                    Err(RuntimeError(format!("List index out of bounds: {}", idx_float)))

                                                }

                                            } else {

                                                Err(RuntimeError(format!("List index must be a number. Got: {}", key_val)))

                                            }

                                        }

                                        Value::Map(map) => {

                                            if let Some(val) = map.get(&key_val) { // Uses Value's PartialEq for key lookup

                                                Ok(val.clone())

                                            } else {

                                                Err(RuntimeError(format!("Key not found in map: {}", key_val)))

                                            }

                                        }

                                        _ => Err(RuntimeError(format!("Cannot index non-list/map type: {}", target_val)))

                                    }

                                }

                                crate::ast::AccessType::Dot(property_name) => {

                                    match target_val {

                                        Value::Map(map) => {

                                            // Dot access uses a string literal as a key

                                            let key_val = Value::String(property_name.clone());

                                            if let Some(val) = map.get(&key_val) {

                                                Ok(val.clone())

                                            } else {

                                                Err(RuntimeError(format!("Property '{}' not found in map.", property_name)))

                                            }

                                        }

                                        _ => Err(RuntimeError(format!("Cannot use dot access on non-map type: {}", target_val)))

                                    }

                                }

                            }

                        }

            

                        Expression::If { condition, then_block, else_branch } => {

                            let condition_val = self.evaluate(condition)?;

                let is_truthy = match condition_val {
                    Value::Boolean(b) => b,
                    Value::Nil => false,
                    _ => true, // All other values are truthy
                };

                if is_truthy {
                    let env_clone = Rc::clone(&self.environment); // Fix: Clone Rc to avoid borrow conflict
                    self.execute_block(then_block, &env_clone)
                } else if let Some(else_expr) = else_branch {
                    // The else_branch can be another IfExpression or a BlockExpression
                    self.evaluate(else_expr) // Evaluate the else expression (which could be a block or another if)
                } else {
                    Ok(Value::Nil) // No else branch, condition false, so return nil
                }
            }

            Expression::Unary { op, expr } => {
                let right_val = self.evaluate(expr)?;
                match op {
                    crate::ast::UnaryOperator::Negate => {
                        if let Value::Number(num) = right_val {
                            Ok(Value::Number(-num))
                        } else {
                            Err(RuntimeError(format!(
                                "Unary '-' operator can only be applied to numbers. Got: {}",
                                right_val.to_string()
                            )))
                        }
                    }
                }
            }

            Expression::Binary { left, op, right } => {
                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;
                use crate::ast::BinaryOperator;

                match op {
                    BinaryOperator::Eq => return Ok(Value::Boolean(left_val == right_val)),
                    BinaryOperator::Neq => return Ok(Value::Boolean(left_val != right_val)),
                    _ => {}
                }

                match (left_val, right_val) {
                    (Value::Number(l), Value::Number(r)) => match op {
                        BinaryOperator::Add => Ok(Value::Number(l + r)),
                        BinaryOperator::Sub => Ok(Value::Number(l - r)),
                        BinaryOperator::Mul => Ok(Value::Number(l * r)),
                        BinaryOperator::Div => {
                            if r == 0.0 {
                                Err(RuntimeError("Division by zero.".to_string()))
                            } else {
                                Ok(Value::Number(l / r))
                            }
                        }
                        BinaryOperator::Lt => Ok(Value::Boolean(l < r)),
                        BinaryOperator::Lte => Ok(Value::Boolean(l <= r)),
                        BinaryOperator::Gt => Ok(Value::Boolean(l > r)),
                        BinaryOperator::Gte => Ok(Value::Boolean(l >= r)),
                        _ => Err(RuntimeError(format!(
                            "Unsupported operator '{:?}' for numbers.",
                            op
                        ))),
                    },
                    (Value::String(l), Value::String(r)) => match op {
                        BinaryOperator::Add => Ok(Value::String(format!("{}{}", l, r))),
                        _ => Err(RuntimeError(format!(
                            "Unsupported operator '{:?}' for strings.",
                            op
                        ))),
                    },
                    (l, r) => Err(RuntimeError(format!(
                        "Cannot apply operator '{:?}' to unsupported types: {} and {}",
                        op,
                        l.to_string(),
                        r.to_string()
                    ))),
                }
            }

            _ => Err(RuntimeError(format!(
                "This expression type is not yet supported: {:?}",
                expression
            ))),
        }
    }

    /// Evaluates a literal value from the AST into a runtime Value.
    fn evaluate_literal(&self, literal: &LiteralValue) -> Result<Value, RuntimeError> {
        Ok(match literal {
            LiteralValue::Number(n) => Value::Number(*n),
            LiteralValue::String(s) => Value::String(s.clone()),
            LiteralValue::Boolean(b) => Value::Boolean(*b),
            LiteralValue::Nil => Value::Nil,
        })
    }
}
