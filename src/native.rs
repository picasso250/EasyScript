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