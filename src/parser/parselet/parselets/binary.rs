use crate::Token;
use crate::error::LoxResult;
use crate::expr::Expr;
use crate::parser::Parser;
use crate::parser::parselet::infix::InfixParselet;
use std::rc::Rc;

pub struct BinaryParselet {
    precedence: i32,
    is_right: bool,
}

impl BinaryParselet {
    pub fn new(precedence: i32) -> Self {
        Self {
            precedence,
            is_right: false,
        }
    }

    pub fn new_right_associative(precedence: i32) -> Self {
        Self {
            precedence,
            is_right: true,
        }
    }
}

impl InfixParselet for BinaryParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Rc<Token>) -> LoxResult<Expr> {
        let rhs = parser.expression_prec(if self.is_right {
            self.precedence - 1
        } else {
            self.precedence
        })?;
        Ok(Expr::binary(lhs, token.clone(), rhs))
    }

    fn get_precedence(&self) -> i32 {
        self.precedence
    }
}
