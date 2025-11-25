use crate::error::{EasyScriptError, SourceLocation};
use crate::token::{Literal, Token};
use std::collections::HashMap;

// 预定义的关键字查找表
lazy_static::lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Token> = {
        let mut m = HashMap::new();
        m.insert("if", Token::KeywordIf);
        m.insert("else", Token::KeywordElse);
        m.insert("for", Token::KeywordFor);
        m.insert("fun", Token::KeywordFun);
        m.insert("in", Token::KeywordIn);
        m.insert("true", Token::KeywordTrue);
        m.insert("false", Token::KeywordFalse);
        m.insert("nil", Token::KeywordNil);
        m.insert("let", Token::KeywordLet); // 添加这一行
        m
    };
}

// 词法分析器结构体
pub struct Lexer<'a> {
    source: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    start: usize,   // 当前 Token 的起始位置（字节索引）
    current: usize, // 当前处理到的位置（字节索引）
    line: usize,    // 当前 Token 的起始行号
    column: usize,  // 当前 Token 的起始列号 (字符索引)
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars().peekable(),
            start: 0,
            current: 0,
            line: 1,   // 初始行号为 1
            column: 1, // 初始列号为 1
            tokens: Vec::new(),
        }
    }
    // 核心方法：扫描所有 Token
    pub fn scan_tokens(mut self) -> Result<Vec<Token>, EasyScriptError> {
        // 返回 Result
        // 在 main.rs 中初始化 lazy_static
        let _ = &*KEYWORDS;

        while self.peek().is_some() {
            self.start = self.current;
            self.scan_token()?; // scan_token 现在返回 Result<()>
        }

        // 添加文件结束符
        self.tokens.push(Token::Eof);

        Ok(self.tokens) // 成功时返回 Token 列表
    }

    // ---------------------- 辅助方法 ----------------------

    // 消耗并返回当前字符，同时更新 current 索引、行号和列号
    fn advance(&mut self) -> Option<char> {
        let next_char = self.chars.next();
        if let Some(c) = next_char {
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            self.current += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    // 查看下一个字符，但不消耗它
    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    // 查看下下个字符，但不消耗它
    fn peek_next(&mut self) -> Option<char> {
        self.chars.clone().nth(1)
    }

    // 检查下一个字符是否匹配期望的字符，如果匹配则消耗并返回 true
    fn match_char(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(c) if c == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    // 获取当前 Token 对应的源代码片段
    fn substring(&self) -> &'a str {
        &self.source[self.start..self.current]
    }

    // 添加 Token
    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    // 报告词法错误，并返回 EasyScriptError
    fn error<T>(&self, message: &str, line: usize, column: usize) -> Result<T, EasyScriptError> {
        Err(EasyScriptError::LexerError {
            message: message.to_string(),
            location: Some(SourceLocation { line, column }),
        })
    }

    // ---------------------- Token 处理器 ----------------------

    // 处理字符串字面量
    fn handle_string(&mut self, line: usize, column: usize) -> Result<(), EasyScriptError> {
        // 查找下一个双引号
        while self.peek().map_or(false, |c| c != '"' && c != '\n') {
            self.advance();
        }

        if self.peek() != Some('"') {
            return self.error("Unterminated string.", line, column);
        }

        self.advance(); // 消耗闭合的双引号 "

        // 提取引号之间的内容 (从 start+1 到 current-1)
        let value_start = self.start + 1;
        let value_end = self.current - 1;
        let value = self.source[value_start..value_end].to_string();
        self.add_token(Token::Literal(Literal::String(value)));
        Ok(())
    }

    // 处理数字字面量 (整数和浮点数)
    fn handle_number(&mut self, line: usize, column: usize) -> Result<(), EasyScriptError> {
        // 整数部分
        while self.peek().map_or(false, |c| c.is_ascii_digit()) {
            self.advance();
        }

        // 小数部分
        if self.peek() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            self.advance(); // 消耗 '.'
            while self.peek().map_or(false, |c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        let num_str = self.substring();
        match num_str.parse::<f64>() {
            Ok(num) => self.add_token(Token::Literal(Literal::Number(num))),
            Err(_) => {
                return self.error(&format!("Invalid number format: {}", num_str), line, column)
            }
        }
        Ok(())
    }

    // 处理标识符和关键字
    fn handle_identifier(&mut self) -> Result<(), EasyScriptError> {
        // 返回 Result<()>
        while self
            .peek()
            .map_or(false, |c| c.is_ascii_alphanumeric() || c == '_')
        {
            self.advance();
        }

        let text = self.substring();

        // 检查是否是关键字
        let token = KEYWORDS.get(text).cloned().unwrap_or_else(|| {
            // 否则是用户定义的标识符
            Token::Identifier(text.to_string())
        });

        self.add_token(token);
        Ok(())
    }

    // ---------------------- 核心分发器 ----------------------

    fn scan_token(&mut self) -> Result<(), EasyScriptError> {
        // 返回 Result<()>
        let token_start_line = self.line; // 记录当前 token 的起始行
        let token_start_column = self.column; // 记录当前 token 的起始列

        let c = match self.advance() {
            Some(c) => c,
            None => return Ok(()), // 达到文件末尾
        };

        match c {
            // 单字符 Token
            '(' => self.add_token(Token::LeftParen),
            ')' => self.add_token(Token::RightParen),
            '{' => self.add_token(Token::LeftBrace),
            '}' => self.add_token(Token::RightBrace),
            '[' => self.add_token(Token::LeftBracket),
            ']' => self.add_token(Token::RightBracket),
            ',' => self.add_token(Token::Comma),
            '.' => self.add_token(Token::Dot),
            ':' => self.add_token(Token::Colon),
            ';' => self.add_token(Token::Semicolon),
            '+' => self.add_token(Token::Plus),
            '-' => self.add_token(Token::Minus), // 一元负号在 Parser 中处理
            '*' => self.add_token(Token::Star),
            '%' => self.add_token(Token::Percent),
            '^' => self.add_token(Token::Caret),

            // 可能是双字符 Token
            '=' => {
                let token = if self.match_char('=') {
                    Token::EqualEqual
                } else {
                    Token::Equal
                };
                self.add_token(token);
            }
            '!' => {
                let token = if self.match_char('=') {
                    Token::BangEqual
                } else {
                    return self.error(
                        "Unexpected character '!'",
                        token_start_line,
                        token_start_column,
                    );
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.match_char('=') {
                    Token::LessEqual
                } else if self.match_char('<') {
                    Token::ShiftLeft
                } else {
                    Token::Less
                };
                self.add_token(token);
            }
            '>' => {
                let token = if self.match_char('=') {
                    Token::GreaterEqual
                } else if self.match_char('>') {
                    Token::ShiftRight
                } else {
                    Token::Greater
                };
                self.add_token(token);
            }
            '&' => {
                let token = if self.match_char('&') {
                    Token::And
                } else {
                    Token::Ampersand
                };
                self.add_token(token);
            }
            '|' => {
                let token = if self.match_char('|') {
                    Token::Or
                } else {
                    Token::Pipe
                };
                self.add_token(token);
            }
            '/' => {
                if self.match_char('/') {
                    // 处理行注释：跳过直到行尾或文件结束
                    while self.peek().map_or(false, |c| c != '\n') {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    // TODO: 实现块注释 /* ... */
                    return self.error(
                        "Block comments are not fully implemented yet.",
                        token_start_line,
                        token_start_column,
                    );
                } else {
                    self.add_token(Token::Slash);
                }
            }

            // 忽略空白字符
            ' ' | '\r' | '\t' => {}
            '\n' => {} // advance 已经处理了行和列更新

            // 字符串字面量
            '"' => self.handle_string(token_start_line, token_start_column)?, // handle_string 现在返回 Result
            // 数字字面量 (0-9 或 .)
            c if c.is_ascii_digit() => self.handle_number(token_start_line, token_start_column)?, // handle_number 现在返回 Result

            // 标识符/关键字 (字母或下划线开头)
            c if c.is_ascii_alphabetic() || c == '_' => self.handle_identifier()?,

            // 未知字符
            _ => {
                return self.error(
                    &format!("Unexpected character: {}", c),
                    token_start_line,
                    token_start_column,
                )
            }
        }
        Ok(())
    }
}

// 别忘了在 Cargo.toml 中添加 lazy_static
// 由于我们不在 Coding Mode 修改 Cargo.toml，所以我们假设用户会自行添加或我们稍后补充。
// 鉴于这是一个关键依赖，我将提醒用户，并提供 Cargo.toml 的修改。
