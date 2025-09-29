use crate::expr::Expr;
use crate::Token;
use crate::parser::{Parser, precedence::Precedence};
use crate::parser::parselet::prefix::PrefixParselet;
use crate::error::LoxResult;

pub struct IfParselet;

impl PrefixParselet for IfParselet {
    fn parse(&self, parser: &mut Parser, _token: &Token) -> LoxResult<Expr> {
        parser.consume(crate::TokenType::LeftParen, "Expected '(' after 'if'")?;
        let condition = parser.expression_prec(Precedence::PREC_NONE)?;
        parser.consume(crate::TokenType::Comma, "Expected ',' after condition")?;
        let then_branch = parser.expression_prec(Precedence::PREC_NONE)?;
        parser.consume(crate::TokenType::Comma, "Expected ',' after then branch")?;
        let else_branch = parser.expression_prec(Precedence::PREC_NONE)?;
        parser.consume(crate::TokenType::RightParen, "Expected ')' after else branch")?;
        
        Ok(Expr::if_expr(condition, then_branch, Some(else_branch)))
    }
}
