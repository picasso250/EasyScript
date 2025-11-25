use crate::ast::{Block, Expression, LiteralValue};
use crate::environment::{Environment, EnvironmentRef};
use crate::error::EasyScriptError;
use crate::native;
use crate::value::{NativeFunction, Value}; // Import NativeFunction type alias
use std::collections::HashMap;
use std::rc::Rc; // Changed from Arc to Rc // Required for HashMap in method_registry

pub struct Interpreter {
    // The environment is now a reference-counted pointer to a mutable Environment.
    environment: EnvironmentRef,
    // Each interpreter instance now holds its own method registry
    method_registry: HashMap<&'static str, HashMap<&'static str, NativeFunction>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let interpreter = Interpreter {
            // Create the top-level (global) environment.
            environment: Environment::new(),
            // Initialize the method registry
            method_registry: native::init_builtin_methods_map(),
        };

        // Register native functions
        {
            let mut env = interpreter.environment.borrow_mut();
            env.assign(
                "print",
                Value::Function(crate::value::FunctionObject::Native(
                    (Rc::new(move |args| native::print_fn(args))) as NativeFunction,
                )),
            );
            // `len` is now a method. Global registration remains as a fallback.
            env.assign(
                "len",
                Value::Function(crate::value::FunctionObject::Native(
                    (Rc::new(move |args| native::len_fn(args))) as NativeFunction,
                )),
            );
            env.assign(
                "type",
                Value::Function(crate::value::FunctionObject::Native(
                    (Rc::new(move |args| native::type_fn(args))) as NativeFunction,
                )),
            );
            env.assign(
                "str",
                Value::Function(crate::value::FunctionObject::Native(
                    (Rc::new(move |args| native::str_fn(args))) as NativeFunction,
                )),
            );
            env.assign(
                "num",
                Value::Function(crate::value::FunctionObject::Native(
                    (Rc::new(move |args| native::num_fn(args))) as NativeFunction,
                )),
            );
            env.assign(
                "input",
                Value::Function(crate::value::FunctionObject::Native(
                    (Rc::new(move |args| native::input_fn(args))) as NativeFunction,
                )),
            );
            env.assign(
                "bool",
                Value::Function(crate::value::FunctionObject::Native(
                    (Rc::new(move |args| native::bool_fn(args))) as NativeFunction,
                )),
            );
            // `keys`, `values` are now methods, no longer globally registered as functions.
            // env.assign(
            //     "keys",
            //     Value::Function(crate::value::FunctionObject::Native(Rc::new(
            //         native::keys_fn as fn(Vec<Value>) -> Result<Value, String>,
            //     ))),
            // );
            // env.assign(
            //     "values",
            //     Value::Function(crate::value::FunctionObject::Native(Rc::new(
            //         native::values_fn as fn(Vec<Value>) -> Result<Value, String>,
            //     ))),
            // );
        } // env 借用在此处结束

        interpreter
    }

    /// Runs the interpreter with a given program block.
    pub fn run(&mut self, program: &Block) -> Result<Value, EasyScriptError> {
        // 克隆 environment，使其与 self 的可变借用不冲突
        let current_env = Rc::clone(&self.environment);
        eprintln!(
            "DEBUG: program.expressions length: {}",
            program.expressions.len()
        ); // DEBUG line
        self.execute_block(program, &current_env)
    }

    /// Helper function to find a built-in method
    fn find_builtin_method(&self, type_name: &str, method_name: &str) -> Option<NativeFunction> {
        self.method_registry
            .get(type_name)?
            .get(method_name)
            .cloned()
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
        for (expr, terminated_by_semicolon) in &block.expressions {
            last_value = self.evaluate(expr)?;
            if *terminated_by_semicolon {
                last_value = Value::Nil; // If terminated by semicolon, return nil
            }
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
                let mut map = std::collections::HashMap::<Value, Value>::new(); // 修改这里
                for (key_expr, value_expr) in expr_pairs {
                    let key = self.evaluate(key_expr)?;
                    let value = self.evaluate(value_expr)?;
                    
                    // Map keys must be primitive types (String, Number, Boolean)
                    match key {
                        Value::String(_) | Value::Number(_) | Value::Boolean(_) => {
                            // These types are hashable and allowed as keys
                            map.insert(key, value);
                        },
                        _ => {
                            return Err(EasyScriptError::RuntimeError {
                                message: format!(
                                    "Map keys must be primitive types (String, Number, Boolean), but got '{}'.",
                                    key.type_of()
                                ),
                                location: None,
                            })
                        }
                    };
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

                        Ok(value_to_assign) // 赋值表达式现在返回被赋的值
                    }

                    crate::ast::LValue::IndexAccess { target, key } => {
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
                                                // 检查 key_val 是否是允许的键类型
                                                match key_val {
                                                    Value::String(_) | Value::Number(_) | Value::Boolean(_) => {
                                                        mut_map.insert(key_val, value_to_assign.clone()); // 直接使用 key_val
                                                    },
                                                    _ => {
                                                        modification_err = Some(EasyScriptError::RuntimeError {
                                                            message: format!("Map keys must be primitive types (String, Number, Boolean) for assignment. Got: {}", key_val.type_of()),
                                                            location: None,
                                                        });
                                                    }
                                                }
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
                                                    Value::String(property_name.clone()), // 修改这里
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
                                // 检查 key_val 是否是允许的键类型
                                match key_val {
                                    Value::String(_) | Value::Number(_) | Value::Boolean(_) => {
                                        if let Some(val) = map.get(&key_val) { // 修改这里
                                            Ok(val.clone())
                                        } else {
                                            Ok(Value::Nil) // Return nil if property not found in map
                                        }
                                    },
                                    _ => {
                                        return Err(EasyScriptError::RuntimeError {
                                            message: format!(
                                                "Map keys must be primitive types (String, Number, Boolean). Got: {}",
                                                key_val.type_of()
                                            ),
                                            location: None,
                                        })
                                    }
                                }
                            }

                            _ => Err(EasyScriptError::RuntimeError {
                                message: format!("Cannot index non-list/map type: {}", target_val),
                                location: None,
                            }),
                        }
                    }

                    crate::ast::AccessType::Dot(property_name) => {
                        // Method-first, property-fallback
                        // 1. Check for built-in method
                        if let Some(method_fn) =
                            self.find_builtin_method(target_val.type_of(), &property_name)
                        {
                            return Ok(Value::BoundMethod {
                                receiver: Box::new(target_val),
                                method: method_fn,
                            });
                        }

                        // 2. Fallback to map property lookup
                        match target_val {
                            Value::Map(map) => {
                                let key_val = Value::String(property_name.clone()); // 将 String 包装成 Value::String
                                if let Some(val) = map.get(&key_val) { // 修改这里
                                    Ok(val.clone())
                                } else {
                                    Ok(Value::Nil) // Return nil if property not found in map
                                }
                            }
                            _ => Err(EasyScriptError::RuntimeError {
                                message: format!(
                                    "Cannot use dot access on non-map type: {} or no method '{}' found.",
                                    target_val.type_of(),
                                    property_name
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
                let mut collected_values = Vec::new(); // Collect results here

                match iterable_val {
                    Value::List(list) => {
                        for element in list.iter() {
                            let loop_env = Environment::new_enclosed(&self.environment);
                            {
                                let mut borrowed_env = loop_env.borrow_mut();
                                borrowed_env.assign(identifier, element.clone());
                            }
                            let iteration_result = self.execute_block(body, &loop_env)?;
                            collected_values.push(iteration_result);
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
                            let iteration_result = self.execute_block(body, &loop_env)?;
                            collected_values.push(iteration_result);
                        }
                    }
                    _ => {
                        return Err(EasyScriptError::RuntimeError {
                            message: format!(
                                "Can only iterate over lists or maps. Got: {}",
                                iterable_val.type_of()
                            ),
                            location: None,
                        })
                    }
                }
                Ok(Value::List(Rc::new(collected_values))) // Return the collected list
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
                    Value::BoundMethod { receiver, method } => {
                        let mut final_args = vec![*receiver];
                        final_args.extend(arg_vals);
                        method(final_args).map_err(|e| EasyScriptError::RuntimeError {
                            message: e,
                            location: None,
                        })
                    }
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
