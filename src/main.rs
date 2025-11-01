mod value;
mod token;
mod ast;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

fn main() {
    println!("EasyScript 解释器启动...");
    
    // 使用更复杂的测试代码来验证优先级和各种运算符
    let source = r#"
        // EBNF: Term -> Factor { TermOp Factor }
        // EBNF: Factor -> Unary { FactorOp Unary }
        
        result = -5 * (10 + 2) > 50 && true || 1 << 2 == 4;
        
        // 预期 AST 结构:
        // (( ((-5 * (10 + 2)) > 50) && true ) || ( (1 << 2) == 4 ))
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

    println!("\n--- Parser 输出 AST ---");
    println!("{:#?}", ast_root);

    // 3. TODO: 求值/解释 (Interpreter)
}