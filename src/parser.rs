use crate::ast::{BinaryOperator, Block, Expression, LValue, LiteralValue, UnaryOperator};
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
            if self.match_tokens(&[Token::Semicolon]) {
                continue;
            }
        }
        (Block { expressions }, self.errors)
    }

    // --- 语法规则实现 ---

    // Expression ::= AssignmentExpression | TermExpression
    fn expression(&mut self) -> Result<Expression, String> {
        // 赋值是特殊的，因为它不是左结合的，并且左侧有特殊要求
        self.assignment()
    }

    // AssignmentExpression ::= Identifier "=" AssignmentExpression | TermExpression
    fn assignment(&mut self) -> Result<Expression, String> {
        let expr = self.term()?; // 解析左侧，现在是 term
        if self.match_tokens(&[Token::Equal]) {
            let lvalue = match expr {
                Expression::Identifier(name) => LValue::Identifier(name),
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

    // UnaryExpression ::= "-" UnaryExpression | PrimaryExpression
    fn unary(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[Token::Minus]) {
            let op = UnaryOperator::Negate;
            let expr = self.unary()?; // 递归调用 unary
            return Ok(Expression::Unary { op, expr: Box::new(expr) });
        }
        self.primary()
    }

    // PrimaryExpression ::= Literal | Identifier | "(" Expression ")"
    fn primary(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[Token::KeywordFalse]) {
            return Ok(Expression::Literal(LiteralValue::Boolean(false)));
        }
        if self.match_tokens(&[Token::KeywordTrue]) {
            return Ok(Expression::Literal(LiteralValue::Boolean(true)));
        }
        if self.match_tokens(&[Token::KeywordNil]) {
            return Ok(Expression::Literal(LiteralValue::Nil));
        }
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
        Err(format!("Expected expression, found {:?}", self.peek()))
    }

    // --- 辅助方法 ---
    
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
                _ => self.advance(),
            };
        }
    }
}