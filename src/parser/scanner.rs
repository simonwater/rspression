use crate::error::{LoxError, LoxResult};
use crate::values::Value;
use crate::{Token, TokenType};
use std::rc::Rc;

pub struct Scanner {
    source: String,
    chars: Vec<(usize, char)>,
    tokens: Vec<Rc<Token>>,
    start: usize,
    current: usize,
    ci: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            chars: source.char_indices().collect(),
            source: source,
            tokens: Vec::<Rc<Token>>::new(),
            start: 0,
            current: 0,
            ci: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> LoxResult<Vec<Rc<Token>>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        let token = Token::new(TokenType::Eof, String::new(), None, self.line);
        self.tokens.push(Rc::new(token));

        Ok(std::mem::take(&mut self.tokens))
    }

    fn scan_token(&mut self) -> LoxResult<()> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '%' => self.add_token(TokenType::Percent),
            '*' => {
                if self.match_char('*') {
                    self.add_token(TokenType::StarStar);
                } else {
                    self.add_token(TokenType::Star);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '/' => {
                if self.match_char('/') {
                    // Comment
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenType::Or);
                } else {
                    return Err(LoxError::ParseError {
                        line: self.line,
                        message: format!("Unexpected character: {}", c),
                    });
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenType::And);
                } else {
                    return Err(LoxError::ParseError {
                        line: self.line,
                        message: format!("Unexpected character: {}", c),
                    });
                }
            }
            ' ' | '\t' | '\r' => {
                // Ignore whitespace
            }
            '\n' => {
                self.line += 1;
            }
            '"' => self.string()?,
            c if c.is_ascii_digit() => self.number()?,
            c if self.is_alpha(c) => self.identifier()?,
            _ => {
                return Err(LoxError::ParseError {
                    line: self.line,
                    message: format!("Unexpected character: {}", c),
                });
            }
        }
        Ok(())
    }

    fn string(&mut self) -> LoxResult<()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::ParseError {
                line: self.line,
                message: "Unterminated string".to_string(),
            });
        }

        self.advance(); // Closing quote

        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token_with_literal(TokenType::String, Some(Value::String(value)));
        Ok(())
    }

    fn number(&mut self) -> LoxResult<()> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        let mut is_double = false;
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            is_double = true;
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value_str = &self.source[self.start..self.current];
        let value = if is_double {
            let d: f64 = value_str.parse().map_err(|_| LoxError::ParseError {
                line: self.line,
                message: "Invalid number".to_string(),
            })?;
            Value::Double(d)
        } else {
            let i: i32 = value_str.parse().map_err(|_| LoxError::ParseError {
                line: self.line,
                message: "Invalid number".to_string(),
            })?;
            Value::Integer(i)
        };

        self.add_token_with_literal(TokenType::Number, Some(value));
        Ok(())
    }

    fn identifier(&mut self) -> LoxResult<()> {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.identifier_type(text);
        self.add_token(token_type);
        Ok(())
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

    fn is_at_end(&self) -> bool {
        self.ci == self.chars.len()
    }

    fn advance(&mut self) -> char {
        let ch = self.chars[self.ci].1;
        self.ci += 1;
        self.current += ch.len_utf8();
        ch
    }
    fn match_char(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let next = self.chars[self.ci].1;
        if next != c {
            return false;
        }
        self.ci += 1;
        self.current += c.len_utf8();
        true
    }
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.ci].1
        }
    }
    fn peek_next(&self) -> char {
        if self.ci + 1 >= self.chars.len() {
            '\0'
        } else {
            self.chars[self.ci + 1].1
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Value>) {
        let text = self.source[self.start..self.current].to_string();
        let token = Token::new(token_type, text, literal, self.line);
        self.tokens.push(Rc::new(token));
    }

    fn is_alpha(&self, c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_' || self.is_chinese_character(c)
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_ascii_digit() || self.is_alpha(c)
    }

    fn is_chinese_character(&self, c: char) -> bool {
        // 检查基本汉字和扩展A区
        (c >= '\u{4E00}' && c <= '\u{9FFF}') || (c >= '\u{3400}' && c <= '\u{4DBF}')
    }
}
