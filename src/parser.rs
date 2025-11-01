use crate::ast::{AccessType, BinaryOperator, Block, Expression, LValue, LiteralValue, UnaryOperator};
use crate::token::{Literal, Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0, errors: Vec::new() }
    }

    pub fn parse(mut self) -> (Block, Vec<String>) {
        let mut expressions = Vec::new();
        while !self.is_at_end() {
            match self.expression() {
                Ok(expr) => expressions.push(expr),
                Err(e) => {
                    self.errors.push(e);
                    self.synchronize();
                }
            }
            // 允许多个分号或最后一个表达式后没有分号
            while self.match_tokens(&[Token::Semicolon]) {
                // consume all semicolons
            }
        }
        (Block { expressions }, self.errors)
    }

    // --- 语法规则实现 ---

    // Expression ::= AssignmentExpression | TermExpression
    fn expression(&mut self) -> Result<Expression, String> {
        self.assignment()
    }

    // AssignmentExpression ::= ( CallExpression "." Identifier | CallExpression "[" Expression "]" ) "=" Assignment | TermExpression
    fn assignment(&mut self) -> Result<Expression, String> {
        let expr = self.term()?;

        if self.match_tokens(&[Token::Equal]) {
            // 将左侧的 Expression 转换为 LValue
            let lvalue = match expr {
                Expression::Identifier(name) => LValue::Identifier(name),
                Expression::Accessor { target, access } => match access {
                    AccessType::Index(key) => LValue::IndexAccess { target, key },
                    AccessType::Dot(property_name) => LValue::DotAccess { target, property_name },
                },
                _ => return Err(format!("Invalid assignment target: {:?}", expr)),
            };
            let value = self.assignment()?; // 赋值是右结合的
            return Ok(Expression::Assignment { lvalue, value: Box::new(value) });
        }
        Ok(expr)
    }
    
    // TermExpression ::= FactorExpression { ( "+" | "-" | "<" | "<=" | ... ) FactorExpression }
    fn term(&mut self) -> Result<Expression, String> {
        let mut expr = self.factor()?;
        while let Some(op) = self.match_term_op() {
            let right = self.factor()?;
            expr = Expression::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }
    
    // FactorExpression ::= UnaryExpression { ( "*" | "/" | "%" | "<<" | ... ) UnaryExpression }
    fn factor(&mut self) -> Result<Expression, String> {
        let mut expr = self.unary()?;
        while let Some(op) = self.match_factor_op() {
            let right = self.unary()?;
            expr = Expression::Binary { left: Box::new(expr), op, right: Box::new(right) };
        }
        Ok(expr)
    }

    // UnaryExpression ::= "-" UnaryExpression | CallAndAccessExpression
    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[Token::Minus]) {
            let op = UnaryOperator::Negate;
            let expr = self.unary()?; // 递归调用 unary
            return Ok(Expression::Unary { op, expr: Box::new(expr) });
        }
        self.call_and_access()
    }

    // 新增: 处理链式调用和访问
    // CallAndAccessExpression ::= PrimaryExpression { "(" Arguments? ")" | "[" Expression "]" | "." Identifier }
    fn call_and_access(&mut self) -> Result<Expression, String> {
        let mut expr = self.primary()?;

        loop {
            if self.match_tokens(&[Token::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_tokens(&[Token::LeftBracket]) {
                let key = self.expression()?;
                self.consume(&Token::RightBracket, "Expect ']' after index.")?;
                expr = Expression::Accessor {
                    target: Box::new(expr),
                    access: AccessType::Index(Box::new(key)),
                };
            } else if self.match_tokens(&[Token::Dot]) {
                let property_name = self.consume_identifier("Expect property name after '.'.")?;
                expr = Expression::Accessor {
                    target: Box::new(expr),
                    access: AccessType::Dot(property_name),
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }


    // PrimaryExpression ::= Literal | Identifier | "(" Expression ")" | ListLiteral | MapLiteral
    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[Token::KeywordFalse]) { return Ok(Expression::Literal(LiteralValue::Boolean(false))); }
        if self.match_tokens(&[Token::KeywordTrue]) { return Ok(Expression::Literal(LiteralValue::Boolean(true))); }
        if self.match_tokens(&[Token::KeywordNil]) { return Ok(Expression::Literal(LiteralValue::Nil)); }

        if let Token::Literal(literal) = self.peek() {
            let owned_literal = literal.clone();
            self.advance();
            return Ok(match owned_literal {
                Literal::Number(n) => Expression::Literal(LiteralValue::Number(n)),
                Literal::String(s) => Expression::Literal(LiteralValue::String(s)),
            });
        }
        
        if let Token::Identifier(name) = self.peek() {
            let owned_name = name.clone();
            self.advance();
            return Ok(Expression::Identifier(owned_name));
        }

        if self.match_tokens(&[Token::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&Token::RightParen, "Expect ')' after expression.")?;
            return Ok(expr);
        }
        
        if self.match_tokens(&[Token::LeftBracket]) {
            return self.list_literal();
        }

        if self.match_tokens(&[Token::LeftBrace]) {
            return self.map_literal();
        }

        Err(format!("Expected expression, found {:?}", self.peek()))
    }

    // --- 辅助方法 ---
    
    // 解析列表字面量
    fn list_literal(&mut self) -> Result<Expression, String> {
        let mut elements = Vec::new();
        if !self.check(&Token::RightBracket) {
            loop {
                elements.push(self.expression()?);
                if !self.match_tokens(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(&Token::RightBracket, "Expect ']' after list elements.")?;
        Ok(Expression::Literal(LiteralValue::List(elements)))
    }

    // 解析字典字面量
    fn map_literal(&mut self) -> Result<Expression, String> {
        let mut pairs = Vec::new();
        if !self.check(&Token::RightBrace) {
            loop {
                let key = self.expression()?;
                self.consume(&Token::Colon, "Expect ':' after map key.")?;
                let value = self.expression()?;
                pairs.push((key, value));

                if !self.match_tokens(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(&Token::RightBrace, "Expect '}' after map entries.")?;
        Ok(Expression::Literal(LiteralValue::Map(pairs)))
    }

    // 完成函数调用解析
    fn finish_call(&mut self, callee: Expression) -> Result<Expression, String> {
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
                // 允许 for x in a { ... }; print(x) 这种, 所以这里需要检查, 是否超过255个参数.
                // if args.len() >= 255 {
                //     return Err("Cannot have more than 255 arguments.".to_string());
                // }
                args.push(self.expression()?);
                if !self.match_tokens(&[Token::Comma]) {
                    break;
                }
            }
        }
        self.consume(&Token::RightParen, "Expect ')' after arguments.")?;
        Ok(Expression::Call { callee: Box::new(callee), args })
    }

    fn match_term_op(&mut self) -> Option<BinaryOperator> {
        let op = match self.peek() {
            Token::Plus => Some(BinaryOperator::Add),
            Token::Minus => Some(BinaryOperator::Sub),
            Token::Less => Some(BinaryOperator::Lt),
            Token::LessEqual => Some(BinaryOperator::Lte),
            Token::Greater => Some(BinaryOperator::Gt),
            Token::GreaterEqual => Some(BinaryOperator::Gte),
            Token::EqualEqual => Some(BinaryOperator::Eq),
            Token::BangEqual => Some(BinaryOperator::Neq),
            Token::And => Some(BinaryOperator::And),
            Token::Or => Some(BinaryOperator::Or),
            _ => None,
        };
        if op.is_some() { self.advance(); }
        op
    }

    fn match_factor_op(&mut self) -> Option<BinaryOperator> {
        let op = match self.peek() {
            Token::Star => Some(BinaryOperator::Mul),
            Token::Slash => Some(BinaryOperator::Div),
            Token::Percent => Some(BinaryOperator::Mod),
            Token::ShiftLeft => Some(BinaryOperator::ShL),
            Token::ShiftRight => Some(BinaryOperator::ShR),
            Token::Ampersand => Some(BinaryOperator::BitAnd),
            Token::Pipe => Some(BinaryOperator::BitOr),
            Token::Caret => Some(BinaryOperator::BitXor),
            _ => None,
        };
        if op.is_some() { self.advance(); }
        op
    }
    
    fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        if let Token::Identifier(name) = self.peek() {
            let owned_name = name.clone();
            self.advance();
            Ok(owned_name)
        } else {
            Err(format!("{} Found {:?}", message, self.peek()))
        }
    }

    fn match_tokens(&mut self, types: &[Token]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }
    
    fn consume(&mut self, token_type: &Token, message: &str) -> Result<&Token, String> {
        if self.check(token_type) {
            self.advance();
            Ok(self.previous())
        } else {
            Err(format!("{} Found {:?}", message, self.peek()))
        }
    }

    fn check(&self, token_type: &Token) -> bool {
        !self.is_at_end() && std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool { matches!(self.peek(), &Token::Eof) }
    fn peek(&self) -> &Token { &self.tokens[self.current] }
    fn previous(&self) -> &Token { &self.tokens[self.current - 1] }

    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if matches!(self.previous(), &Token::Semicolon) { return; }
            match self.peek() {
                Token::KeywordFun | Token::KeywordFor | Token::KeywordIf => return,
                _ => { self.advance(); }
            };
        }
    }
}