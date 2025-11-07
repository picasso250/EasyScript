use std::sync::Arc;
use crate::value::FunctionObject;

// 核心的抽象语法树节点：一切皆 Expression
#[derive(Debug, Clone)]
pub enum Expression {
    // ----------------------------------------------------
    // I. 基础表达式 (Basic Primitives)
    // ----------------------------------------------------
    Literal(LiteralValue), // 字面量 (Number, String, True, False, Nil)
    Identifier(String),    // 变量引用
    Block(Block),          // 新增: 表达式块 { expr1; expr2 }

    // ----------------------------------------------------
    // II. 运算表达式 (Operations)
    // ----------------------------------------------------
    // 一元运算
    Unary {
        op: UnaryOperator,
        expr: Box<Expression>,
    },
    // 二元运算
    Binary {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    
    // ----------------------------------------------------
    // III. 控制流 / 函数 / 赋值
    // ----------------------------------------------------
    // 函数定义 (FunctionDefinition)
    FunctionDef(FunctionObject), // 直接存储 FunctionObject，简化 AST
    
    // 赋值 (AssignmentExpression)
    Assignment {
        lvalue: LValue, 
        value: Box<Expression>,
    },
    
    // If Expression
    If {
        condition: Box<Expression>,
        then_block: Block,
        // else_branch 可以是另一个 IfExpression 或一个 BlockExpression
        else_branch: Option<Box<Expression>>, 
    },
    
    // For Expression (列表生成/map)
    For {
        identifier: String,          // for x
        iterable: Box<Expression>,   // in collection
        body: Block,
    },

    // ----------------------------------------------------
    // IV. 访问与调用 (Access & Call)
    // ----------------------------------------------------
    // 函数调用 (FunctionCall)
    Call {
        callee: Box<Expression>,         // 被调用的函数表达式 (e.g., f, obj.method)
        args: Vec<Expression>,           // 参数列表
    },
    
    // 列表/字典/属性访问 (Accessor 规则)
    Accessor {
        target: Box<Expression>,       // 目标对象
        access: AccessType,            // 访问类型 (Index or Dot)
    },
}

// 辅助结构：字面量
#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    List(Vec<Expression>), // 列表字面量 [1, 2+3]
    Map(Vec<(Expression, Expression)>), // 字典字面量 {k: v, ...}
}

// 辅助结构：表达式块
#[derive(Debug, Clone)]
pub struct Block {
    pub expressions: Vec<Expression>,
}

// 辅助结构：赋值左值 (LValue)
#[derive(Debug, Clone)]
pub enum LValue {
    Identifier(String), // 变量名
    IndexAccess {       // 列表/字典索引赋值 e.g. arr[0] = 1
        target: Box<Expression>,
        key: Box<Expression>,
    },
    DotAccess {         // 属性点访问赋值 e.g. obj.prop = 1
        target: Box<Expression>,
        property_name: String, // '.' 后的标识符
    },
}

// 辅助结构：访问类型
#[derive(Debug, Clone)]
pub enum AccessType {
    Index(Box<Expression>),      // 索引访问 [TermExpression]
    Dot(String),                 // 点访问 .Identifier
}


// --- 运算符枚举 (可与 Token 对应) ---
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum UnaryOperator {
    Negate, // - (一元负号)
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum BinaryOperator {
    // 算术
    Add, Sub, Mul, Div, Mod, 
    // 位运算
    ShL, ShR, BitAnd, BitOr, BitXor,
    // 关系/逻辑
    Lt, Lte, Gt, Gte, Eq, Neq, And, Or,
}