// 新增 Literal 枚举来存储字符串和数字的实际值
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // --- 字面量 (Literals) ---
    // 标识符现在携带其名称
    Identifier(String),
    // Literal 变体携带具体的字面量值 (数字、字符串)
    Literal(Literal),

    // --- 关键字 (Keywords) ---
    KeywordIf,
    KeywordElse,
    KeywordFor,
    KeywordFun,
    KeywordIn,
    KeywordTrue,
    KeywordFalse,
    KeywordNil,

    // --- 运算符 (Operators) ---
    // 算术
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %

    // 位移/位运算
    ShiftLeft,  // <<
    ShiftRight, // >>
    Ampersand,  // &
    Pipe,       // |
    Caret,      // ^

    // 关系/相等
    Less,       // <
    LessEqual,  // <=
    Greater,    // >
    GreaterEqual,// >=
    EqualEqual, // ==
    BangEqual,  // !=

    // 逻辑
    And,        // &&
    Or,         // ||

    // --- 标点符号 (Punctuation) / 单个字符 ---
    LeftParen,  // (
    RightParen, // )
    LeftBracket,// [
    RightBracket,// ]
    LeftBrace,  // {
    RightBrace, // }
    Comma,      // ,
    Dot,        // .
    Colon,      // :
    Semicolon,  // ;

    // --- 赋值 (Assignment) ---
    Equal,      // =

    // --- 文件结束 ---
    Eof,
}