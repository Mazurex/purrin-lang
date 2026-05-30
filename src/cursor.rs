// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

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

    pub fn get_line(&self, target_line: usize) -> String {
        self.src
            .lines()
            .nth(target_line - 1)
            .unwrap_or("")
            .to_string()
    }
}