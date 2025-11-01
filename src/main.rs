mod value;
mod token;
mod ast;
mod lexer; // 引入新的 Lexer 模块

// 引入 lazy_static 宏
#[macro_use]
extern crate lazy_static;

// ... (保留原有的 use 声明)
use lexer::Lexer; // 引入 Lexer

fn main() {
    println!("EasyScript 解释器正在启动...");
    
    // 测试代码片段
    let source = r#"
        // 这是一个注释
        a = if x > 10 { 
            42.5 + true 
        } else {
            "hello";
        };
        
        arr[0] = fun(z) { z * 2 };
        b = 10 << 2;
    "#;
    
    // 实例化并运行 Lexer
    let lexer = Lexer::new(source);
    let (tokens, errors) = lexer.scan_tokens();

    // 打印结果
    if !errors.is_empty() {
        println!("\n--- Lexer 错误 ---");
        for error in errors {
            println!("{}", error);
        }
        return; // 如果有词法错误，停止继续
    }

    println!("\n--- Lexer 输出 Token ({}) ---", tokens.len());
    for token in tokens.iter().take(20) { // 仅显示前 20 个 Token
        println!("{:?}", token);
    }
    
    // TODO: 在这里我们将开始调用语法分析器 (Parser)
    
    // println!("测试 AST 节点: {:?}", test_expression); // 移除测试 AST
}