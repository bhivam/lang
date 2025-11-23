use crate::{BinaryOp, Expr, SyntaxError, Token, TokenType, UnaryOp};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, SyntaxError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, SyntaxError> {
        self.term()
    }

    fn term(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[TokenType::Plus, TokenType::Minus]) {
            let operator = match self.previous().token_type {
                TokenType::Plus => BinaryOp::Add,
                TokenType::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, SyntaxError> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[TokenType::Star, TokenType::Slash]) {
            let operator = match self.previous().token_type {
                TokenType::Star => BinaryOp::Mul,
                TokenType::Slash => BinaryOp::Div,
                _ => unreachable!(),
            };
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, SyntaxError> {
        if self.match_tokens(&[TokenType::Minus, TokenType::Bang]) {
            let operator = match self.previous().token_type {
                TokenType::Minus => UnaryOp::Negate,
                TokenType::Bang => UnaryOp::Not,
                _ => unreachable!(),
            };
            let operand = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                operand: Box::new(operand),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, SyntaxError> {
        if self.is_at_end() {
            return Err(self.error("Unexpected end of input"));
        }

        let token = &self.tokens[self.current];

        match &token.token_type {
            TokenType::Number(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Number(val))
            }
            TokenType::String(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::String(val))
            }
            TokenType::True => {
                self.advance();
                Ok(Expr::Bool(true))
            }
            TokenType::False => {
                self.advance();
                Ok(Expr::Bool(false))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                Ok(Expr::Grouping(Box::new(expr)))
            }
            _ => Err(self.error(&format!("Unexpected token: {:?}", token.token_type))),
        }
    }

    fn match_tokens(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.current].token_type)
            == std::mem::discriminant(token_type)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, SyntaxError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        Err(self.error(message))
    }

    fn error(&self, message: &str) -> SyntaxError {
        let token = self.peek();
        SyntaxError {
            message: message.to_string(),
            line: token.line,
            column: token.column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer;

    fn parse_expr(input: &str) -> Expr {
        let tokens = lexer::scan(input).unwrap();
        let mut parser = Parser::new(tokens);
        parser.parse().unwrap()
    }

    #[test]
    fn test_number() {
        let expr = parse_expr("42");
        assert!(matches!(expr, Expr::Number(42)));
    }

    #[test]
    fn test_addition() {
        let expr = parse_expr("1 + 2");
        match expr {
            Expr::Binary { operator: BinaryOp::Add, .. } => {}
            _ => panic!("Expected Binary Add"),
        }
    }

    #[test]
    fn test_multiplication() {
        let expr = parse_expr("3 * 4");
        match expr {
            Expr::Binary { operator: BinaryOp::Mul, .. } => {}
            _ => panic!("Expected Binary Mul"),
        }
    }

    #[test]
    fn test_precedence() {
        // 2 + 3 * 4 should parse as 2 + (3 * 4)
        let expr = parse_expr("2 + 3 * 4");
        match expr {
            Expr::Binary { operator: BinaryOp::Add, right, .. } => {
                match *right {
                    Expr::Binary { operator: BinaryOp::Mul, .. } => {}
                    _ => panic!("Expected Mul as right operand"),
                }
            }
            _ => panic!("Expected Binary Add at top"),
        }
    }

    #[test]
    fn test_grouping() {
        // (1 + 2) * 3 should have Add inside Grouping
        let expr = parse_expr("(1 + 2) * 3");
        match expr {
            Expr::Binary { operator: BinaryOp::Mul, left, .. } => {
                match *left {
                    Expr::Grouping(inner) => {
                        match *inner {
                            Expr::Binary { operator: BinaryOp::Add, .. } => {}
                            _ => panic!("Expected Add inside grouping"),
                        }
                    }
                    _ => panic!("Expected Grouping as left operand"),
                }
            }
            _ => panic!("Expected Binary Mul at top"),
        }
    }

    #[test]
    fn test_unary_negation() {
        let expr = parse_expr("-5");
        match expr {
            Expr::Unary { operator: UnaryOp::Negate, operand } => {
                assert!(matches!(*operand, Expr::Number(5)));
            }
            _ => panic!("Expected Unary Negate"),
        }
    }

    #[test]
    fn test_left_associativity() {
        // 1 + 2 + 3 should parse as (1 + 2) + 3
        let expr = parse_expr("1 + 2 + 3");
        match expr {
            Expr::Binary { operator: BinaryOp::Add, left, right } => {
                match *left {
                    Expr::Binary { operator: BinaryOp::Add, .. } => {}
                    _ => panic!("Expected Add as left operand"),
                }
                assert!(matches!(*right, Expr::Number(3)));
            }
            _ => panic!("Expected Binary Add at top"),
        }
    }
}
