use crate::error::{RspError, RspResult};
use crate::values::Value;
use crate::{Token, TokenType};
use std::iter::Peekable;
use std::rc::Rc;
use std::str::Chars;

pub struct Scanner<'a> {
    source: &'a str,
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Rc<Token<'a>>>,
    current_char: Option<char>,
    start: usize,
    current: usize,
    line: usize,
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || is_chinese_character(c)
}

fn is_alpha_numeric(c: char) -> bool {
    c.is_ascii_digit() || is_alpha(c)
}

fn is_chinese_character(c: char) -> bool {
    // 检查基本汉字和扩展A区
    (c >= '\u{4E00}' && c <= '\u{9FFF}') || (c >= '\u{3400}' && c <= '\u{4DBF}')
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            source: source,
            tokens: Vec::<Rc<Token>>::new(),
            current_char: None,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> RspResult<Vec<Rc<Token<'a>>>> {
        while !self.is_at_end() {
            let token = self.next_token()?;
            self.tokens.push(Rc::new(token));
        }

        let token = Token::new(TokenType::Eof, "", None, self.line);
        self.tokens.push(Rc::new(token));

        Ok(std::mem::take(&mut self.tokens))
    }

    pub fn next_token(&mut self) -> RspResult<Token<'a>> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenType::Eof);
        }

        self.advance();

        let token = self.scan_token()?;
        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    if self.peek_next() == '/' {
                        // Comment
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    fn scan_token(&mut self) -> RspResult<Token<'a>> {
        let c = self.current_char.unwrap_or('\0');
        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            ';' => self.make_token(TokenType::Semicolon),
            '%' => self.make_token(TokenType::Percent),
            '*' => {
                if self.match_char('*') {
                    self.make_token(TokenType::StarStar)
                } else {
                    self.make_token(TokenType::Star)
                }
            }
            '/' => self.make_token(TokenType::Slash),
            '!' => {
                if self.match_char('=') {
                    self.make_token(TokenType::BangEqual)
                } else {
                    self.make_token(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.make_token(TokenType::EqualEqual)
                } else {
                    self.make_token(TokenType::Equal)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.make_token(TokenType::GreaterEqual)
                } else {
                    self.make_token(TokenType::Greater)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.make_token(TokenType::LessEqual)
                } else {
                    self.make_token(TokenType::Less)
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.make_token(TokenType::Or)
                } else {
                    return Err(RspError::ParseError {
                        line: self.line,
                        message: format!("Unexpected character: {}", c),
                    });
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.make_token(TokenType::And)
                } else {
                    return Err(RspError::ParseError {
                        line: self.line,
                        message: format!("Unexpected character: {}", c),
                    });
                }
            }
            '"' => self.string(),
            c if c.is_ascii_digit() => self.number(),
            c if is_alpha(c) => self.identifier(),
            '\0' => Ok(Token::new(TokenType::Eof, "", None, self.line)),
            _ => {
                return Err(RspError::ParseError {
                    line: self.line,
                    message: format!("Unexpected character: {}", c),
                });
            }
        }
    }

    fn string(&mut self) -> RspResult<Token<'a>> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(RspError::ParseError {
                line: self.line,
                message: "Unterminated string".to_string(),
            });
        }

        self.advance(); // Closing quote

        let value = self.source[self.start + 1..self.current - 1].to_string();
        Ok(self.token_with_literal(TokenType::String, Some(Value::String(value))))
    }

    fn number(&mut self) -> RspResult<Token<'a>> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let mut is_double = false;
        if self.peek() == '.' {
            self.advance();
            if self.peek().is_ascii_digit() {
                // 小数点后必须有数字
                is_double = true;
                while self.peek().is_ascii_digit() {
                    self.advance();
                }
            } else {
                return Err(RspError::ParseError {
                    line: self.line,
                    message: "Invalid number format".to_string(),
                });
            }
        }

        let value_str = &self.source[self.start..self.current];
        let value = if is_double {
            let d: f64 = value_str.parse().map_err(|_| RspError::ParseError {
                line: self.line,
                message: "Invalid number".to_string(),
            })?;
            Value::Double(d)
        } else {
            let i: i32 = value_str.parse().map_err(|_| RspError::ParseError {
                line: self.line,
                message: "Invalid number".to_string(),
            })?;
            Value::Integer(i)
        };

        Ok(self.token_with_literal(TokenType::Number, Some(value)))
    }

    fn identifier(&mut self) -> RspResult<Token<'a>> {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.identifier_type(text);
        self.make_token(token_type)
    }

    fn identifier_type(&self, text: &str) -> TokenType {
        match text {
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "null" => TokenType::Null,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.chars.peek().is_none()
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.chars.next() {
            self.current_char = Some(c);
            self.current += c.len_utf8();
            return Some(c);
        } else {
            self.current_char = None;
            return None;
        }
    }

    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.peek() != c {
            return false;
        }
        self.advance();
        true
    }

    fn peek(&mut self) -> char {
        if let Some(&c) = self.chars.peek() {
            c
        } else {
            '\0'
        }
    }

    fn peek_next(&mut self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        if let Some(&c) = iter.peek() { c } else { '\0' }
    }

    fn make_token(&mut self, token_type: TokenType) -> RspResult<Token<'a>> {
        Ok(self.token_with_literal(token_type, None))
    }

    fn token_with_literal(&mut self, token_type: TokenType, literal: Option<Value>) -> Token<'a> {
        let text = &self.source[self.start..self.current];
        Token::new(token_type, text, literal, self.line)
    }
}
