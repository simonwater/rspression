use crate::LoxError;
use crate::Token;
use crate::error::LoxResult;
use crate::expr::Expr;
use crate::parser::Parser;

pub trait InfixParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: &Token) -> LoxResult<Expr>;
    fn get_precedence(&self) -> i32;
}

pub struct UnknownInfixParselet;
impl InfixParselet for UnknownInfixParselet {
    fn parse(&self, _parser: &mut Parser, _lhs: Expr, token: &Token) -> LoxResult<Expr> {
        Err(LoxError::ParseError {
            line: token.line,
            message: format!("Unknown infix operator: {:?}", token),
        })
    }

    fn get_precedence(&self) -> i32 {
        0
    }
}
