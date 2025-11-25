use crate::value::Value;
use std::rc::Rc;

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
            return Err(format!("len() does not support type '{}'", other.type_of()));
        }
    };

    Ok(Value::Number(len as f64))
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

// Native string conversion function
pub fn str_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "string() expected 1 argument, but got {}",
            args.len()
        ));
    }

    Ok(Value::String(format!("{}", args[0])))
}

// Native number conversion function
pub fn num_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        // 参数数量错误仍然抛出运行时错误，因为这是函数用法错误
        return Err(format!(
            "number() expected 1 argument, but got {}",
            args.len()
        ));
    }

    match &args[0] {
        Value::Number(n) => Ok(Value::Number(*n)),
        Value::String(s) => {
            // 尝试解析，失败则返回 nil
            match s.trim().parse::<f64>() {
                Ok(n) => Ok(Value::Number(n)),
                Err(_) => Ok(Value::Nil), // 转换失败时返回 nil
            }
        }
        Value::Boolean(b) => Ok(Value::Number(if *b { 1.0 } else { 0.0 })),
        Value::Nil => Ok(Value::Number(0.0)),
        other => {
            // 不支持的类型返回 nil
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

    // Print prompt if provided
    if let Some(prompt_value) = args.get(0) {
        // 使用 print! 而不是 println!，因为用户可能希望在同一行输入
        print!("{}", prompt_value);
        // 确保提示符被立即刷新到控制台
        use std::io::Write;
        std::io::stdout().flush().map_err(|e| e.to_string())?;
    }

    // Read a line from stdin
    std::io::stdin()
        .read_line(&mut line)
        .map_err(|e| e.to_string())?;

    // Remove trailing newline (platform dependent: \n or \r\n)
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

// Native keys function
pub fn keys_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "keys() expected 1 argument, but got {}",
            args.len()
        ));
    }

    match &args[0] {
        Value::Map(m) => {
            let keys: Vec<Value> = m.keys().cloned().collect();
            Ok(Value::List(Rc::new(keys)))
        }
        other => Err(format!(
            "keys() expected a map, but got type '{}'.",
            other.type_of()
        )),
    }
}

// Native values function
pub fn values_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!(
            "values() expected 1 argument, but got {}",
            args.len()
        ));
    }

    match &args[0] {
        Value::Map(m) => {
            let values: Vec<Value> = m.values().cloned().collect();
            Ok(Value::List(Rc::new(values)))
        }
        other => Err(format!(
            "values() expected a map, but got type '{}'.",
            other.type_of()
        )),
    }
}
