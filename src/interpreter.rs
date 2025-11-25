use crate::ast::{Block, Expression, LiteralValue};
use crate::environment::{Environment, EnvironmentRef};
use crate::error::EasyScriptError;
use crate::native;
use crate::value::Value;
use std::rc::Rc;

pub struct Interpreter {
    // The environment is now a reference-counted pointer to a mutable Environment.
    environment: EnvironmentRef,
}

impl Interpreter {
    pub fn new() -> Self {
        let interpreter = Interpreter {
            // Create the top-level (global) environment.
            environment: Environment::new(),
        };

        // Register native functions
        {
            let mut env = interpreter.environment.borrow_mut();
            env.assign(
                "print",
                Value::Function(crate::value::FunctionObject::Native(Rc::new(
                    native::print_fn,
                ))),
            );
            env.assign(
                "len",
                Value::Function(crate::value::FunctionObject::Native(Rc::new(
                    native::len_fn,
                ))),
            );
            env.assign(
                "type",
                Value::Function(crate::value::FunctionObject::Native(Rc::new(
                    native::type_fn,
                ))),
            );
            env.assign(
                "string",
                Value::Function(crate::value::FunctionObject::Native(Rc::new(
                    native::string_fn,
                ))),
            );
            env.assign(
                "number",
                Value::Function(crate::value::FunctionObject::Native(Rc::new(
                    native::number_fn,
                ))),
            );
            env.assign(
                "input",
                Value::Function(crate::value::FunctionObject::Native(Rc::new(
                    native::input_fn,
                ))),
            );
            env.assign(
                "bool",
                Value::Function(crate::value::FunctionObject::Native(Rc::new(
                    native::bool_fn,
                ))),
            );
            env.assign(
                "keys",
                Value::Function(crate::value::FunctionObject::Native(Rc::new(
                    native::keys_fn,
                ))),
            );
        } // env 借用在此处结束

        interpreter
    }

    /// The main entry point to run a program.
    pub fn run(&mut self, block: &Block) -> Result<Value, EasyScriptError> {
        // Clone the Rc to avoid a mutable borrow conflict with self.environment
        let env_clone = Rc::clone(&self.environment);
        self.execute_block(block, &env_clone)
    }

    /// Executes a block of expressions in a given environment.
    /// For nested blocks, a new enclosed environment is created.
    fn execute_block(
        &mut self,
        block: &Block,
        env: &EnvironmentRef,
    ) -> Result<Value, EasyScriptError> {
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
    pub fn evaluate(&mut self, expression: &Expression) -> Result<Value, EasyScriptError> {
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

            Expression::Identifier(name) => {
                self.environment
                    .borrow()
                    .get(name)
                    .map_err(|e| EasyScriptError::RuntimeError {
                        message: e,
                        location: None,
                    })
            }

            Expression::FunctionDef { params, body } => {
                Ok(Value::Function(crate::value::FunctionObject::User {
                    params: params.clone(),
                    body: std::rc::Rc::new(body.clone()),
                    defined_env: Rc::clone(&self.environment), // 捕获当前环境
                }))
            }

            // 新增: Let 表达式的处理
            Expression::Let { identifier, value } => {
                let assigned_value = self.evaluate(value)?;
                self.environment // Assigns in the current environment, allowing shadowing
                    .borrow_mut()
                    .assign(identifier, assigned_value.clone());
                Ok(assigned_value) // let 表达式返回被赋的值
            }

            Expression::Assignment { lvalue, value } => {
                let value_to_assign = self.evaluate(value)?;

                match lvalue {
                    crate::ast::LValue::Identifier(name) => {
                        // Find the environment where the variable is defined.
                        // If not found, define it in the current environment.
                        if let Some(target_env_ref) =
                            Environment::find_environment(&self.environment, name)
                        {
                            target_env_ref
                                .borrow_mut()
                                .assign(name, value_to_assign.clone());
                        } else {
                            // If not found, it's an error: variables must be declared with 'let' first.
                            return Err(EasyScriptError::RuntimeError {
                                message: format!("Cannot assign to undeclared variable '{}'. Use 'let' to declare it.", name),
                                location: None,
                            });
                        }

                        Ok(value_to_assign)
                    }

                    crate::ast::LValue::IndexAccess { target, key } => {
                        let value_to_assign = self.evaluate(value)?;
                        let key_val = self.evaluate(key)?;

                        match &**target {
                            Expression::Identifier(ref target_name) => {
                                if let Some(target_env_ref) =
                                    Environment::find_environment(&self.environment, target_name)
                                {
                                    let mut target_env = target_env_ref.borrow_mut();

                                    if let Some(mut existing_val) =
                                        target_env.values.remove(target_name.as_str())
                                    {
                                        let mut modification_err = None;
                                        match &mut existing_val {
                                            Value::List(list_rc) => {
                                                let mut_list = Rc::make_mut(list_rc);
                                                if let Value::Number(idx_float) = key_val {
                                                    let index = idx_float as usize;
                                                    if index < mut_list.len() {
                                                        mut_list[index] = value_to_assign.clone();
                                                    } else {
                                                        modification_err = Some(EasyScriptError::RuntimeError {
                                                            message: format!("List index out of bounds for assignment: {}", idx_float),
                                                            location: None,
                                                        });
                                                    }
                                                } else {
                                                    modification_err = Some(EasyScriptError::RuntimeError {
                                                        message: format!("List index must be a number for assignment. Got: {}", key_val),
                                                        location: None,
                                                    });
                                                }
                                            }
                                            Value::Map(map_rc) => {
                                                let mut_map = Rc::make_mut(map_rc);
                                                mut_map.insert(key_val, value_to_assign.clone());
                                            }
                                            _ => {
                                                modification_err = Some(EasyScriptError::RuntimeError {
                                                    message: format!("Cannot index non-list/map variable '{}'", target_name),
                                                    location: None,
                                                });
                                            }
                                        }

                                        target_env
                                            .values
                                            .insert(target_name.clone(), existing_val);

                                        if let Some(err) = modification_err {
                                            return Err(err);
                                        }
                                        Ok(value_to_assign)
                                    } else {
                                        Err(EasyScriptError::RuntimeError {
                                        message: format!("Internal error: Variable '{}' found but could not be removed for mutation.", target_name),
                                        location: None,
                                    })
                                    }
                                } else {
                                    Err(EasyScriptError::RuntimeError {
                                        message: format!("Undefined variable '{}' in index assignment.", target_name),
                                        location: None,
                                    })
                                }
                            }
                            _ => Err(EasyScriptError::RuntimeError {
                                message: "Nested accessor assignment (e.g., obj.prop[idx]) not yet supported.".to_string(),
                                location: None,
                            }),
                        }
                    }
                    crate::ast::LValue::DotAccess {
                        target,
                        property_name,
                    } => {
                        let value_to_assign = self.evaluate(value)?;
                        match &**target {
                            Expression::Identifier(ref target_name) => {
                                if let Some(target_env_ref) =
                                    Environment::find_environment(&self.environment, target_name)
                                {
                                    let mut target_env = target_env_ref.borrow_mut();

                                    if let Some(mut existing_val) =
                                        target_env.values.remove(target_name.as_str())
                                    {
                                        let mut modification_err = None;
                                        match &mut existing_val {
                                            Value::Map(map_rc) => {
                                                let mut_map = Rc::make_mut(map_rc);
                                                mut_map.insert(
                                                    Value::String(property_name.clone()),
                                                    value_to_assign.clone(),
                                                );
                                            }
                                            _ => {
                                                modification_err = Some(EasyScriptError::RuntimeError {
                                                    message: format!("Cannot use dot access on non-map variable '{}'", target_name),
                                                    location: None,
                                                });
                                            }
                                        }

                                        target_env
                                            .values
                                            .insert(target_name.clone(), existing_val);

                                        if let Some(err) = modification_err {
                                            return Err(err);
                                        }
                                        Ok(value_to_assign)
                                    } else {
                                        Err(EasyScriptError::RuntimeError {
                                        message: format!("Internal error: Variable '{}' found but could not be removed for mutation.", target_name),
                                        location: None,
                                    })
                                    }
                                } else {
                                    Err(EasyScriptError::RuntimeError {
                                        message: format!("Undefined variable '{}' in dot assignment.", target_name),
                                        location: None,
                                    })
                                }
                            }
                            _ => Err(EasyScriptError::RuntimeError {
                                message: "Nested accessor assignment (e.g., obj[idx].prop) not yet supported.".to_string(),
                                location: None,
                            }),
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
                                        Err(EasyScriptError::RuntimeError {
                                            message: format!(
                                                "List index out of bounds: {}",
                                                idx_float
                                            ),
                                            location: None,
                                        })
                                    }
                                } else {
                                    Err(EasyScriptError::RuntimeError {
                                        message: format!(
                                            "List index must be a number. Got: {}",
                                            key_val
                                        ),
                                        location: None,
                                    })
                                }
                            }

                            Value::Map(map) => {
                                if let Some(val) = map.get(&key_val) {
                                    // Uses Value's PartialEq for key lookup

                                    Ok(val.clone())
                                } else {
                                    Err(EasyScriptError::RuntimeError {
                                        message: format!("Key not found in map: {}", key_val),
                                        location: None,
                                    })
                                }
                            }

                            _ => Err(EasyScriptError::RuntimeError {
                                message: format!("Cannot index non-list/map type: {}", target_val),
                                location: None,
                            }),
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
                                    Err(EasyScriptError::RuntimeError {
                                        message: format!(
                                            "Property '{}' not found in map.",
                                            property_name
                                        ),
                                        location: None,
                                    })
                                }
                            }

                            _ => Err(EasyScriptError::RuntimeError {
                                message: format!(
                                    "Cannot use dot access on non-map type: {}",
                                    target_val
                                ),
                                location: None,
                            }),
                        }
                    }
                }
            }

            Expression::If {
                condition,
                then_block,
                else_branch,
            } => {
                let condition_val = self.evaluate(condition)?;

                if condition_val.is_truthy() {
                    // Create a new scope for the block and execute it.
                    let new_env = Environment::new_enclosed(&self.environment);
                    self.execute_block(then_block, &new_env)
                } else if let Some(else_expr) = else_branch {
                    // The else_branch can be another IfExpression or a BlockExpression
                    self.evaluate(else_expr) // Evaluate the else expression (which could be a block or another if)
                } else {
                    Ok(Value::Nil) // No else branch, condition false, so return nil
                }
            }

            Expression::For {
                identifier,
                iterable,
                body,
            } => {
                let iterable_val = self.evaluate(iterable)?;
                let mut last_value = Value::Nil; // For loop typically returns nil or the last evaluated expression

                match iterable_val {
                    Value::List(list) => {
                        for element in list.iter() {
                            let loop_env = Environment::new_enclosed(&self.environment);
                            {
                                let mut borrowed_env = loop_env.borrow_mut();
                                borrowed_env.assign(identifier, element.clone());
                            }
                            last_value = self.execute_block(body, &loop_env)?;
                        }
                    }
                    Value::Map(map) => {
                        for (key, _value) in map.iter() {
                            // Iterate over keys for maps
                            let loop_env = Environment::new_enclosed(&self.environment);
                            {
                                let mut borrowed_env = loop_env.borrow_mut();
                                borrowed_env.assign(identifier, key.clone());
                            }
                            last_value = self.execute_block(body, &loop_env)?;
                        }
                    }
                    _ => {
                        return Err(EasyScriptError::RuntimeError {
                            message: format!(
                                "Can only iterate over lists or maps. Got: {}",
                                iterable_val
                            ),
                            location: None,
                        })
                    }
                }
                Ok(last_value)
            }

            Expression::Unary { op, expr } => {
                let right_val = self.evaluate(expr)?;
                match op {
                    crate::ast::UnaryOperator::Negate => {
                        if let Value::Number(num) = right_val {
                            Ok(Value::Number(-num))
                        } else {
                            Err(EasyScriptError::RuntimeError {
                                message: format!(
                                    "Unary '-' operator can only be applied to numbers. Got: {}",
                                    right_val.to_string()
                                ),
                                location: None,
                            })
                        }
                    }
                }
            }

            Expression::Call { callee, args } => {
                let callee_val = self.evaluate(callee)?;
                let mut arg_vals = Vec::new();
                for arg_expr in args {
                    arg_vals.push(self.evaluate(arg_expr)?);
                }

                match callee_val {
                    Value::Function(func_obj) => match func_obj {
                        crate::value::FunctionObject::Native(native_fn) => native_fn(arg_vals)
                            .map_err(|e| EasyScriptError::RuntimeError {
                                message: e,
                                location: None,
                            }),
                        crate::value::FunctionObject::User {
                            params,
                            body,
                            defined_env,
                        } => {
                            if params.len() != arg_vals.len() {
                                return Err(EasyScriptError::RuntimeError {
                                    message: format!(
                                        "Expected {} arguments but got {}.",
                                        params.len(),
                                        arg_vals.len()
                                    ),
                                    location: None,
                                });
                            }

                            // Create a new environment for the function call,
                            // based on the environment where the function was defined (closure)
                            let function_env = Environment::new_enclosed(&defined_env); // 使用 defined_env
                            {
                                let mut borrowed_env = function_env.borrow_mut();
                                for (param_name, arg_val) in params.iter().zip(arg_vals.into_iter())
                                {
                                    borrowed_env.assign(param_name, arg_val);
                                }
                            }
                            // Execute the function body in the new environment
                            self.execute_block(&body, &function_env)
                        }
                    },
                    _ => Err(EasyScriptError::RuntimeError {
                        message: format!("Cannot call non-function value: {}", callee_val),
                        location: None,
                    }),
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
                                Err(EasyScriptError::RuntimeError {
                                    message: "Division by zero.".to_string(),
                                    location: None,
                                })
                            } else {
                                Ok(Value::Number(l / r))
                            }
                        }
                        BinaryOperator::Lt => Ok(Value::Boolean(l < r)),
                        BinaryOperator::Lte => Ok(Value::Boolean(l <= r)),
                        BinaryOperator::Gt => Ok(Value::Boolean(l > r)),
                        BinaryOperator::Gte => Ok(Value::Boolean(l >= r)),
                        _ => Err(EasyScriptError::RuntimeError {
                            message: format!("Unsupported operator '{:?}' for numbers.", op),
                            location: None,
                        }),
                    },
                    (Value::String(l), Value::String(r)) => match op {
                        BinaryOperator::Add => Ok(Value::String(format!("{}{}", l, r))),
                        _ => Err(EasyScriptError::RuntimeError {
                            message: format!("Unsupported operator '{:?}' for strings.", op),
                            location: None,
                        }),
                    },
                    (Value::List(l), Value::List(r)) => match op {
                        BinaryOperator::Add => {
                            let mut new_list = (**l).to_vec();
                            new_list.extend_from_slice(&**r);
                            Ok(Value::List(Rc::new(new_list)))
                        }
                        _ => Err(EasyScriptError::RuntimeError {
                            message: format!("Unsupported operator '{:?}' for lists.", op),
                            location: None,
                        }),
                    },
                    (l, r) => Err(EasyScriptError::RuntimeError {
                        message: format!(
                            "Cannot apply operator '{:?}' to unsupported types: {} and {}",
                            op,
                            l.type_of(),
                            r.type_of()
                        ),
                        location: None,
                    }),
                }
            }
        }
    }

    /// Evaluates a literal value from the AST into a runtime Value.
    fn evaluate_literal(&self, literal: &LiteralValue) -> Result<Value, EasyScriptError> {
        Ok(match literal {
            LiteralValue::Number(n) => Value::Number(*n),
            LiteralValue::String(s) => Value::String(s.clone()),
            LiteralValue::Boolean(b) => Value::Boolean(*b),
            LiteralValue::Nil => Value::Nil,
        })
    }
}
