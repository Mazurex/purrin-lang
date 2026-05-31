// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

#![allow(dead_code)]

use crate::cursor::LexerCursor;
use crate::error::lexer_error::{LexerError, LexerErrorKind};
use crate::file::SourceFile;
use crate::lexer::tokens::{ESCAPE_CHARS, KEYWORDS, SYMBOLS, Token, TokenType};
use crate::span::{Position, Span};

pub struct Lexer {
    pub source_file: SourceFile,
    pub cursor: LexerCursor,
}

impl Lexer {
    pub fn new(source_file: SourceFile) -> Lexer {
        let chars = source_file.source.chars().collect();

        Self {
            source_file,
            cursor: LexerCursor::new(chars),
        }
    }

    pub fn try_comment(&mut self) -> Result<bool, LexerError> {
        let is_multiline = self.cursor.peek_by(3) == Some(String::from("///"));
        let mut is_terminated = false;

        let starting_pos = Position::from_cursor(&self.cursor);

        if is_multiline || self.cursor.peek_by(2) == Some(String::from("//")) {
            if is_multiline {
                self.cursor.advance_by(3);
            }

            while let Some(c) = self.cursor.peek() {
                if !is_multiline && (c == '\n' || c == '\0') {
                    break;
                }

                if is_multiline && self.cursor.peek_by(3) == Some(String::from("///")) {
                    is_terminated = true;
                    self.cursor.advance_by(3);
                    break;
                }

                self.cursor.advance();
            }

            if is_multiline && !is_terminated {
                return Err(LexerError::new(
                    self.source_file.clone(),
                    Span::new(starting_pos, Position::from_cursor(&self.cursor)),
                    LexerErrorKind::UnterminatedMultilineComment,
                    String::from("Multiline comment missing a '///' terminator"),
                    Some(String::from(
                        "Append '///' to the end of your multiline comment",
                    )),
                ));
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn try_number(&mut self) -> Result<Option<Token>, LexerError> {
        let Some(c) = self.cursor.peek() else {
            return Ok(None);
        };

        if !c.is_ascii_digit() {
            return Ok(None);
        };

        let starting_pos = Position::from_cursor(&self.cursor);
        let mut value = String::new();
        let mut is_float = false;
        let mut last_valid_num_pos = 0;

        while let Some(c) = self.cursor.peek() {
            if c.is_ascii_digit() {
                last_valid_num_pos += 1;
                value.push(c);
                self.cursor.advance();
                continue;
            }

            if c == '_' {
                self.cursor.advance();
                continue;
            }

            if c == '.' {
                if is_float {
                    return Err(LexerError::new(
                        self.source_file.clone(),
                        Span::new(
                            Position::from_cursor(&self.cursor),
                            Position::new(
                                self.cursor.position.line,
                                self.source_file.get_line(self.cursor.position.line).len(),
                            ),
                        ),
                        LexerErrorKind::InvalidFloat,
                        String::from("Invalid float literal"),
                        Some(format!(
                            "Omit trailing '.' -> {}",
                            &value[..value.len() - 1]
                        )),
                    ));
                }

                is_float = true;

                let dot_pos = Position::from_cursor(&self.cursor);

                value.push(c);
                self.cursor.advance();

                match self.cursor.peek() {
                    Some(next) if next.is_ascii_digit() => {}
                    _ => {
                        return Err(LexerError::new(
                            self.source_file.clone(),
                            Span::single(dot_pos),
                            LexerErrorKind::InvalidNumber,
                            String::from("Expected digit after '.'"),
                            Some(format!(
                                "Omit the trailing '.' -> {}",
                                &value[..value.len() - 1]
                            )),
                        ));
                    }
                }

                continue;
            }

            if c.is_alphabetic() {
                return Err(LexerError::new(
                    self.source_file.clone(),
                    Span::new(
                        Position::from_cursor(&self.cursor),
                        Position::new(
                            self.cursor.position.line,
                            self.source_file.get_line(self.cursor.position.line).len(),
                        ),
                    ),
                    LexerErrorKind::InvalidNumber,
                    String::from("Invalid integer literal"),
                    Some(format!(
                        "Omit invalid characters -> {}",
                        &value[..last_valid_num_pos]
                    )),
                ));
            }

            break;
        }

        Ok(Some(Token::with_value(
            if is_float {
                TokenType::FloatLit
            } else {
                TokenType::IntegerLit
            },
            value,
            Span::new(starting_pos, Position::from_cursor(&self.cursor)),
        )))
    }

    pub fn try_identifier(&mut self) -> Option<Token> {
        let Some(c) = self.cursor.peek() else {
            return None;
        };

        if !c.is_alphabetic() && c != '_' {
            return None;
        }

        let starting_pos = Position::from_cursor(&self.cursor);

        let mut value = String::new();

        while let Some(c) = self.cursor.peek() {
            if c.is_alphanumeric() || c == '_' {
                value.push(c);
                self.cursor.advance();
                continue;
            }

            break;
        }

        let keyword_token = KEYWORDS
            .iter()
            .find(|(kw, _)| *kw == value)
            .map(|(_, token)| token);
        let token_type = keyword_token.copied().unwrap_or(TokenType::Identifier);

        let token_value = if token_type == TokenType::Identifier || token_type == TokenType::BoolLit
        {
            Some(value)
        } else {
            None
        };

        Some(Token {
            t: token_type,
            v: token_value,
            span: Span::new(starting_pos, Position::from_cursor(&self.cursor)),
        })
    }

    pub fn try_symbol(&mut self) -> Option<Token> {
        let starting_pos = Position::from_cursor(&self.cursor);

        for (symbol, token_type) in SYMBOLS {
            if self.cursor.peek_by(symbol.len()).as_deref() == Some(symbol) {
                self.cursor.advance_by(symbol.len());

                return Some(Token::new(
                    *token_type,
                    Span::new(starting_pos, Position::from_cursor(&self.cursor)),
                ));
            }
        }

        None
    }

    pub fn try_string(&mut self) -> Result<Option<Token>, LexerError> {
        let Some(c) = self.cursor.peek() else {
            return Ok(None);
        };

        if c != '"' {
            return Ok(None);
        }

        let starting_pos = Position::from_cursor(&self.cursor);
        let is_multiline = self.cursor.peek_by(3) == Some(String::from("\"\"\""));

        let term = if is_multiline { "\"\"\"" } else { "\"" };
        let term_len = term.len();

        let mut is_terminated = false;
        let mut value = String::new();

        self.cursor.advance_by(if is_multiline { 3 } else { 1 });

        while let Some(c) = self.cursor.peek() {
            if c == '\n' && !is_multiline {
                break;
            }

            if self.cursor.peek_by(term_len) == Some(term.to_string()) {
                self.cursor.advance_by(term_len);
                is_terminated = true;
                break;
            }

            value.push(c);
            self.cursor.advance();
        }

        if !is_terminated {
            return Err(LexerError::new(
                self.source_file.clone(),
                Span::single(Position::from_cursor(&self.cursor)),
                LexerErrorKind::UnterminatedString,
                format!("String literal missing {} terminator", term),
                Some(format!("\"{}\"", value)),
            ));
        }

        Ok(Some(Token::with_value(
            TokenType::StringLit,
            value,
            Span::new(starting_pos, Position::from_cursor(&self.cursor)),
        )))
    }

    pub fn try_char(&mut self) -> Result<Option<Token>, LexerError> {
        let Some(c) = self.cursor.peek() else {
            return Ok(None);
        };

        if c != '\'' {
            return Ok(None);
        }

        let starting_pos = Position::from_cursor(&self.cursor);

        self.cursor.advance();

        let mut is_terminated = false;
        let mut value = String::new();

        while let Some(c) = self.cursor.peek() {
            if c == '\n' {
                break;
            }

            if c == '\'' {
                is_terminated = true;
                self.cursor.advance();
                break;
            }

            value.push(c);
            self.cursor.advance();
        }

        if !is_terminated {
            return Err(LexerError::new(
                self.source_file.clone(),
                Span::single(Position::from_cursor(&self.cursor)),
                LexerErrorKind::UnterminatedChar,
                String::from("Unterminated char literal"),
                if value.len() == 1 {
                    Some(format!("'{}'", value))
                } else {
                    None
                },
            ));
        }

        let is_escape = ESCAPE_CHARS.iter().any(|name| *name == value);

        if value.len() != 1 && !is_escape {
            return Err(LexerError::new(
                self.source_file.clone(),
                Span::new(starting_pos, Position::from_cursor(&self.cursor)),
                LexerErrorKind::InvalidChar,
                String::from("Char literal is not a valid char"),
                Some(format!(
                    "Remove extra chars -> {:?}",
                    value.chars().next().unwrap_or(' ')
                )),
            ));
        }

        Ok(Some(Token::with_value(
            TokenType::CharLit,
            value,
            Span::new(starting_pos, Position::from_cursor(&self.cursor)),
        )))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.cursor.peek() {
            if self.try_comment()? {
                continue;
            }

            if c.is_whitespace() {
                self.cursor.advance();
                continue;
            }

            if let Some(token) = self.try_number()? {
                tokens.push(token);
                continue;
            }

            if let Some(token) = self.try_identifier() {
                tokens.push(token);
                continue;
            }

            if let Some(token) = self.try_symbol() {
                tokens.push(token);
                continue;
            }

            if let Some(token) = self.try_string()? {
                tokens.push(token);
                continue;
            }

            if let Some(token) = self.try_char()? {
                tokens.push(token);
                continue;
            }

            return Err(LexerError::new(
                self.source_file.clone(),
                Span::single(Position::from_cursor(&self.cursor)),
                LexerErrorKind::UnexpectedCharacter,
                format!("Unknown symbol '{c}'"),
                None,
            ));
        }

        tokens.push(Token::new(
            TokenType::EOF,
            Span::single(Position::from_cursor(&self.cursor)),
        ));

        Ok(tokens)
    }
}
