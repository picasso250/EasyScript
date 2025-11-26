use crate::ast::{Block, Expression, LiteralValue};
use crate::environment::{Environment, EnvironmentRef};
use crate::error::EasyScriptError;
use crate::value::{BoundMethodInner, FunctionObjectInner, Heap, NativeFunction, Object, Value};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter {
    pub heap: Heap,
    environment: EnvironmentRef,
    // Add the builtin_methods field
    builtin_methods: HashMap<&'static str, HashMap<&'static str, NativeFunction>>,
}

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Interpreter {
            heap: Heap::new(),
            environment: Environment::new(),
            builtin_methods: HashMap::new(), // Temporarily initialize as empty
        };

        // Initialize builtin_methods after heap is available
        interpreter.builtin_methods =
            crate::native::init_builtin_methods_map(&mut interpreter.heap);

        // Register global native functions
        let global_env_ref = Rc::clone(&interpreter.environment);
        {
            let mut global_env = global_env_ref.borrow_mut();
            global_env.assign(
                "print",
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::print_fn)),
                ),
            );
            global_env.assign(
                "len",
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::len_fn)),
                ),
            );
            global_env.assign(
                "type",
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::type_fn)),
                ),
            );
            global_env.assign(
                "bool",
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::bool_fn)),
                ),
            );
            global_env.assign(
                "str",
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::str_fn)),
                ),
            );
            global_env.assign(
                "num",
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::num_fn)),
                ),
            );
            global_env.assign(
                "input",
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::input_fn)),
                ),
            );
            global_env.assign(
                "repr", // Register the new repr function
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::repr_fn)),
                ),
            );
            global_env.assign(
                "gc_collect",
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::gc_collect_fn)),
                ),
            );
            global_env.assign(
                "make_map",
                Value::function(
                    &mut interpreter.heap,
                    FunctionObjectInner::Native(Rc::new(crate::native::make_map_fn)),
                ),
            );
        } // The mutable borrow of global_env is dropped here.

        interpreter
    }

    /// Runs the interpreter with a given program block.
    pub fn run(&mut self, program: &Block) -> Result<Value, EasyScriptError> {
        // 克隆 environment，使其与 self 的可变借用不冲突
        let current_env = Rc::clone(&self.environment);

        self.execute_block(program, &current_env)
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

        // Introduce a special variable in the current environment to store the result of expressions.
        // This makes the result a root for the GC.
        let result_var_name = "__res";
        env.borrow_mut()
            .assign(result_var_name, Value::nil(&mut self.heap)); // Initialize with nil

        for (index, (expr, terminated_by_semicolon)) in block.expressions.iter().enumerate() {
            let expr_value = self.evaluate(expr)?;
            // Update the __res variable in the environment.
            env.borrow_mut().assign(result_var_name, expr_value);

            // Only set to nil if terminated by semicolon AND it's not the last expression.
            if *terminated_by_semicolon && index < block.expressions.len() - 1 {
                env.borrow_mut()
                    .assign(result_var_name, Value::nil(&mut self.heap));
            }
        }

        // Restore the previous environment.
        self.environment = previous_env;

        // Return the final value stored in the __res variable.
        // It's guaranteed to exist since we initialized it.
        Ok(env.borrow().get(result_var_name).unwrap().clone())
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
                Ok(Value::list(&mut self.heap, values))
            }

            Expression::MapLiteral(expr_pairs) => {
                let mut map = std::collections::HashMap::<Value, Value>::new();
                for (key_expr, value_expr) in expr_pairs {
                    let key = self.evaluate(key_expr)?;
                    let value = self.evaluate(value_expr)?;

                    // Map keys must be primitive types (String, Number, Boolean)
                    match key.type_of() { // Check the type_of the value
                        "string" | "number" | "boolean" => {
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
                Ok(Value::map(&mut self.heap, map))
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
                Ok(Value::function(
                    &mut self.heap,
                    crate::value::FunctionObjectInner::User {
                        params: params.clone(),
                        body: std::rc::Rc::new(body.clone()),
                        defined_env: Rc::clone(&self.environment), // 捕获当前环境
                    },
                ))
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

                        Ok(Value::nil(&mut self.heap)) // 赋值表达式现在返回 nil
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
                                        match existing_val.0.deref_mut() { // Access the inner Object mutably
                                            Object::List(list) => {
                                                if let Some(idx_float) = key_val.0.deref().as_number() {
                                                    let index = *idx_float as usize;
                                                    if index < list.len() {
                                                        list[index] = value_to_assign.clone();
                                                    } else {
                                                        modification_err = Some(EasyScriptError::RuntimeError {
                                                            message: format!("List index out of bounds for assignment: {}", idx_float),
                                                            location: None,
                                                        });
                                                    }
                                                } else {
                                                    modification_err = Some(EasyScriptError::RuntimeError {
                                                        message: format!("List index must be a number for assignment. Got: {}", key_val.type_of()),
                                                            location: None,
                                                        });
                                                    }
                                                }
                                            Object::Map(map) => {
                                                // 检查 key_val 是否是允许的键类型
                                                match key_val.type_of() {
                                                    "string" | "number" | "boolean" => {
                                                        map.insert(key_val, value_to_assign.clone()); // 直接使用 key_val
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
                                        Ok(Value::nil(&mut self.heap))
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
                                        match existing_val.0.deref_mut() { // Access the inner Object mutably
                                            Object::Map(map) => {
                                                map.insert(
                                                    Value::string(&mut self.heap, property_name.clone()),
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

                        match target_val.0.deref() {
                            Object::List(list) => {
                                if let Some(idx_float) = key_val.0.deref().as_number() {
                                    let index = *idx_float as usize; // Cast to usize for list indexing

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
                                            key_val.type_of()
                                        ),
                                        location: None,
                                    })
                                }
                            }

                            Object::Map(map) => {
                                // 检查 key_val 是否是允许的键类型
                                match key_val.type_of() {
                                    "string" | "number" | "boolean" => {
                                        if let Some(val) = map.get(&key_val) { // 修改这里
                                            Ok(val.clone())
                                        } else {
                                            Ok(Value::nil(&mut self.heap)) // Return nil if property not found in map
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
                                message: format!(
                                    "Cannot index non-list/map type: {}",
                                    target_val.type_of()
                                ),
                                location: None,
                            }),
                        }
                    }

                    crate::ast::AccessType::Dot(property_name) => {
                        // 1. Check for built-in methods first
                        if let Some(methods_for_type) =
                            self.builtin_methods.get(target_val.type_of())
                        {
                            if methods_for_type.contains_key(property_name.as_str()) {
                                // Found a built-in method, return a BoundMethod
                                return Ok(Value::bound_method(
                                    &mut self.heap,
                                    BoundMethodInner {
                                        receiver: target_val.clone(),
                                        method_name: property_name.clone(),
                                    },
                                ));
                            }
                        }

                        // 2. Fallback to map property lookup if not a built-in method
                        if target_val.type_of() == "map" {
                            let key_val = Value::string(&mut self.heap, property_name.clone());
                            if let Some(val) = target_val.0.deref().as_map().unwrap().get(&key_val)
                            {
                                Ok(val.clone())
                            } else {
                                Ok(Value::nil(&mut self.heap)) // Return nil if property not found in map
                            }
                        } else {
                            // If not a map and no built-in method found
                            Err(EasyScriptError::RuntimeError {
                                message: format!(
                                    "Cannot use dot access on type '{}'. No method '{}' or map key found.",
                                    target_val.type_of(),
                                    property_name
                                ),
                                location: None,
                            })
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
                    Ok(Value::nil(&mut self.heap)) // No else branch, condition false, so return nil
                }
            }

            Expression::ForIn {
                identifier,
                iterable,
                condition, // Destructure the condition
                body,
            } => {
                let iterable_val = self.evaluate(iterable)?;
                let mut collected_values = Vec::new(); // Collect results here

                match iterable_val.0.deref() {
                    Object::List(list) => {
                        for element in list.iter() {
                            let loop_env = Environment::new_enclosed(&self.environment);
                            {
                                let mut borrowed_env = loop_env.borrow_mut();
                                borrowed_env.assign(identifier, element.clone());
                            }

                            // Evaluate the condition (if present) in the loop's environment
                            let should_execute_body = if let Some(cond_expr) = &condition {
                                // Temporarily switch interpreter's environment for condition evaluation
                                let original_env_rc = Rc::clone(&self.environment);
                                self.environment = Rc::clone(&loop_env);
                                let cond_val = self.evaluate(cond_expr)?;
                                self.environment = original_env_rc; // Restore original environment
                                cond_val.is_truthy()
                            } else {
                                true // No condition, so always execute
                            };

                            if should_execute_body {
                                let iteration_result = self.execute_block(body, &loop_env)?;
                                collected_values.push(iteration_result);
                            }
                        }
                    }
                    Object::Map(map) => {
                        for (key, _value) in map.iter() {
                            // Iterate over keys for maps
                            let loop_env = Environment::new_enclosed(&self.environment);
                            {
                                let mut borrowed_env = loop_env.borrow_mut();
                                borrowed_env.assign(identifier, key.clone());
                            }

                            // Evaluate the condition (if present) in the loop's environment
                            let should_execute_body = if let Some(cond_expr) = &condition {
                                let original_env_rc = Rc::clone(&self.environment);
                                self.environment = Rc::clone(&loop_env);
                                let cond_val = self.evaluate(cond_expr)?;
                                self.environment = original_env_rc;
                                cond_val.is_truthy()
                            } else {
                                true // No condition, so always execute
                            };

                            if should_execute_body {
                                let iteration_result = self.execute_block(body, &loop_env)?;
                                collected_values.push(iteration_result);
                            }
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
                Ok(Value::list(&mut self.heap, collected_values)) // Return the collected list
            }

            Expression::ForCondition { condition, body } => {
                let mut collected_values = Vec::new(); // Collect results here

                loop {
                    // Evaluate condition in the current scope
                    let condition_val = self.evaluate(condition)?;

                    if condition_val.is_truthy() {
                        // Create a new scope for the body of each iteration
                        let loop_env = Environment::new_enclosed(&self.environment);
                        let iteration_result = self.execute_block(body, &loop_env)?;
                        collected_values.push(iteration_result);
                    } else {
                        break; // Condition is false, exit loop
                    }
                }
                Ok(Value::list(&mut self.heap, collected_values)) // Return the collected list of results
            }

            Expression::Unary { op, expr } => {
                let right_val = self.evaluate(expr)?;
                match op {
                    crate::ast::UnaryOperator::Negate => {
                        if let Some(num) = right_val.0.deref().as_number() {
                            Ok(Value::number(&mut self.heap, -num))
                        } else {
                            Err(EasyScriptError::RuntimeError {
                                message: format!(
                                    "Unary '-' operator can only be applied to numbers. Got: {}",
                                    right_val.type_of()
                                ),
                                location: None,
                            })
                        }
                    }
                    crate::ast::UnaryOperator::Not => {
                        let is_truthy = right_val.is_truthy();
                        Ok(Value::boolean(&mut self.heap, !is_truthy))
                    }
                }
            }

            Expression::Call { callee, args } => {
                let callee_val = self.evaluate(callee)?;
                let mut arg_vals = Vec::new();
                for arg_expr in args {
                    arg_vals.push(self.evaluate(arg_expr)?);
                }

                match callee_val.0.deref() {
                    crate::value::Object::Function(func_obj) => match func_obj {
                        crate::value::FunctionObjectInner::User {
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
                        crate::value::FunctionObjectInner::Native(native_fn) => {
                            // 调用原生函数
                            native_fn(&mut self.heap, &self.environment, arg_vals).map_err(|e| {
                                EasyScriptError::RuntimeError {
                                    message: e,
                                    location: None,
                                }
                            })
                        }
                    },
                    crate::value::Object::BoundMethod(bound_method_inner) => {
                        let receiver = bound_method_inner.receiver.clone();
                        let method_name = bound_method_inner.method_name.clone();

                        // Look up the actual native function from the interpreter's built-in methods
                        if let Some(methods_for_type) = self.builtin_methods.get(receiver.type_of())
                        {
                            if let Some(native_method_fn) =
                                methods_for_type.get(method_name.as_str())
                            {
                                // Prepend the receiver to the arguments
                                let mut full_args = vec![receiver];
                                full_args.extend(arg_vals);

                                native_method_fn(&mut self.heap, &self.environment, full_args)
                                    .map_err(|e| EasyScriptError::RuntimeError {
                                        message: e,
                                        location: None,
                                    })
                            } else {
                                // This should ideally not happen if Accessor correctly returns BoundMethod
                                Err(EasyScriptError::RuntimeError {
                                    message: format!(
                                        "Internal error: Bound method '{}' not found for type '{}'.",
                                        method_name,
                                        receiver.type_of()
                                    ),
                                    location: None,
                                })
                            }
                        } else {
                            Err(EasyScriptError::RuntimeError {
                                message: format!(
                                    "Internal error: No built-in methods registered for type '{}'.",
                                    receiver.type_of()
                                ),
                                location: None,
                            })
                        }
                    }
                    _ => Err(EasyScriptError::RuntimeError {
                        message: format!(
                            "Cannot call non-function or non-method value: {}",
                            callee_val
                        ),
                        location: None,
                    }),
                }
            }

            Expression::Binary { left, op, right } => {
                let left_val = self.evaluate(left)?;
                // Short-circuiting for logical operators
                match op {
                    BinaryOperator::Or => {
                        if left_val.is_truthy() {
                            return Ok(left_val);
                        }
                        let right_val = self.evaluate(right)?;
                        return Ok(right_val);
                    }
                    BinaryOperator::And => {
                        if !left_val.is_truthy() {
                            return Ok(left_val);
                        }
                        let right_val = self.evaluate(right)?;
                        return Ok(right_val);
                    }
                    _ => {}
                }

                let right_val = self.evaluate(right)?; // Evaluate right_val only if not short-circuited
                use crate::ast::BinaryOperator;

                match op {
                    BinaryOperator::Eq => {
                        return Ok(Value::boolean(&mut self.heap, left_val == right_val))
                    }
                    BinaryOperator::Neq => {
                        return Ok(Value::boolean(&mut self.heap, left_val != right_val))
                    }
                    _ => {}
                }

                let left_obj = left_val.0.deref();
                let right_obj = right_val.0.deref();

                match (left_obj, right_obj) {
                    (Object::Number(l), Object::Number(r)) => match op {
                        BinaryOperator::Add => Ok(Value::number(&mut self.heap, l + r)),
                        BinaryOperator::Sub => Ok(Value::number(&mut self.heap, l - r)),
                        BinaryOperator::Mul => Ok(Value::number(&mut self.heap, l * r)),
                        BinaryOperator::Div => {
                            if *r == 0.0 {
                                Err(EasyScriptError::RuntimeError {
                                    message: "Division by zero.".to_string(),
                                    location: None,
                                })
                            } else {
                                Ok(Value::number(&mut self.heap, l / r))
                            }
                        }
                        BinaryOperator::Mod => Ok(Value::number(&mut self.heap, l % r)),
                        BinaryOperator::BitAnd => Ok(Value::number(&mut self.heap, (*l as i64 & *r as i64) as f64)),
                        BinaryOperator::BitOr => Ok(Value::number(&mut self.heap, (*l as i64 | *r as i64) as f64)),
                        BinaryOperator::BitXor => Ok(Value::number(&mut self.heap, (*l as i64 ^ *r as i64) as f64)),
                        BinaryOperator::ShL => {
                            if *r < 0.0 {
                                return Err(EasyScriptError::RuntimeError {
                                    message: "Shift amount cannot be negative.".to_string(),
                                    location: None,
                                });
                            }
                            Ok(Value::number(&mut self.heap, (*l as i64).wrapping_shl(*r as u32) as f64))
                        }
                        BinaryOperator::ShR => {
                            if *r < 0.0 {
                                return Err(EasyScriptError::RuntimeError {
                                    message: "Shift amount cannot be negative.".to_string(),
                                    location: None,
                                });
                            }
                            Ok(Value::number(&mut self.heap, (*l as i64).wrapping_shr(*r as u32) as f64))
                        }
                        BinaryOperator::Lt => Ok(Value::boolean(&mut self.heap, l < r)),
                        BinaryOperator::Lte => Ok(Value::boolean(&mut self.heap, l <= r)),
                        BinaryOperator::Gt => Ok(Value::boolean(&mut self.heap, l > r)),
                        BinaryOperator::Gte => Ok(Value::boolean(&mut self.heap, l >= r)),
                        _ => Err(EasyScriptError::RuntimeError {
                            message: format!("Unsupported operator '{:?}' for numbers.", op),
                            location: None,
                        }),
                    },
                    (Object::String(l), Object::String(r)) => match op {
                        BinaryOperator::Add => {
                            Ok(Value::string(&mut self.heap, format!("{}{}", l, r)))
                        }
                        _ => Err(EasyScriptError::RuntimeError {
                            message: format!("Unsupported operator '{:?}' for strings.", op),
                            location: None,
                        }),
                    },
                    (Object::List(l), Object::List(r)) => match op {
                        BinaryOperator::Add => {
                            let mut new_list = l.to_vec();
                            new_list.extend_from_slice(r);
                            Ok(Value::list(&mut self.heap, new_list))
                        }
                        _ => Err(EasyScriptError::RuntimeError {
                            message: format!("Unsupported operator '{:?}' for lists.", op),
                            location: None,
                        }),
                    },
                    (_l, _r) => Err(EasyScriptError::RuntimeError {
                        message: format!(
                            "Cannot apply operator '{:?}' to unsupported types: {} and {}",
                            op,
                            left_val.type_of(),
                            right_val.type_of()
                        ),
                        location: None,
                    }),
                }
            }
        }
    }

    /// Evaluates a literal value from the AST into a runtime Value.
    fn evaluate_literal(&mut self, literal: &LiteralValue) -> Result<Value, EasyScriptError> {
        Ok(match literal {
            LiteralValue::Number(n) => Value::number(&mut self.heap, *n),
            LiteralValue::String(s) => Value::string(&mut self.heap, s.clone()),
            LiteralValue::Boolean(b) => Value::boolean(&mut self.heap, *b),
            LiteralValue::Nil => Value::nil(&mut self.heap),
        })
    }
}
