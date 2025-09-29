use crate::error::LoxResult;
use crate::expr::Expr;
use crate::parser::Parser;
use crate::parser::parselet::prefix::PrefixParselet;
use crate::token::Token;

pub struct IdParselet;

impl PrefixParselet for IdParselet {
    fn parse(&self, _parser: &mut Parser, token: &Token) -> LoxResult<Expr> {
        Ok(Expr::id(token.clone()))
    }
}
