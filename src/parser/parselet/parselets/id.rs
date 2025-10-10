use crate::Token;
use crate::error::LoxResult;
use crate::expr::Expr;
use crate::parser::Parser;
use crate::parser::parselet::prefix::PrefixParselet;
use std::rc::Rc;

pub struct IdParselet;

impl PrefixParselet for IdParselet {
    fn parse(&self, _parser: &mut Parser, token: Rc<Token>) -> LoxResult<Expr> {
        Ok(Expr::id(token.clone()))
    }
}
