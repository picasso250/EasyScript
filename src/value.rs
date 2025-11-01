use std::collections::HashMap;
use std::fmt;
use std::sync::Arc; // 使用 Arc 来进行引用计数，便于共享

// 定义函数签名，用于表示原生的 Rust 函数
// (Vec<Value>) -> Result<Value, String> 表示接受一列参数，返回一个 Result
pub type NativeFunction = fn(Vec<Value>) -> Result<Value, String>;

// 运行时函数对象，可以是用户定义的（Closure）或原生（Native）
#[derive(Clone)]
pub enum FunctionObject {
    // Rust 原生函数
    Native(NativeFunction),
    // EasyScript 用户函数 (暂时简化，不包含环境/闭包信息)
    User {
        params: Vec<String>,
        body: Arc<super::ast::Block>, // 使用 Arc 共享 Block
        // TODO: Environment/Closure
    },
}

impl fmt::Debug for FunctionObject {
    // 简化 Debug 输出
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FunctionObject::Native(_) => write!(f, "<native fn>"),
            FunctionObject::User { params, .. } => write!(f, "<fn ({})>", params.join(", ")),
        }
    }
}

// EasyScript 核心运行时值类型，一切皆 Value。
#[derive(Clone, Debug)]
pub enum Value {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    // List 是可变集合，使用 Arc<Mutex<Vec<Value>>> 实现 (这里先简化为 Vec)
    List(Arc<Vec<Value>>), 
    // Map/Dict/Object，使用 Arc<Mutex<HashMap<Value, Value>>> 实现 (这里先简化为 HashMap)
    Map(Arc<HashMap<Value, Value>>),
    Function(FunctionObject),
}

// 为了能作为 Map 的键，Value 必须实现 Eq 和 Hash (这里暂时只实现部分类型)
// 生产级实现中，List 和 Map 的 Hash/Eq 实现会更复杂，这里先仅为编译通过。
impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Nil, Value::Nil) => true,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            // 默认 List/Map 比较引用地址，复杂值比较暂不实现
            _ => false,
        }
    }
}

impl Eq for Value {}

use std::hash::{Hash, Hasher};
impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Nil => 0.hash(state),
            Value::Boolean(b) => b.hash(state),
            // 注意：浮点数 Hash 需要处理 NaN/Inf，这里使用简单的比特表示
            Value::Number(n) => n.to_bits().hash(state), 
            Value::String(s) => s.hash(state),
            // List/Map 默认不可作为 Hash Key，若强行要用，需要特殊的标识符或引用 Hash
            _ => { /* 不可作为 Key */ }
        }
    }
}