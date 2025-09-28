use crate::expr::{Expr, GetExpr};
use crate::token::Token;
use crate::parser::{Parser, precedence::Precedence};
use crate::parser::parselet::infix::InfixParselet;
use crate::error::LoxResult;

pub struct AssignParselet {
    precedence: i32,
}

impl AssignParselet {
    pub fn new(precedence: i32) -> Self {
        Self { precedence }
    }
}

impl InfixParselet for AssignParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: &Token) -> LoxResult<Expr> {
        // 右结合，优先级降低一位，有连续等号时先解析后面的
        let rhs = parser.expression_prec(self.precedence - 1)?;
        
        if let Expr::Get { object, name } = lhs {
            Ok(Expr::set(*object, name, rhs))
        } else {
            Ok(Expr::assign(token.clone(), rhs))
        }
    }

    fn get_precedence(&self) -> i32 {
        self.precedence
    }
}
