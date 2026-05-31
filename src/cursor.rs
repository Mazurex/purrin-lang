// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

use crate::span::Position;

pub struct LexerCursor {
    pub chars: Vec<char>,
    pub char_position: usize,
    pub position: Position,
}

impl LexerCursor {
    pub fn new(chars: Vec<char>) -> Self {
        Self {
            chars,
            char_position: 0,
            position: Position::new(1, 0),
        }
    }

    pub fn is_eof(&self) -> bool {
        self.char_position >= self.chars.len()
    }

    pub fn peek(&self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            Some(self.chars[self.char_position])
        }
    }

    pub fn peek_next(&self) -> Option<char> {
        if self.char_position + 1 >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.char_position + 1])
        }
    }

    pub fn peek_by(&self, chars: usize) -> Option<String> {
        let upper_bound = self.char_position + chars;

        if upper_bound > self.chars.len() {
            return None;
        }

        Some(self.chars[self.char_position..upper_bound].iter().collect())
    }

    pub fn advance(&mut self) -> Option<char> {
        if self.is_eof() {
            None
        } else {
            let c = self.peek();
            self.char_position += 1;

            if c == Some('\n') {
                self.position.advance_line()
            } else {
                self.position.advance_col()
            }

            c
        }
    }

    pub fn advance_by(&mut self, count: usize) -> Option<String> {
        if self.char_position + count > self.chars.len() {
            return None;
        }

        let mut result = String::new();

        for _ in 0..count {
            let c = self.advance()?;
            result.push(c);
        }

        Some(result)
    }
}
