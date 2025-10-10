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

pub struct Parser {
    tokens: Vec<Rc<Token>>,
    current: usize,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }

    pub fn parse(&mut self, source: String) -> LoxResult<Expr> {
        let scanner = &mut Scanner::new(source);
        let tokens = scanner.scan_tokens()?;
        self.parse_tokens(tokens)
    }

    pub fn parse_tokens(&mut self, tokens: Vec<Rc<Token>>) -> LoxResult<Expr> {
        self.tokens = tokens;
        self.current = 0;
        let result = self.expression_prec(Precedence::PREC_NONE)?;
        if self.peek().token_type != TokenType::Eof {
            return Err(LoxError::ParseError {
                line: self.peek().line,
                message: format!("Unknown token: {:?}", self.peek()),
            });
        }
        Ok(result)
    }

    pub fn expression_prec(&mut self, min_prec: i32) -> LoxResult<Expr> {
        let token = self.advance();
        let prefix_parselet = self.get_prefix(&token.token_type);
        let mut lhs = prefix_parselet.parse(self, token)?;

        while self.peek().token_type != TokenType::Eof {
            let next = self.peek();
            let infix_parselet = self.get_infix(&next.token_type);

            if infix_parselet.get_precedence() <= min_prec {
                break;
            }

            let token = self.advance();
            lhs = infix_parselet.parse(self, lhs, token)?;
        }

        Ok(lhs)
    }

    fn get_prefix(&mut self, token_type: &TokenType) -> Box<dyn PrefixParselet> {
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
            Ok(self.advance())
        } else {
            Err(LoxError::ParseError {
                line: self.peek().line,
                message: message.to_string(),
            })
        }
    }

    pub fn advance(&mut self) -> Rc<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        self.peek().token_type == *token_type
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    pub fn previous(&self) -> Rc<Token> {
        //std::mem::take(&mut self.tokens[self.current - 1])
        self.tokens[self.current - 1].clone()
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}
