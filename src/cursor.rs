// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

use crate::error::parser_error::{ParserError, ParserErrorKind};
use crate::lexer::tokens::{Token, TokenType};

pub struct LexerCursor {
    pub src: String,
    pub chars: Vec<char>,
    pub position: usize,
    pub line: usize,
    pub col: usize,
}

impl LexerCursor {
    pub fn new(src: String) -> Self {
        let chars = src.chars().collect();

        Self {
            src,
            chars,
            position: 0,
            line: 1,
            col: 0,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.position >= self.chars.len()
    }

    pub fn peek(&self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            Some(self.chars[self.position])
        }
    }

    pub fn peek_next(&self) -> Option<char> {
        if self.position + 1 >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.position + 1])
        }
    }

    pub fn peek_by(&self, chars: usize) -> Option<String> {
        let upper_bound = self.position + chars;

        if upper_bound > self.chars.len() {
            return None;
        }

        Some(self.chars[self.position..upper_bound].iter().collect())
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            let c = self.peek();
            self.position += 1;

            if c == Some('\n') {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }

            c
        }
    }

    pub fn advance_by(&mut self, count: usize) -> Option<String> {
        if self.position + count > self.chars.len() {
            return None;
        }

        let mut result = String::new();

        for _ in 0..count {
            let c = self.advance()?;
            result.push(c);
        }

        Some(result)
    }

    // TODO: remove unused functions
    pub fn consume(&mut self, expected: char) -> Option<char> {
        if self.peek()? != expected {
            None
        } else {
            self.advance()
        }
    }

    pub fn get_line(&self, target_line: usize) -> String {
        self.src
            .lines()
            .nth(target_line - 1)
            .unwrap_or("")
            .to_string()
    }
}

pub struct ParserCursor {
    pub tokens: Vec<Token>,
    pub position: usize,
}

impl ParserCursor {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        let token = self.tokens.get(self.position)?;

        if token.t == TokenType::EOF {
            None
        } else {
            Some(token)
        }
    }

    pub fn peek_by(&self, count: usize) -> Option<&[Token]> {
        let start = self.position;
        let end = start.checked_add(count)?;

        Some(&self.tokens[start..end])
    }

    pub fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.position)?;

        if token.t == TokenType::EOF {
            return None;
        }

        self.position += 1;
        Some(token)
    }

    pub fn advance_by(&mut self, count: usize) -> Option<Vec<&Token>> {
        if self.position + count > self.tokens.len() {
            return None;
        }

        let mut result = Vec::new();

        for _ in 0..count {
            result.push(self.tokens.get(self.position)?);
        }

        Some(result)
    }

    pub fn consume(&mut self, expected: TokenType) -> Result<Option<&Token>, ParserError> {
        let Some(token) = self.advance() else {
            return Ok(None)
        };

        if token.t != expected {
            return Err(ParserError::new(ParserErrorKind::UnexpectedToken, String::from("Unexpected token provided")))
        }

        Ok(Some(token))
    }
}
