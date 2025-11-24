// main.rs now acts as a consumer of the `easyscript_rs` library
use easyscript_rs::{Lexer, Parser, Interpreter};

fn main() {
    println!("EasyScript 解释器启动...");
    
    // 使用一个简单的数字字面量来测试完整的流程
    let source = r#"
        42.0
    "#;
    
    println!("\n--- 源代码 ---\n{}", source);

    // 1. 词法分析 (Lexer)
    let lexer = Lexer::new(source);
    let (tokens, lexer_errors) = lexer.scan_tokens();

    if !lexer_errors.is_empty() {
        println!("\n--- Lexer 错误 ---");
        lexer_errors.iter().for_each(|e| println!("{}", e));
        return;
    }

    // 2. 语法分析 (Parser)
    let parser = Parser::new(tokens);
    let (ast_root, parser_errors) = parser.parse();

    if !parser_errors.is_empty() {
        println!("\n--- Parser 错误 ---");
        parser_errors.iter().for_each(|e| println!("{}", e));
        return;
    }

    // 3. 求值/解释 (Interpreter)
    println!("\n--- Interpreter 执行中 ---");
    let mut interpreter = Interpreter::new();
    match interpreter.run(&ast_root) {
        Ok(value) => {
            println!("\n--- 执行结果 ---");
            println!("{:?}", value);
        }
        Err(e) => {
            println!("\n--- Runtime 错误 ---");
            println!("{}", e);
        }
    }
}