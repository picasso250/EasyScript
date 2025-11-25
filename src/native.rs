use crate::value::{NativeFunction, Value};
use std::collections::HashMap;
use std::io::{self, Write};
use std::rc::Rc; // Used for NativeFunction now // Required for stdout().flush() and writeln!

// --- BUILT-IN METHODS REGISTRY ---
// BUILTIN_METHODS is no longer a static OnceCell, it will be initialized per Interpreter instance.

// Helper function to initialize the map
pub fn init_builtin_methods_map() -> HashMap<&'static str, HashMap<&'static str, NativeFunction>> {
    let mut methods = HashMap::new();

    // --- String Methods ---
    let mut string_methods = HashMap::new();
    string_methods.insert(
        "trim",
        (Rc::new(move |args| str_trim_fn(args))) as NativeFunction,
    );
    string_methods.insert("len", (Rc::new(move |args| len_fn(args))) as NativeFunction);
    methods.insert("string", string_methods);

    // --- List Methods ---
    let mut list_methods = HashMap::new();
    list_methods.insert("len", (Rc::new(move |args| len_fn(args))) as NativeFunction);
    // list_methods.insert("append", Rc::new(list_append_fn)); // Will be implemented later
    methods.insert("list", list_methods);

    // --- Map Methods ---
    let mut map_methods = HashMap::new();
    map_methods.insert(
        "keys",
        (Rc::new(move |args| keys_fn(args))) as NativeFunction,
    );
    map_methods.insert(
        "values",
        (Rc::new(move |args| values_fn(args))) as NativeFunction,
    );
    map_methods.insert("len", (Rc::new(move |args| len_fn(args))) as NativeFunction);
    methods.insert("map", map_methods);

    methods
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
pub fn print_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        writeln!(io::stdout()).map_err(|e| e.to_string())?;
    } else {
        let output: Vec<String> = args.iter().map(|arg| format!("{}", arg)).collect();
        writeln!(io::stdout(), "{}", output.join(" ")).map_err(|e| e.to_string())?;
    }
    Ok(Value::Nil)
}

// Native len method (polymorphic, but called as a method)
pub fn len_fn(args: Vec<Value>) -> Result<Value, String> {
    // Expect `self` (the string/list/map) as the first argument, and no other arguments.
    if args.len() != 1 {
        return Err(format!(
            "len() expected 1 argument (self), but got {}",
            args.len()
        ));
    }

    let len = match &args[0] {
        Value::String(s) => s.chars().count(),
        Value::List(l) => l.len(),
        Value::Map(m) => m.len(),
        other => {
            return Err(format!(
                "len() method does not support type '{}'.",
                other.type_of()
            ));
        }
    };

    Ok(Value::Number(len as f64))
}

// Native string trim method
pub fn str_trim_fn(args: Vec<Value>) -> Result<Value, String> {
    // Expect `self` (the string) as the first argument, and no other arguments.
    if args.len() != 1 {
        return Err(format!(
            "trim() expected 1 argument (self), but got {}",
            args.len()
        ));
    }

    match &args[0] {
        Value::String(s) => Ok(Value::String(s.trim().to_string())),
        other => Err(format!(
            "trim() method expected a string, but got type '{}'.",
            other.type_of()
        )),
    }
}

// Native type function
pub fn type_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "type() expected 1 argument, but got {}",
            args.len()
        ));
    }

    let type_str = args[0].type_of();
    Ok(Value::String(type_str.to_string()))
}

// Native str conversion function
pub fn str_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("str() expected 1 argument, but got {}", args.len()));
    }

    Ok(Value::String(format!("{}", args[0])))
}

// Native num conversion function
/// Converts a value to a number.
///
/// - Number values are returned as-is.
/// - String values are parsed to f64; if parsing fails, returns `Value::Nil`.
/// - Boolean `true` becomes 1.0, `false` becomes 0.0.
/// - `Nil` becomes 0.0.
/// - For any other type, returns `Value::Nil`.
pub fn num_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("num() expected 1 argument, but got {}", args.len()));
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::String(s) => {
            match s.trim().parse::<f64>() {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Ok(Value::Nil), // If string parsing fails, return Nil
            }
        }
        Value::Boolean(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
        Value::Nil => Ok(Value::Number(0.0)),
        _other => {
            // For other types (e.g., List, Map, Function), return Nil as they cannot be coerced to a number.
            Ok(Value::Nil)
        }
    }
}

// Native input function
pub fn input_fn(args: Vec<Value>) -> Result<Value, String> {
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

    Ok(Value::String(
        line.trim_end_matches(&['\n', '\r'][..]).to_string(),
    ))
}

// Native bool conversion function
pub fn bool_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "bool() expected 1 argument, but got {}",
            args.len()
        ));
    }

    Ok(Value::Boolean(args[0].is_truthy()))
}

// Native keys method
pub fn keys_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "keys() expected 1 argument (self), but got {}.",
            args.len()
        ));
    }

    match &args[0] {
        Value::Map(m) => {
            let keys: Vec<Value> = m.keys().map(|k| k.clone()).collect();
            Ok(Value::List(Rc::new(keys)))
        }
        other => Err(format!(
            "keys() method expected a map, but got type '{}'.",
            other.type_of()
        )),
    }
}

// Native values method
pub fn values_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "values() expected 1 argument (self), but got {}.",
            args.len()
        ));
    }

    match &args[0] {
        Value::Map(m) => {
            let values: Vec<Value> = m.values().cloned().collect();
            Ok(Value::List(Rc::new(values)))
        }
        other => Err(format!(
            "values() method expected a map, but got type '{}'.",
            other.type_of()
        )),
    }
}
