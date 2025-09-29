use crate::LoxError;
use crate::Token;
use crate::error::LoxResult;
use crate::expr::Expr;
use crate::parser::Parser;

pub trait PrefixParselet {
    fn parse(&self, parser: &mut Parser, token: &Token) -> LoxResult<Expr>;
}

pub struct UnknownPrefixParselet;
impl PrefixParselet for UnknownPrefixParselet {
    fn parse(&self, _parser: &mut Parser, token: &Token) -> LoxResult<Expr> {
        Err(LoxError::ParseError {
            line: token.line,
            message: format!("Unknown token: {:?}", token),
        })
    }
}
