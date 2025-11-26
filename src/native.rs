use crate::environment::EnvironmentRef;
use crate::value::{Heap, NativeFunction, Object, Value};
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc;

// --- BUILT-IN METHODS REGISTRY ---
// BUILTIN_METHODS is no longer a static OnceCell, it will be initialized per Interpreter instance.

// Helper function to initialize the map
pub fn init_builtin_methods_map(
    _heap: &mut Heap,
) -> HashMap<&'static str, HashMap<&'static str, NativeFunction>> {
    let mut methods = HashMap::new();

    // --- String Methods ---
    let mut string_methods = HashMap::new();
    string_methods.insert("trim", Rc::new(str_trim_fn) as NativeFunction);
    string_methods.insert("len", Rc::new(len_fn) as NativeFunction);
    string_methods.insert("starts_with", Rc::new(str_starts_with_fn) as NativeFunction);
    string_methods.insert("find", Rc::new(str_find_fn) as NativeFunction);
    string_methods.insert("contains", Rc::new(str_contains_fn) as NativeFunction);
    string_methods.insert("replace", Rc::new(str_replace_fn) as NativeFunction);
    string_methods.insert("split", Rc::new(str_split_fn) as NativeFunction);
    string_methods.insert("to_upper", Rc::new(str_to_upper_fn) as NativeFunction);
    string_methods.insert("to_lower", Rc::new(str_to_lower_fn) as NativeFunction);
    methods.insert("string", string_methods);

    // --- List Methods ---
    let mut list_methods = HashMap::new();
    list_methods.insert("len", Rc::new(len_fn) as NativeFunction);
    list_methods.insert("push", Rc::new(list_push_fn) as NativeFunction);
    methods.insert("list", list_methods);

    // --- Map Methods ---
    let mut map_methods = HashMap::new();
    map_methods.insert("keys", Rc::new(keys_fn) as NativeFunction);
    map_methods.insert("values", Rc::new(values_fn) as NativeFunction);
    map_methods.insert("len", Rc::new(len_fn) as NativeFunction);
    methods.insert("map", map_methods);

    methods
}

// Native string starts_with method
pub fn str_starts_with_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "starts_with() expected 2 arguments (self, prefix), but got {}",
            args.len()
        ));
    }

    let self_string = match &args[0].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "starts_with() method expected a string as the receiver, but got type '{}'.",
                args[0].type_of()
            ));
        }
    };

    let prefix = match &args[1].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "starts_with() method expected a string as the prefix argument, but got type '{}'.",
                args[1].type_of()
            ));
        }
    };

    Ok(Value::boolean(heap, self_string.starts_with(prefix)))
}

// Native string contains method
pub fn str_contains_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "contains() expected 2 arguments (self, substring), but got {}",
            args.len()
        ));
    }

    let self_string = match &args[0].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "contains() method expected a string as the receiver, but got type '{}'.",
                args[0].type_of()
            ));
        }
    };

    let substring = match &args[1].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "contains() method expected a string as the substring argument, but got type '{}'.",
                args[1].type_of()
            ));
        }
    };

    Ok(Value::boolean(heap, self_string.contains(substring)))
}
// Native string find method
pub fn str_find_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "find() expected 2 arguments (self, substring), but got {}",
            args.len()
        ));
    }

    let self_string = match &args[0].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "find() method expected a string as the receiver, but got type '{}'.",
                args[0].type_of()
            ));
        }
    };

    let substring = match &args[1].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "find() method expected a string as the substring argument, but got type '{}'.",
                args[1].type_of()
            ));
        }
    };

    // Use Rust's `find` method which returns `Option<usize>` (byte index)
    if let Some(byte_index) = self_string.find(substring) {
        // Convert byte index to character index
        let char_index = self_string[..byte_index].chars().count();
        Ok(Value::number(heap, char_index as f64))
    } else {
        Ok(Value::nil(heap))
    }
}

// Native string replace method
pub fn str_replace_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 3 {
        return Err(format!(
            "replace() expected 3 arguments (self, old, new), but got {}",
            args.len()
        ));
    }

    let self_string = match &args[0].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "replace() method expected a string as the receiver, but got type '{}'.",
                args[0].type_of()
            ));
        }
    };

    let old_substring = match &args[1].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "replace() method expected a string as the 'old' argument, but got type '{}'.",
                args[1].type_of()
            ));
        }
    };

    let new_substring = match &args[2].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "replace() method expected a string as the 'new' argument, but got type '{}'.",
                args[2].type_of()
            ));
        }
    };

    Ok(Value::string(
        heap,
        self_string.replace(old_substring, new_substring),
    ))
}

// Native string split method
pub fn str_split_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "split() expected 2 arguments (self, delimiter), but got {}",
            args.len()
        ));
    }

    let self_string = match &args[0].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "split() method expected a string as the receiver, but got type '{}'.",
                args[0].type_of()
            ));
        }
    };

    let delimiter = match &args[1].0.deref() {
        // Access inner Object
        Object::String(s) => s,
        _other => {
            return Err(format!(
                "split() method expected a string as the delimiter argument, but got type '{}'.",
                args[1].type_of()
            ));
        }
    };

    // If delimiter is empty, split by characters
    let parts: Vec<Value> = if delimiter.is_empty() {
        self_string
            .chars()
            .map(|c| Value::string(heap, c.to_string()))
            .collect()
    } else {
        self_string
            .split(delimiter)
            .map(|s| Value::string(heap, s.to_string()))
            .collect()
    };

    Ok(Value::list(heap, parts))
}

// Native string to_upper method
pub fn str_to_upper_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "to_upper() expected 1 argument (self), but got {}",
            args.len()
        ));
    }

    match &args[0].0.deref() {
        Object::String(s) => Ok(Value::string(heap, s.to_uppercase())),
        _other => Err(format!(
            "to_upper() method expected a string, but got type '{}'.",
            args[0].type_of()
        )),
    }
}

// Native string to_lower method
pub fn str_to_lower_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "to_lower() expected 1 argument (self), but got {}",
            args.len()
        ));
    }

    match &args[0].0.deref() {
        Object::String(s) => Ok(Value::string(heap, s.to_lowercase())),
        _other => Err(format!(
            "to_lower() method expected a string, but got type '{}'.",
            args[0].type_of()
        )),
    }
}

// Helper function to find a built-in method (no longer needed here, will be in Interpreter)
// pub fn find_builtin_method(type_name: &str, method_name: &str) -> Option<NativeFunction> {
//     BUILTIN_METHODS
//         .get_or_init(init_builtin_methods) // Initialize if not already
//         .get(type_name)?
//         .get(method_name)
//         .cloned()
// }

// Native print function
pub fn print_fn(heap: &mut Heap, _env: &EnvironmentRef, args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        writeln!(io::stdout()).map_err(|e| e.to_string())?;
    } else {
        let output: Vec<String> = args.iter().map(|arg| format!("{}", arg)).collect();
        writeln!(io::stdout(), "{}", output.join(" ")).map_err(|e| e.to_string())?;
    }
    Ok(Value::nil(heap))
}

// Native len method (polymorphic, but called as a method)
pub fn len_fn(heap: &mut Heap, _env: &EnvironmentRef, args: Vec<Value>) -> Result<Value, String> {
    // Expect `self` (the string/list/map) as the first argument, and no other arguments.
    if args.len() != 1 {
        return Err(format!(
            "len() expected 1 argument (self), but got {}",
            args.len()
        ));
    }

    let len = match args[0].0.deref() {
        Object::String(s) => s.chars().count(),
        Object::List(l) => l.len(),
        Object::Map(m) => m.len(),
        _other => {
            return Err(format!(
                "len() method does not support type '{}'.",
                args[0].type_of()
            ));
        }
    };

    Ok(Value::number(heap, len as f64))
}

// Native string trim method
pub fn str_trim_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    // Expect `self` (the string) as the first argument, and no other arguments.
    if args.len() != 1 {
        return Err(format!(
            "trim() expected 1 argument (self), but got {}",
            args.len()
        ));
    }

    match &args[0].0.deref() {
        Object::String(s) => Ok(Value::string(heap, s.trim().to_string())),
        _other => Err(format!(
            "trim() method expected a string, but got type '{}'.",
            args[0].type_of()
        )),
    }
}

// Native type function
pub fn type_fn(heap: &mut Heap, _env: &EnvironmentRef, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "type() expected 1 argument, but got {}",
            args.len()
        ));
    }

    let type_str = args[0].type_of();
    Ok(Value::string(heap, type_str.to_string()))
}

// Native str conversion function
pub fn str_fn(heap: &mut Heap, _env: &EnvironmentRef, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("str() expected 1 argument, but got {}", args.len()));
    }

    let result_string = match args[0].0.deref() {
        Object::String(s) => s.clone(), // If already a string, just clone its content (no extra quotes)
        _ => format!("{}", args[0]),    // For other types, use Display trait
    };

    Ok(Value::string(heap, result_string))
}

// Native num conversion function
/// Converts a value to a number.
///
/// - Number values are returned as-is.
/// - String values are parsed to f64; if parsing fails, returns `Value::Nil`.
/// - Boolean `true` becomes 1.0, `false` becomes 0.0.
/// - `Nil` becomes 0.0.
/// - For any other type, returns `Value::Nil`.
pub fn num_fn(heap: &mut Heap, _env: &EnvironmentRef, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("num() expected 1 argument, but got {}", args.len()));
    }

    match &args[0] {
        value_val if value_val.type_of() == "number" => {
            // Check using type_of()
            Ok(Value::number(
                heap,
                *value_val.0.deref().as_number().unwrap(),
            ))
        }
        value_val if value_val.type_of() == "string" => {
            // Check using type_of()
            match value_val
                .0
                .deref()
                .as_string()
                .unwrap()
                .trim()
                .parse::<f64>()
            {
                Ok(n) => Ok(Value::number(heap, n)),
                Err(_) => Ok(Value::nil(heap)), // If string parsing fails, return Nil
            }
        }
        value_val if value_val.type_of() == "boolean" => {
            // Check using type_of()
            Ok(Value::number(
                heap,
                if *value_val.0.deref().as_boolean().unwrap() {
                    1.0
                } else {
                    0.0
                },
            ))
        }
        value_val if value_val.type_of() == "nil" => {
            // Check using type_of()
            Ok(Value::number(heap, 0.0))
        }
        _other => {
            // For other types (e.g., List, Map, Function), return Nil as they cannot be coerced to a number.
            Ok(Value::nil(heap))
        }
    }
}

// Native input function
pub fn input_fn(heap: &mut Heap, _env: &EnvironmentRef, args: Vec<Value>) -> Result<Value, String> {
    if args.len() > 1 {
        return Err(format!(
            "input() expected 0 or 1 argument, but got {}",
            args.len()
        ));
    }

    let mut line = String::new();

    if let Some(prompt_value) = args.get(0) {
        print!("{}", prompt_value);
        std::io::stdout().flush().map_err(|e| e.to_string())?;
    }

    std::io::stdin()
        .read_line(&mut line)
        .map_err(|e| e.to_string())?;

    Ok(Value::string(
        heap, // Use the provided heap
        line.trim_end_matches(&['\n', '\r'][..]).to_string(),
    ))
}

// Native bool conversion function
pub fn bool_fn(heap: &mut Heap, _env: &EnvironmentRef, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "bool() expected 1 argument, but got {}",
            args.len()
        ));
    }

    Ok(Value::boolean(heap, args[0].is_truthy()))
}

// Native repr conversion function
pub fn repr_fn(heap: &mut Heap, _env: &EnvironmentRef, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "repr() expected 1 argument, but got {}",
            args.len()
        ));
    }

    // Use the custom repr_string() for Python-like repr()
    Ok(Value::string(heap, args[0].repr_string()))
}

// Native keys method
pub fn keys_fn(heap: &mut Heap, _env: &EnvironmentRef, args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "keys() expected 1 argument (self), but got {}.",
            args.len()
        ));
    }

    match args[0].0.deref() {
        Object::Map(m) => {
            let keys: Vec<Value> = m.keys().map(|k| k.clone()).collect();
            Ok(Value::list(heap, keys))
        }
        _other => Err(format!(
            "keys() method expected a map, but got type '{}'.",
            args[0].type_of()
        )),
    }
}

// Native values method
pub fn values_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "values() expected 1 argument (self), but got {}.",
            args.len()
        ));
    }

    match args[0].0.deref() {
        Object::Map(m) => {
            let values: Vec<Value> = m.values().cloned().collect();
            Ok(Value::list(heap, values))
        }
        _other => Err(format!(
            "values() method expected a map, but got type '{}'.",
            args[0].type_of()
        )),
    }
}

// Native list push method
pub fn list_push_fn(
    heap: &mut Heap,
    _env: &EnvironmentRef,
    mut args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 2 {
        return Err(format!(
            "push() expected 2 arguments (self, element), but got {}",
            args.len()
        ));
    }

    // 先克隆出 element_to_push，避免与 args[0] 的可变借用冲突
    let element_to_push = args[1].clone();

    // 然后再获取 args[0] 的可变引用
    let list_value = &mut args[0];

    match list_value.0.deref_mut() {
        // Directly deref_mut the GcRef to get Object
        Object::List(list) => {
            list.push(element_to_push); // Push to the mutable Vec directly
            Ok(Value::nil(heap))
        }
        _other => Err(format!(
            "push() method expected a list as the receiver, but got type '{}'.",
            list_value.type_of()
        )),
    }
}

// Native GC collection function
pub fn gc_collect_fn(
    heap: &mut Heap,
    env: &EnvironmentRef,
    args: Vec<Value>,
) -> Result<Value, String> {
    if !args.is_empty() {
        return Err(format!(
            "gc_collect() expected 0 arguments, but got {}",
            args.len()
        ));
    }

    let mut roots = Vec::new();
    let mut current_env = Some(Rc::clone(env));
    while let Some(env_ref) = current_env {
        let env_borrow = env_ref.borrow();
        for value in env_borrow.values.values() {
            roots.push(value.clone());
        }
        current_env = env_borrow.parent.as_ref().map(Rc::clone);
    }

    // Also add the arguments passed to this function as roots.
    // Although for gc_collect() it's empty, it's good practice for other potential GC-aware functions.
    roots.extend_from_slice(&args);

    let collected_count = heap.collect(&roots);

    Ok(Value::number(heap, collected_count as f64))
}
