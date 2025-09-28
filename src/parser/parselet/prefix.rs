use crate::expr::Expr;
use crate::token::Token;
use crate::parser::Parser;
use crate::error::LoxResult;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: &Token) -> LoxResult<Expr>;
}
