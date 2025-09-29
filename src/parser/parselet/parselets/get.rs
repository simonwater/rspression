use crate::Token;
use crate::error::LoxResult;
use crate::expr::Expr;
use crate::parser::Parser;
use crate::parser::parselet::infix::InfixParselet;

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
        let name = parser.consume(
            crate::TokenType::Identifier,
            "Expect property name after '.'",
        )?;
        Ok(Expr::get(object, name))
    }

    fn get_precedence(&self) -> i32 {
        self.precedence
    }
}
