use std::fmt;

// 错误的位置信息
#[derive(Debug, Clone, PartialEq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize, // 0-based column index
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {} column {}", self.line, self.column)
    }
}

// 统一的错误类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum EasyScriptError {
    LexerError {
        message: String,
        location: Option<SourceLocation>,
    },
    ParserError {
        message: String,
        location: Option<SourceLocation>,
    },
    RuntimeError {
        message: String,
        location: Option<SourceLocation>,
    },
}

impl fmt::Display for EasyScriptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EasyScriptError::LexerError { message, location } => {
                if let Some(loc) = location {
                    write!(f, "[Lexer Error at {}]: {}", loc, message)
                } else {
                    write!(f, "[Lexer Error]: {}", message)
                }
            }
            EasyScriptError::ParserError { message, location } => {
                if let Some(loc) = location {
                    write!(f, "[Parser Error at {}]: {}", loc, message)
                } else {
                    write!(f, "[Parser Error]: {}", message)
                }
            }
            EasyScriptError::RuntimeError { message, location } => {
                if let Some(loc) = location {
                    write!(f, "[Runtime Error at {}]: {}", loc, message)
                } else {
                    write!(f, "[Runtime Error]: {}", message)
                }
            }
        }
    }
}

// 帮助将 String 转换为 RuntimeError
impl From<String> for EasyScriptError {
    fn from(message: String) -> Self {
        EasyScriptError::RuntimeError {
            message,
            location: None, // 默认没有位置信息
        }
    }
}
