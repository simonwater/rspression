use crate::expr::Expr;
use crate::token::Token;
use crate::parser::Parser;
use crate::error::LoxResult;

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: &Token) -> LoxResult<Expr>;
    fn get_precedence(&self) -> i32;
}
