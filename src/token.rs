#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // --- 字面量 (Literals) ---
    Identifier(String),
    Number(f64),
    StringLiteral(String),

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