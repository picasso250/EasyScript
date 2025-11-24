// main.rs now acts as a consumer of the `easyscript_rs` library
use easyscript_rs::{Lexer, Parser, Interpreter};
use easyscript_rs::error::EasyScriptError; // <-- 新增

fn main() {
    println!("EasyScript 解释器启动...");
    
    // Using a simple numeric literal for a clean state
    let source = r#"
        let a = 10;
        let b = a + 20;
        if b > 25 {
            b + 5;
        } else {
            b - 5;
        };
    "#; // 使用一个更复杂的例子来测试

    println!("\n--- 源代码 ---\n{}", source);

    // 1. 词法分析 (Lexer)
    let tokens = match Lexer::new(source).scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("\n--- 词法错误 ---");
            eprintln!("{}", e);
            return;
        }
    };
    
    // 2. 语法分析 (Parser)
    let ast_root = match Parser::new(tokens).parse() {
        Ok(ast) => ast,
        Err(e) => {
            eprintln!("\n--- 语法错误 ---");
            eprintln!("{}", e);
            return;
        }
    };

    // 3. 求值/解释 (Interpreter)
    println!("\n--- 解释器执行中 ---");
    match Interpreter::new().run(&ast_root) {
        Ok(value) => {
            println!("\n--- 执行结果 ---");
            println!("{}", value); // 使用 Display trait，更友好
        }
        Err(e) => {
            eprintln!("\n--- 运行时错误 ---");
            eprintln!("{}", e);
        }
    }
}