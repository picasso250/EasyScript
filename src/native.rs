use crate::value::Value;
// use crate::value::FunctionObject; // 暂时用不到
// use crate::environment::EnvironmentRef; // 暂时用不到，但为了未来其他 NativeFunction 可能需要环境而引入

// 所有的 NativeFunction 都应符合这个签名
// pub type NativeFunction = fn(Vec<Value>) -> Result<Value, String>;

// Native print function
pub fn print_fn(args: Vec<Value>) -> Result<Value, String> {
    if args.is_empty() {
        println!(); // 打印空行
    } else {
        // 简单地打印所有参数，用空格分隔
        let output: Vec<String> = args.iter().map(|arg| format!("{}", arg)).collect();
        println!("{}", output.join(" "));
    }
    Ok(Value::Nil) // print 函数通常返回 nil
}