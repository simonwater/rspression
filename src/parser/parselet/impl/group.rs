use crate::expr::Expr;
use crate::token::{Token, TokenType};
use crate::parser::{Parser, precedence::Precedence};
use crate::parser::parselet::prefix::PrefixParselet;
use crate::error::LoxResult;

pub struct GroupParselet;

impl PrefixParselet for GroupParselet {
    fn parse(&self, parser: &mut Parser, _token: &Token) -> LoxResult<Expr> {
        let expr = parser.expression_prec(Precedence::PREC_NONE)?;
        parser.consume(TokenType::RightParen, "Expected ')' after expression")?;
        Ok(expr)
    }
}
