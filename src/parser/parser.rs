use crate::error::{RspError, RspResult};
use crate::expr::{Expr, GetExpr};
use crate::parser::precedence::Precedence;
use crate::parser::scanner::Scanner;
use crate::{Token, TokenType, Value};
use std::rc::Rc;

pub struct Parser<'a> {
    previous: Rc<Token<'a>>,
    current: Rc<Token<'a>>,
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            previous: Rc::new(Token::default()),
            current: Rc::new(Token::default()),
            scanner: Scanner::new(source),
        }
    }

    pub fn parse(&mut self) -> RspResult<Expr<'a>> {
        self.advance()?;
        let result = self.expression_prec(Precedence::PREC_NONE)?;
        if self.current.token_type != TokenType::Eof {
            return Err(RspError::ParseError {
                line: self.current.line,
                message: format!("Unknown token: {:?}", self.current),
            });
        }
        Ok(result)
    }

    pub fn expression_prec(&mut self, min_prec: i32) -> RspResult<Expr<'a>> {
        self.advance()?;
        let mut lhs = self.parse_prefix(self.previous.clone())?;
        while self.current.token_type != TokenType::Eof {
            let precedence = self.get_precedence(&self.current.token_type);
            if precedence <= min_prec {
                break;
            }

            self.advance()?;
            lhs = self.parse_infix(lhs, self.previous.clone())?;
        }

        Ok(lhs)
    }

    fn get_precedence(&self, token_type: &TokenType) -> i32 {
        match token_type {
            TokenType::Plus | TokenType::Minus => Precedence::PREC_TERM,
            TokenType::Star | TokenType::Slash | TokenType::Percent => Precedence::PREC_FACTOR,
            TokenType::StarStar => Precedence::PREC_POWER,
            TokenType::Equal => Precedence::PREC_ASSIGNMENT,
            TokenType::Or => Precedence::PREC_OR,
            TokenType::And => Precedence::PREC_AND,
            TokenType::EqualEqual | TokenType::BangEqual => Precedence::PREC_EQUALITY,
            TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual => Precedence::PREC_COMPARISON,
            TokenType::LeftParen => Precedence::PREC_CALL,
            TokenType::Dot => Precedence::PREC_CALL,
            _ => Precedence::PREC_NONE,
        }
    }

    fn parse_prefix(&mut self, token: Rc<Token<'a>>) -> RspResult<Expr<'a>> {
        match token.token_type {
            TokenType::Number
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Null => self.literal(token),
            TokenType::Identifier => self.id(token),
            TokenType::LeftParen => self.group(token),
            TokenType::Minus | TokenType::Bang => self.unary(token, Precedence::PREC_UNARY),
            TokenType::If => self.if_(token),
            _ => Err(RspError::ParseError {
                line: token.line,
                message: format!("Unknown token: {:?}", token),
            }),
        }
    }

    fn parse_infix(&mut self, lhs: Expr<'a>, token: Rc<Token<'a>>) -> RspResult<Expr<'a>> {
        match token.token_type {
            TokenType::Plus | TokenType::Minus => {
                self.binary(lhs, token, Precedence::PREC_TERM, false)
            }
            TokenType::Star | TokenType::Slash | TokenType::Percent => {
                self.binary(lhs, token, Precedence::PREC_FACTOR, false)
            }
            TokenType::StarStar => self.binary(lhs, token, Precedence::PREC_POWER, true),
            TokenType::Equal => self.assign(lhs, token),
            TokenType::Or => self.logic(lhs, token, Precedence::PREC_OR),
            TokenType::And => self.logic(lhs, token, Precedence::PREC_AND),
            TokenType::EqualEqual | TokenType::BangEqual => {
                self.binary(lhs, token, Precedence::PREC_EQUALITY, false)
            }
            TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual => {
                self.binary(lhs, token, Precedence::PREC_COMPARISON, false)
            }
            TokenType::LeftParen => self.call(lhs, token),
            TokenType::Dot => self.get(lhs, token),
            _ => Err(RspError::ParseError {
                line: token.line,
                message: format!("Unknown infix operator: {:?}", token),
            }),
        }
    }

    fn assign(&mut self, lhs: Expr<'a>, token: Rc<Token<'a>>) -> RspResult<Expr<'a>> {
        // 右结合，优先级降低一位，有连续等号时先解析后面的
        let rhs = self.expression_prec(Precedence::PREC_ASSIGNMENT - 1)?;

        if let Expr::Get(GetExpr { object, name }) = lhs {
            Ok(Expr::set(*object, name, rhs))
        } else {
            Ok(Expr::assign(lhs, token.clone(), rhs))
        }
    }

    fn binary(
        &mut self,
        lhs: Expr<'a>,
        token: Rc<Token<'a>>,
        precedence: i32,
        is_right: bool,
    ) -> RspResult<Expr<'a>> {
        let rhs = self.expression_prec(if is_right { precedence - 1 } else { precedence })?;
        Ok(Expr::binary(lhs, token.clone(), rhs))
    }

    fn call(&mut self, callee: Expr<'a>, _token: Rc<Token<'a>>) -> RspResult<Expr<'a>> {
        let mut arguments = Vec::new();

        if !self.check(&crate::TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(self.parse_err("Can't have more than 255 arguments".to_string()));
                }
                arguments.push(self.expression_prec(Precedence::PREC_NONE)?);

                if !self.match_token(&[crate::TokenType::Comma])? {
                    break;
                }
            }
        }
        let paren = self.consume(crate::TokenType::RightParen, "Expected ')' after arguments")?;
        Ok(Expr::call(callee, arguments, paren))
    }

    fn get(&mut self, object: Expr<'a>, _token: Rc<Token<'a>>) -> RspResult<Expr<'a>> {
        let name = self.consume(
            crate::TokenType::Identifier,
            "Expect property name after '.'",
        )?;
        Ok(Expr::get(object, name))
    }

    fn group(&mut self, _token: Rc<Token<'a>>) -> RspResult<Expr<'a>> {
        let expr = self.expression_prec(Precedence::PREC_NONE)?;
        self.consume(TokenType::RightParen, "Expected ')' after expression")?;
        Ok(expr)
    }

    fn id(&mut self, token: Rc<Token<'a>>) -> RspResult<Expr<'a>> {
        Ok(Expr::id(token.clone()))
    }

    fn if_(&mut self, _token: Rc<Token<'a>>) -> RspResult<Expr<'a>> {
        self.consume(crate::TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = self.expression_prec(Precedence::PREC_NONE)?;
        self.consume(crate::TokenType::Comma, "Expected ',' after condition")?;
        let then_branch = self.expression_prec(Precedence::PREC_NONE)?;
        self.consume(crate::TokenType::Comma, "Expected ',' after then branch")?;
        let else_branch = self.expression_prec(Precedence::PREC_NONE)?;
        self.consume(
            crate::TokenType::RightParen,
            "Expected ')' after else branch",
        )?;
        Ok(Expr::if_expr(condition, then_branch, Some(else_branch)))
    }

    fn literal(&mut self, token: Rc<Token<'a>>) -> RspResult<Expr<'a>> {
        let value = match token.token_type {
            TokenType::Number | TokenType::String => token.literal.clone().unwrap_or(Value::Null),
            TokenType::True => Value::Boolean(true),
            TokenType::False => Value::Boolean(false),
            TokenType::Null => Value::Null,
            _ => Value::Null,
        };
        Ok(Expr::literal(value))
    }

    fn logic(
        &mut self,
        lhs: Expr<'a>,
        token: Rc<Token<'a>>,
        precedence: i32,
    ) -> RspResult<Expr<'a>> {
        let rhs = self.expression_prec(precedence)?;
        Ok(Expr::logic(lhs, token.clone(), rhs))
    }

    fn unary(&mut self, token: Rc<Token<'a>>, precedence: i32) -> RspResult<Expr<'a>> {
        let rhs = self.expression_prec(precedence)?;
        Ok(Expr::unary(token.clone(), rhs))
    }

    pub fn parse_err(&self, message: String) -> RspError {
        RspError::ParseError {
            line: self.current.line,
            message,
        }
    }

    pub fn match_token(&mut self, types: &[TokenType]) -> RspResult<bool> {
        for token_type in types {
            if self.check(token_type) {
                self.advance()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> RspResult<Rc<Token<'a>>> {
        if self.check(&token_type) {
            self.advance()?;
            Ok(self.previous.clone())
        } else {
            Err(RspError::ParseError {
                line: self.current.line,
                message: message.to_string(),
            })
        }
    }

    fn advance(&mut self) -> RspResult<()> {
        self.previous = self.current.clone();

        if !self.is_at_end() {
            let token = self.scanner.next_token()?;
            self.current = Rc::new(token);
        }
        Ok(())
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        self.current.token_type == *token_type
    }

    fn is_at_end(&self) -> bool {
        self.current.token_type == TokenType::Eof
    }
}
