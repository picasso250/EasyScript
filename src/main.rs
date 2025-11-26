// main.rs now acts as a consumer of the `easyscript_rs` library
use easyscript_rs::{Interpreter, Lexer, Parser};
use std::env; // Added
use std::fs; // Added

fn main() {
    let args: Vec<String> = env::args().collect();

    let source = if args.len() == 2 {
        let file_path = &args[1];
        match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("错误: 无法读取文件 '{}': {}", file_path, e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("用法: {} <文件路径>", args[0]);
        eprintln!("  例如: {} examples/hello.es", args[0]);
        std::process::exit(1);
    };

    println!("EasyScript 解释器启动...");
    println!("\n--- 源代码 ---\n{}", source);

    // 1. 词法分析 (Lexer)
    let tokens = match Lexer::new(&source).scan_tokens() {
        // Changed to &source
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
