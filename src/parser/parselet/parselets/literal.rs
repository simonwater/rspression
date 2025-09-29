use crate::Value;
use crate::error::LoxResult;
use crate::expr::Expr;
use crate::parser::Parser;
use crate::parser::parselet::prefix::PrefixParselet;
use crate::{Token, TokenType};

pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    fn parse(&self, _parser: &mut Parser, token: &Token) -> LoxResult<Expr> {
        let value = match token.token_type {
            TokenType::Number | TokenType::String => token.literal.clone().unwrap_or(Value::Null),
            TokenType::True => Value::Boolean(true),
            TokenType::False => Value::Boolean(false),
            TokenType::Null => Value::Null,
            _ => Value::Null,
        };
        Ok(Expr::literal(value))
    }
}
