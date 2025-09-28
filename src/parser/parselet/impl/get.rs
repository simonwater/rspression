use crate::expr::Expr;
use crate::token::Token;
use crate::parser::{Parser, precedence::Precedence};
use crate::parser::parselet::infix::InfixParselet;
use crate::error::LoxResult;

pub struct GetParselet {
    precedence: i32,
}

impl GetParselet {
    pub fn new(precedence: i32) -> Self {
        Self { precedence }
    }
}

impl InfixParselet for GetParselet {
    fn parse(&self, parser: &mut Parser, object: Expr, _token: &Token) -> LoxResult<Expr> {
        let name = parser.consume(crate::token::TokenType::Identifier, "Expect property name after '.'")?;
        Ok(Expr::get(object, name))
    }

    fn get_precedence(&self) -> i32 {
        self.precedence
    }
}
