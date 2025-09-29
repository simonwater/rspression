use crate::error::LoxResult;
use crate::expr::Expr;
use crate::expr::GetExpr;
use crate::parser::parselet::infix::InfixParselet;
use crate::parser::{Parser, precedence::Precedence};
use crate::token::Token;

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

        if let Expr::Get(GetExpr { object, name }) = lhs {
            Ok(Expr::set(*object, name, rhs))
        } else {
            Ok(Expr::assign(lhs, token.clone(), rhs))
        }
    }

    fn get_precedence(&self) -> i32 {
        self.precedence
    }
}
