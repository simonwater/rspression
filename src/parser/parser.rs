use crate::error::{LoxError, LoxResult};
use crate::expr::Expr;
use crate::parser::parselet::{
    infix::{InfixParselet, UnknownInfixParselet},
    parselets::{
        AssignParselet, BinaryParselet, CallParselet, GetParselet, GroupParselet, IdParselet,
        IfParselet, LiteralParselet, LogicParselet, PreUnaryParselet,
    },
    prefix::{PrefixParselet, UnknownPrefixParselet},
};
use crate::parser::precedence::Precedence;
use crate::parser::scanner::Scanner;
use crate::{Token, TokenType};
use std::rc::Rc;

pub struct Parser<'a> {
    previous: Rc<Token>,
    current: Rc<Token>,
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            previous: Rc::new(Token::default()),
            current: Rc::new(Token::default()),
            scanner: Scanner::new(source),
        }
    }

    pub fn parse(&mut self) -> LoxResult<Expr> {
        self.advance()?;
        let result = self.expression_prec(Precedence::PREC_NONE)?;
        if self.current.token_type != TokenType::Eof {
            return Err(LoxError::ParseError {
                line: self.current.line,
                message: format!("Unknown token: {:?}", self.current),
            });
        }
        Ok(result)
    }

    pub fn expression_prec(&mut self, min_prec: i32) -> LoxResult<Expr> {
        self.advance();
        let prefix_parselet = self.get_prefix(&self.previous.token_type);
        let mut lhs = prefix_parselet.parse(self, self.previous.clone())?;

        while self.current.token_type != TokenType::Eof {
            let infix_parselet = self.get_infix(&self.current.token_type);

            if infix_parselet.get_precedence() <= min_prec {
                break;
            }

            self.advance();
            lhs = infix_parselet.parse(self, lhs, self.previous.clone())?;
        }

        Ok(lhs)
    }

    fn get_prefix(&self, token_type: &TokenType) -> Box<dyn PrefixParselet> {
        match token_type {
            TokenType::Number
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Null => Box::new(LiteralParselet {}),
            TokenType::Identifier => Box::new(IdParselet {}),
            TokenType::LeftParen => Box::new(GroupParselet {}),
            TokenType::Minus | TokenType::Bang => {
                Box::new(PreUnaryParselet::new(Precedence::PREC_UNARY))
            }
            TokenType::If => Box::new(IfParselet {}),
            _ => Box::new(UnknownPrefixParselet {}),
        }
    }

    fn get_infix(&self, token_type: &TokenType) -> Box<dyn InfixParselet> {
        match token_type {
            TokenType::Plus | TokenType::Minus => {
                Box::new(BinaryParselet::new(Precedence::PREC_TERM))
            }
            TokenType::Star | TokenType::Slash | TokenType::Percent => {
                Box::new(BinaryParselet::new(Precedence::PREC_FACTOR))
            }
            TokenType::StarStar => Box::new(BinaryParselet::new_right_associative(
                Precedence::PREC_POWER,
            )),
            TokenType::Equal => Box::new(AssignParselet {}),
            TokenType::Or => Box::new(LogicParselet::new(Precedence::PREC_OR)),
            TokenType::And => Box::new(LogicParselet::new(Precedence::PREC_AND)),
            TokenType::EqualEqual | TokenType::BangEqual => {
                Box::new(BinaryParselet::new(Precedence::PREC_EQUALITY))
            }
            TokenType::Less
            | TokenType::LessEqual
            | TokenType::Greater
            | TokenType::GreaterEqual => Box::new(BinaryParselet::new(Precedence::PREC_COMPARISON)),
            TokenType::LeftParen => Box::new(CallParselet::new(Precedence::PREC_CALL)),
            TokenType::Dot => Box::new(GetParselet::new(Precedence::PREC_CALL)),
            _ => Box::new(UnknownInfixParselet {}),
        }
    }

    pub fn parse_err(&self, message: String) -> LoxError {
        LoxError::ParseError {
            line: self.current.line,
            message,
        }
    }

    pub fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn consume(&mut self, token_type: TokenType, message: &str) -> LoxResult<Rc<Token>> {
        if self.check(&token_type) {
            self.advance();
            Ok(self.previous.clone())
        } else {
            Err(LoxError::ParseError {
                line: self.current.line,
                message: message.to_string(),
            })
        }
    }

    fn advance(&mut self) -> LoxResult<()> {
        self.previous = self.current.clone();

        if !self.is_at_end() {
            let token = self.scanner.next_token()?;
            self.current = Rc::new(token);
        }
        Ok(())
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        self.current.token_type == *token_type
    }

    fn is_at_end(&self) -> bool {
        self.current.token_type == TokenType::Eof
    }
}
