use crate::Token;
use crate::error::LoxResult;
use crate::expr::Expr;
use crate::expr::GetExpr;
use crate::parser::parselet::infix::InfixParselet;
use crate::parser::{Parser, precedence::Precedence};
use std::rc::Rc;

pub struct AssignParselet;

impl InfixParselet for AssignParselet {
    fn parse(&self, parser: &mut Parser, lhs: Expr, token: Rc<Token>) -> LoxResult<Expr> {
        // 右结合，优先级降低一位，有连续等号时先解析后面的
        let rhs = parser.expression_prec(self.get_precedence() - 1)?;

        if let Expr::Get(GetExpr { object, name }) = lhs {
            Ok(Expr::set(*object, name, rhs))
        } else {
            Ok(Expr::assign(lhs, token.clone(), rhs))
        }
    }

    fn get_precedence(&self) -> i32 {
        Precedence::PREC_ASSIGNMENT
    }
}
