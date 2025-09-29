use crate::Token;
use crate::error::LoxResult;
use crate::expr::Expr;
use crate::parser::parselet::infix::InfixParselet;
use crate::parser::{Parser, precedence::Precedence};

pub struct CallParselet {
    precedence: i32,
}

impl CallParselet {
    pub fn new(precedence: i32) -> Self {
        Self { precedence }
    }
}

impl InfixParselet for CallParselet {
    fn parse(&self, parser: &mut Parser, callee: Expr, token: &Token) -> LoxResult<Expr> {
        let mut arguments = Vec::new();

        if !parser.check(&crate::TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(crate::error::LoxError::ParseError {
                        line: parser.peek().line,
                        message: "Can't have more than 255 arguments".to_string(),
                    });
                }
                arguments.push(parser.expression_prec(Precedence::PREC_NONE)?);

                if !parser.match_token(&[crate::TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = parser.consume(crate::TokenType::RightParen, "Expected ')' after arguments")?;
        Ok(Expr::call(callee, arguments, paren))
    }

    fn get_precedence(&self) -> i32 {
        self.precedence
    }
}
