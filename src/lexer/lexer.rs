#![allow(dead_code)]

use crate::error::{LexerError, LexerErrorKind};
use crate::lexer::cursor::Cursor;
use crate::lexer::tokens::{KEYWORDS, SYMBOLS, Token, TokenType};

pub struct Lexer {
    pub file_name: String,
    pub cursor: Cursor,
}

impl Lexer {
    pub fn new(src: String, file_name: String) -> Lexer {
        Self {
            file_name,
            cursor: Cursor::new(src),
        }
    }

    pub fn try_comment(&mut self) -> bool {
        if self.cursor.peek_by(2) == Some(String::from("//")) {
            while let Some(c) = self.cursor.peek() {
                if c == '\n' || c == '\0' {break}

                self.cursor.advance();
            }

            true
        } else {
            false
        }
    }

    pub fn try_number(&mut self) -> Result<Option<Token>, LexerError> {
        let Some(c) = self.cursor.peek() else {
            return Ok(None);
        };

        if !c.is_ascii_digit() {
            return Ok(None);
        };

        let mut value = String::new();
        let mut is_float = false;

        while let Some(c) = self.cursor.peek() {
            if c.is_ascii_digit() {
                value.push(c);
                self.cursor.advance();
                continue;
            }

            if c == '.' {
                if is_float {
                    return Err(LexerError::init(
                        self,
                        LexerErrorKind::InvalidNumber,
                        String::from("Invalid float literals"),
                    ));
                }

                is_float = true;

                value.push(c);
                self.cursor.advance();

                match self.cursor.peek() {
                    Some(next) if next.is_ascii_digit() => {}
                    _ => {
                        return Err(LexerError::init(
                            self,
                            LexerErrorKind::InvalidNumber,
                            String::from("Expected digit after '.'"),
                        ));
                    }
                }

                continue;
            }

            if c.is_alphabetic() {
                return Err(LexerError::init(
                    self,
                    LexerErrorKind::InvalidNumber,
                    String::from("Invalid number literal"),
                ));
            }

            break;
        }

        Ok(Some(Token::with_value(
            if is_float {
                TokenType::Float
            } else {
                TokenType::Number
            },
            value,
        )))
    }

    pub fn try_identifier(&mut self) -> Option<Token> {
        let Some(c) = self.cursor.peek() else {
            return None;
        };

        if !c.is_alphabetic() {
            return None;
        }

        let mut value = String::new();

        while let Some(c) = self.cursor.peek() {
            if c.is_alphanumeric() {
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

        Some(Token::new(token_type))
    }

    pub fn try_symbol(&mut self) -> Option<Token> {
        for (symbol, token_type) in SYMBOLS {
            if self.cursor.peek_by(symbol.len()).as_deref() == Some(symbol) {
                self.cursor.advance_by(symbol.len());

                return Some(Token::new(*token_type));
            }
        }

        None
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(c) = self.cursor.peek() {
            // TODO: Separate line management into its own manager
            if c == '\n' {
                self.cursor.line += 1;
                self.cursor.col = 0;
            } else {
                self.cursor.col += 1;
            }

            if self.try_comment() {
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

            return Err(LexerError::init(
                self,
                LexerErrorKind::UnexpectedCharacter,
                format!("Unknown symbol '{c}'"),
            ));
        }

        tokens.push(Token {
            t: TokenType::EOF,
            v: None,
        });

        Ok(tokens)
    }
}
