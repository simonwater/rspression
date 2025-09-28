use crate::expr::Expr;
use crate::token::Token;
use crate::parser::{Parser, precedence::Precedence};
use crate::parser::parselet::prefix::PrefixParselet;
use crate::error::LoxResult;

pub struct PreUnaryParselet {
    precedence: i32,
}

impl PreUnaryParselet {
    pub fn new(precedence: i32) -> Self {
        Self { precedence }
    }
}

impl PrefixParselet for PreUnaryParselet {
    fn parse(&self, parser: &mut Parser, token: &Token) -> LoxResult<Expr> {
        let rhs = parser.expression_prec(self.precedence)?;
        Ok(Expr::unary(token.clone(), rhs))
    }
}
