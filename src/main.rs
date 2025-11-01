// 声明项目中的模块，这样其他文件就能通过 `crate::module_name::...` 访问
mod value;
mod token;
mod ast;

// 我们可以引入一些常用的类型，以便在 main 函数中使用
use value::Value;
use ast::{Expression, Block};

fn main() {
    println!("EasyScript 解释器正在启动...");
    
    // 我们可以测试一下之前定义的结构是否可以正常使用
    let test_expression = Expression::Literal(ast::LiteralValue::Number(42.0));
    
    println!("测试 AST 节点: {:?}", test_expression);
    
    // TODO: 在这里我们将开始调用词法分析器和语法分析器
    // 例如：
    // let source = "a = 1 + 2;";
    // let tokens = token::Lexer::new(source).scan_tokens();
    // let ast = ast::Parser::new(tokens).parse();
    // let result = value::Interpreter::new().interpret(ast);
}