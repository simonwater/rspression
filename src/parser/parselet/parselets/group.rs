use crate::error::LoxResult;
use crate::expr::Expr;
use crate::parser::parselet::prefix::PrefixParselet;
use crate::parser::{Parser, precedence::Precedence};
use crate::{Token, TokenType};

pub struct GroupParselet;

impl PrefixParselet for GroupParselet {
    fn parse(&self, parser: &mut Parser, _token: &Token) -> LoxResult<Expr> {
        let expr = parser.expression_prec(Precedence::PREC_NONE)?;
        parser.consume(TokenType::RightParen, "Expected ')' after expression")?;
        Ok(expr)
    }
}
