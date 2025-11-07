use crate::ast::{AccessType, BinaryOperator, Block, Expression, LValue, LiteralValue, UnaryOperator};
use crate::token::{Literal, Token};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        // --- FIX: Corrected Vec::new() syntax ---
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

    // Expression ::= IfExpression | ForExpression | FunctionDefinition | AssignmentExpression
    fn expression(&mut self) -> Result<Expression, String> {
        if self.match_tokens(&[Token::KeywordIf]) {
            return self.if_expression();
        }
        // TODO: Add `for` and `fun` keyword checks here in the future
        
        self.assignment()
    }

    fn if_expression(&mut self) -> Result<Expression, String> {
        let condition = self.expression()?;
        let then_block = self.block()?;

        let mut else_branch = None;
        if self.match_tokens(&[Token::KeywordElse]) {
            if self.match_tokens(&[Token::KeywordIf]) {
                // 处理 else-if 链 (if_expression is also an Expression)
                let else_if_expr = self.if_expression()?;
                else_branch = Some(Box::new(else_if_expr));
            } else {
                // 处理 else 块
                let else_block = self.block()?;
                // Wrap the block in an Expression::Block variant
                else_branch = Some(Box::new(Expression::Block(else_block)));
            }
        }

        Ok(Expression::If { condition: Box::new(condition), then_block, else_branch })
    }

    // AssignmentExpression ::= LValue "=" Assignment | TermExpression
    fn assignment(&mut self) -> Result<Expression, String> {
        let expr = self.term()?;

        if self.match_tokens(&[Token::Equal]) {
            let value = self.assignment()?; // 赋值是右结合的
            
            // 将左侧的 Expression 转换为 LValue
            return match expr {
                Expression::Identifier(name) => Ok(Expression::Assignment { 
                    lvalue: LValue::Identifier(name), 
                    value: Box::new(value) 
                }),
                Expression::Accessor { target, access } => match access {
                    AccessType::Index(key) => Ok(Expression::Assignment {
                        lvalue: LValue::IndexAccess { target, key },
                        value: Box::new(value),
                    }),
                    AccessType::Dot(property_name) => Ok(Expression::Assignment {
                        lvalue: LValue::DotAccess { target, property_name },
                        value: Box::new(value),
                    }),
                },
                _ => Err(format!("Invalid assignment target: {:?}", expr)),
            }
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

    // PrimaryExpression ::= Literal | Identifier | "(" Expression ")" | ListLiteral | MapLiteral | BlockExpression
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
            // Check for map first, then fall back to block.
            // A map is distinguished by a key followed by a colon.
            // An empty `{}` will be parsed as an empty map.
            // A block is `{ <expr> ... }`
            if self.check_next(&Token::Colon) {
                 return self.map_literal();
            } else if self.check(&Token::RightBrace) { // Empty {} is a map
                 return self.map_literal();
            } else { // It's a block expression
                 let block = self.block()?;
                 return Ok(Expression::Block(block));
            }
        }

        Err(format!("Expected expression, found {:?}", self.peek()))
    }

    // --- 辅助方法 ---
    
    // Parse a block `{...}`
    fn block(&mut self) -> Result<Block, String> {
        let mut expressions = Vec::new();
        
        // consume opening brace which was already matched
        self.consume(&Token::LeftBrace, "Expect '{' to start a block.")?;

        while !self.check(&Token::RightBrace) && !self.is_at_end() {
            expressions.push(self.expression()?);
            // Eat trailing semicolons
            while self.match_tokens(&[Token::Semicolon]) {}
        }

        self.consume(&Token::RightBrace, "Expect '}' after block.")?;
        Ok(Block { expressions })
    }
    
    // Parse a list literal `[...]`
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

    // Parse a map literal `{...}`
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

    // Finish parsing a function call
    fn finish_call(&mut self, callee: Expression) -> Result<Expression, String> {
        let mut args = Vec::new();
        if !self.check(&Token::RightParen) {
            loop {
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
            Ok(self.advance())
        } else {
            Err(format!("{} Found {:?}", message, self.peek()))
        }
    }

    fn check(&self, token_type: &Token) -> bool {
        !self.is_at_end() && std::mem::discriminant(self.peek()) == std::mem::discriminant(token_type)
    }

    fn check_next(&self, token_type: &Token) -> bool {
        if self.is_at_end() { return false; }
        if matches!(self.tokens.get(self.current + 1), Some(&Token::Eof)) { return false; }
        
        if let Some(next_token) = self.tokens.get(self.current + 1) {
             std::mem::discriminant(next_token) == std::mem::discriminant(token_type)
        } else {
            false
        }
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