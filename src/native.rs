use crate::value::Value;

// Native print function
pub fn print_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        println!();
    } else {
        let output: Vec<String> = args.iter().map(|arg| format!("{}", arg)).collect();
        println!("{}", output.join(" "));
    }
    Ok(Value::Nil)
}

// Native len function
pub fn len_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("len() expected 1 argument, but got {}", args.len()));
    }

    let len = match &args[0] {
        Value::String(s) => s.chars().count(),
        Value::List(l) => l.len(),
        Value::Map(m) => m.len(),
        other => {
            return Err(format!(
                "len() does not support type '{}'",
                other.type_of()
            ));
        }
    };

    Ok(Value::Number(len as f64))
}

// Native type function
pub fn type_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("type() expected 1 argument, but got {}", args.len()));
    }

    let type_str = args[0].type_of();
    Ok(Value::String(type_str.to_string()))
}

// Native string conversion function
pub fn string_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("string() expected 1 argument, but got {}", args.len()));
    }

    Ok(Value::String(format!("{}", args[0])))
}

// Native number conversion function
pub fn number_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("number() expected 1 argument, but got {}", args.len()));
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::String(s) => {
            match s.trim().parse::<f64>() {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Err(format!("Could not convert string '{}' to number.", s)),
            }
        }
        Value::Boolean(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
        Value::Nil => Ok(Value::Number(0.0)),
        other => Err(format!(
            "number() does not support converting type '{}'.",
            other.type_of()
        )),
    }
}