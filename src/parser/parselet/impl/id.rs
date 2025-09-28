use crate::expr::Expr;
use crate::token::Token;
use crate::parser::Parser;
use crate::parser::parselet::prefix::PrefixParselet;
use crate::error::LoxResult;

pub struct IdParselet;

impl PrefixParselet for IdParselet {
    fn parse(&self, _parser: &mut Parser, token: &Token) -> LoxResult<Expr> {
        Ok(Expr::variable(token.clone()))
    }
}
