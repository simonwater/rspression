use crate::error::{LoxError, LoxResult};
use crate::expr::{Expr, GetExpr};
use crate::parser::parselet::{
    infix::InfixParselet,
    parselets::{
        AssignParselet, BinaryParselet, CallParselet, GetParselet, GroupParselet, IdParselet,
        IfParselet, LiteralParselet, LogicParselet, PreUnaryParselet,
    },
    prefix::PrefixParselet,
};
use crate::parser::precedence::Precedence;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> LoxResult<Expr> {
        let result = self.expression_prec(Precedence::PREC_NONE)?;
        if self.peek().token_type != TokenType::Eof {
            return Err(LoxError::ParseError {
                line: self.peek().line,
                message: format!("Unknown token: {:?}", self.peek()),
            });
        }
        Ok(result)
    }

    pub fn expression_prec(&mut self, min_prec: i32) -> LoxResult<Expr> {
        let token = self.advance();
        let lhs = self.parse_prefix(&token)?;
        self.parse_infix(lhs, min_prec)
    }

    fn parse_prefix(&mut self, token: &Token) -> LoxResult<Expr> {
        match token.token_type {
            TokenType::Number | TokenType::String => Ok(Expr::literal(
                token.literal.clone().unwrap_or(crate::values::Value::Null),
            )),
            TokenType::True => Ok(Expr::literal(crate::values::Value::Boolean(true))),
            TokenType::False => Ok(Expr::literal(crate::values::Value::Boolean(false))),
            TokenType::Null => Ok(Expr::literal(crate::values::Value::Null)),
            TokenType::Identifier => Ok(Expr::id(token.clone())),
            TokenType::LeftParen => {
                let expr = self.expression_prec(Precedence::PREC_NONE)?;
                self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                Ok(expr)
            }
            TokenType::Minus | TokenType::Bang => {
                let rhs = self.expression_prec(Precedence::PREC_UNARY)?;
                Ok(Expr::unary(token.clone(), rhs))
            }
            TokenType::If => {
                self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
                let condition = self.expression_prec(Precedence::PREC_NONE)?;
                self.consume(TokenType::Comma, "Expected ',' after condition")?;
                let then_branch = self.expression_prec(Precedence::PREC_NONE)?;
                self.consume(TokenType::Comma, "Expected ',' after then branch")?;
                let else_branch = self.expression_prec(Precedence::PREC_NONE)?;
                self.consume(TokenType::RightParen, "Expected ')' after else branch")?;
                Ok(Expr::if_expr(condition, then_branch, Some(else_branch)))
            }
            _ => Err(LoxError::ParseError {
                line: token.line,
                message: format!("Unknown token: {:?}", token),
            }),
        }
    }

    fn parse_infix(&mut self, mut lhs: Expr, min_prec: i32) -> LoxResult<Expr> {
        while self.peek().token_type != TokenType::Eof {
            let next = self.peek();
            let precedence = self.get_precedence(&next.token_type);

            if precedence <= min_prec {
                break;
            }

            let token = self.advance();
            lhs = self.parse_infix_op(lhs, &token)?;
        }

        Ok(lhs)
    }

    fn parse_infix_op(&mut self, lhs: Expr, token: &Token) -> LoxResult<Expr> {
        match token.token_type {
            TokenType::Plus | TokenType::Minus => {
                let rhs = self.expression_prec(Precedence::PREC_TERM)?;
                Ok(Expr::binary(lhs, token.clone(), rhs))
            }
            TokenType::Star | TokenType::Slash | TokenType::Percent => {
                let rhs = self.expression_prec(Precedence::PREC_FACTOR)?;
                Ok(Expr::binary(lhs, token.clone(), rhs))
            }
            TokenType::StarStar => {
                let rhs = self.expression_prec(Precedence::PREC_POWER - 1)?; // Right associative
                Ok(Expr::binary(lhs, token.clone(), rhs))
            }
            TokenType::Equal => {
                let rhs = self.expression_prec(Precedence::PREC_ASSIGNMENT - 1)?; // Right associative
                if let Expr::Get(GetExpr { object, name }) = lhs {
                    Ok(Expr::set(*object, name, rhs))
                } else {
                    Ok(Expr::assign(lhs, token.clone(), rhs))
                }
            }
            TokenType::Or | TokenType::And => {
                let rhs = self.expression_prec(if token.token_type == TokenType::Or {
                    Precedence::PREC_OR
                } else {
                    Precedence::PREC_AND
                })?;
                Ok(Expr::logic(lhs, token.clone(), rhs))
            }
            TokenType::EqualEqual | TokenType::BangEqual => {
                let rhs = self.expression_prec(Precedence::PREC_EQUALITY)?;
                Ok(Expr::binary(lhs, token.clone(), rhs))
            }
            TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual => {
                let rhs = self.expression_prec(Precedence::PREC_COMPARISON)?;
                Ok(Expr::binary(lhs, token.clone(), rhs))
            }
            TokenType::LeftParen => {
                let mut arguments = Vec::new();

                if !self.check(&TokenType::RightParen) {
                    loop {
                        if arguments.len() >= 255 {
                            return Err(LoxError::ParseError {
                                line: self.peek().line,
                                message: "Can't have more than 255 arguments".to_string(),
                            });
                        }
                        arguments.push(self.expression_prec(Precedence::PREC_NONE)?);

                        if !self.match_token(&[TokenType::Comma]) {
                            break;
                        }
                    }
                }

                let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments")?;
                Ok(Expr::call(lhs, arguments, paren))
            }
            TokenType::Dot => {
                let name = self.consume(TokenType::Identifier, "Expect property name after '.'")?;
                Ok(Expr::get(lhs, name))
            }
            _ => Err(LoxError::ParseError {
                line: token.line,
                message: format!("Unknown infix operator: {:?}", token),
            }),
        }
    }

    fn get_precedence(&self, token_type: &TokenType) -> i32 {
        match token_type {
            TokenType::Equal => Precedence::PREC_ASSIGNMENT,
            TokenType::Or => Precedence::PREC_OR,
            TokenType::And => Precedence::PREC_AND,
            TokenType::EqualEqual | TokenType::BangEqual => Precedence::PREC_EQUALITY,
            TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual => Precedence::PREC_COMPARISON,
            TokenType::Plus | TokenType::Minus => Precedence::PREC_TERM,
            TokenType::Star | TokenType::Slash | TokenType::Percent => Precedence::PREC_FACTOR,
            TokenType::StarStar => Precedence::PREC_POWER,
            TokenType::LeftParen | TokenType::Dot => Precedence::PREC_CALL,
            _ => 0,
        }
    }

    pub fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> LoxResult<Token> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(LoxError::ParseError {
                line: self.peek().line,
                message: message.to_string(),
            })
        }
    }

    pub fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        self.peek().token_type == *token_type
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}
