mod value;
mod token;
mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    println!("EasyScript 解释器启动...");
    
    // 更新测试代码以包含 if-else 表达式
    let source = r#"
        // 这是一个综合测试用例
        
        // 列表和字典字面量
        my_list = [1, "two", true];
        my_map = { "key": my_list[0], "another": false };
        
        // 链式调用、访问和赋值
        my_map.new_prop = some_object.get_list(arg1)[0]();
        
        // 新增：If-Else 表达式测试
        a = 10;
        result = if a > 5 { 
            "greater"; 
        } else { 
            "smaller_or_equal";
        };
        
        // else-if 链式测试
        final_val = if a == 1 { 
            1; 
        } else if a > 100 {
            100;
        } else {
            -1;
        };

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
    
    // println!("\n--- Tokens ---");
    // for token in &tokens {
    //     println!("{:?}", token);
    // }

    // 2. 语法分析 (Parser)
    let parser = Parser::new(tokens);
    let (ast_root, parser_errors) = parser.parse();

    if !parser_errors.is_empty() {
        println!("\n--- Parser 错误 ---");
        parser_errors.iter().for_each(|e| println!("{}", e));
        return;
    }

    println!("\n--- Parser 输出 AST ---");
    println!("{:#?}", ast_root);

    // 3. TODO: 求值/解释 (Interpreter)
}