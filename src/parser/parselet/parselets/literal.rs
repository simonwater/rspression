use crate::expr::Expr;
use crate::token::Token;
use crate::parser::{Parser, precedence::Precedence};
use crate::parser::parselet::prefix::PrefixParselet;
use crate::error::LoxResult;

pub struct LiteralParselet;

impl PrefixParselet for LiteralParselet {
    fn parse(&self, _parser: &mut Parser, token: &Token) -> LoxResult<Expr> {
        Ok(Expr::literal(token.literal.clone().unwrap_or(crate::values::Value::Null)))
    }
}
