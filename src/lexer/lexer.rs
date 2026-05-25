#![allow(dead_code)]

use std::process;
use crate::lexer::tokens::{Token, TokenType};

const KEYWORDS: [&str; 1] = ["return"];

pub struct Lexer {
    pub(crate) src: Vec<u8>,
    pub(crate) position: usize,
    pub(crate) line: usize,
    pub(crate) col: usize
}

impl Lexer {
    pub fn init(src: String) -> Lexer {
        Self {
            src: Vec::from(src),
            position: 0,
            line: 1,
            col: 0,
        }
    }

    fn peek(&self) -> char {
        if self.position >= self.src.len() {
            return '\0';
        }

        self.src[self.position] as char
    }

    fn peek_next(&self) -> char {
        if self.position + 1 >= self.src.len() {
            return '\0';
        }

        self.src[self.position+1] as char
    }

    fn is_eof(&self) -> bool {
        self.position >= self.src.len()
    }

    fn advance(&mut self) -> char {
        if self.is_eof() { return '\0'; }

        let c = self.src[self.position] as char;
        self.position += 1;
        c
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while self.position < self.src.len() {
            let c = self.peek();

            if c == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }

            if c == '/' && self.peek_next() == '/' {
                while self.peek_next() != '\n' && self.peek_next() != '\0' {
                    self.advance();
                }

                self.advance();

                continue
            }

            // Skip whitespace
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            // Numeric Literals
            if c.is_numeric() {
                let mut value = String::new();
                let mut is_float = false;

                while self.peek().is_numeric() {
                    let n = self.peek_next();
                    if n.is_alphabetic() {
                            println!("Unknown character in integer literal '{}' at ({}:{})", value, self.line, self.col);
                            process::exit(1);
                    }

                    if n == '.' {
                        if is_float {
                            println!("Unknown integer literal '{}' at ({}:{})", value, self.line, self.col);
                            process::exit(1);
                        }

                        is_float = true;

                        value.push(self.advance());

                        if !self.peek_next().is_numeric() {
                            println!("Unknown integer literal '{}' at ({}:{})", value, self.line, self.col);
                            process::exit(1);
                        }
                    }

                    value.push(self.advance());
                }

                tokens.push(Token {
                    t: if is_float {TokenType::FloatLiteral} else { TokenType::IntLiteral },
                    value: Some(Vec::from(value))
                });

                continue;
            }

            // Keywords and Identifiers
            if c.is_alphabetic() {
                let mut value = String::new();

                while self.peek().is_alphanumeric() {
                    value.push(self.advance());
                }

                if KEYWORDS.contains(&value.as_str()) {
                    tokens.push(Token {
                        t: TokenType::Keyword, value: Some(Vec::from(value))
                    });
                } else {
                    tokens.push(Token {
                        t: TokenType::Identifier, value: Some(Vec::from(value))
                    });
                }

                continue;
            }

            // Double symbols (ie. ==)
            match TokenType::try_from_two(c, self.peek_next()) {
                Ok(token) => {
                    tokens.push(Token {
                        t: token,
                        value: None
                    });

                    self.advance();
                    self.advance();

                    continue
                },
                Err(_) => {}
            }

            // Singular symbols (ie. =)
            match TokenType::try_from(c) {
                Ok(token) => {
                    tokens.push(Token {
                        t: token,
                        value: None,
                    });

                    self.advance();

                    continue
                },
                Err(_) => {}
            }

            println!("Unknown symbol '{}' at ({}:{})", c, self.line, self.col);
            process::exit(1);
        }

        tokens.push(Token {
            t: TokenType::EOF,
            value: None
        });

        tokens
    }
}