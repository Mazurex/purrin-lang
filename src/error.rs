// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

use crate::lexer::lexer::Lexer;

#[derive(Debug)]
pub enum LexerErrorKind {
    UnexpectedCharacter,
    UnterminatedString,
    UnterminatedChar,
    InvalidChar,
    InvalidNumber,
    InvalidFloat,

    UnterminatedMultilineComment
}

// TODO: Change "note:" to "suggested fix:" (but also make it optional)
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub file_name: String,
    pub line: usize,
    pub col: usize,
    pub message: String,
    pub suggested_fix: Option<String>,
    pub source_line: String,
}

impl LexerError {
    pub fn new(lexer: &Lexer, kind: LexerErrorKind, message: String, suggested_fix: Option<String>) -> Self {
        Self {
            kind,
            file_name: lexer.file_name.clone(),
            line: lexer.cursor.line,
            col: lexer.cursor.col,
            message,
            suggested_fix,
            source_line: lexer.cursor.get_line(lexer.cursor.line),
        }
    }

    pub fn with_span(lexer: &Lexer, kind: LexerErrorKind, line: usize, col: usize, message: String, suggested_fix: Option<String>) -> Self {
        Self {
            kind,
            file_name: lexer.file_name.clone(),
            line,
            col,
            message,
            suggested_fix,
            source_line: lexer.cursor.get_line(lexer.cursor.line),
        }
    }

    pub fn display(&self) {
        println!("Error[{:?}]: {}", self.kind, self.message);
        println!(" --> {}:{}:{}", self.file_name, self.line, self.col);

        let line_str = self.line.to_string();

        println!("{:>width$} |", "", width = line_str.len());

        println!("{} | {}", line_str, self.source_line);

        let padding = self.col;
        println!(
            "{:>width$} | {:>padding$}\x1b[31m^\x1b[0m",
            "",
            "",
            width = line_str.len(),
            padding = padding
        );

        if let Some(s) = &self.suggested_fix {
            println!("{:>width$} |", "", width = line_str.len());
            println!("{:>width$} = help: {}", "", s, width = line_str.len())
        }
    }
}