// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

use crate::lexer::lexer::Lexer;
use crate::span::{Span};

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

pub struct LexerError {
    pub kind: LexerErrorKind,
    pub file_name: String,
    pub span: Span,
    pub message: String,
    pub suggested_fix: Option<String>,
    pub source_line: String,
}

impl LexerError {
    pub fn new(lexer: &Lexer, span: Span, kind: LexerErrorKind, message: String, suggested_fix: Option<String>) -> Self {
        Self {
            kind,
            file_name: lexer.file_name.clone(),
            span,
            message,
            suggested_fix,
            source_line: lexer.cursor.get_line(lexer.cursor.position.line),
        }
    }

    /// TODO: Support span rather than just start pos
    pub fn display(&self) {
        println!("Error[{:?}]: {}", self.kind, self.message);
        println!(" --> {}:{}:{}", self.file_name, self.span.start.line, self.span.start.col);

        let line_str = self.span.start.line.to_string();

        println!("{:>width$} |", "", width = line_str.len());

        println!("{} | {}", line_str, self.source_line);

        let padding = self.span.start.col;
        let span_width = self.span.end.col.saturating_sub(self.span.start.col).max(1);

        println!(
            "{:>width$} | {}{}",
            "",
            " ".repeat(padding),
            format!("\x1b[31m{}\x1b[0m", "^".repeat(span_width)),
            width = line_str.len()
        );

        if let Some(s) = &self.suggested_fix {
            println!("{:>width$} |", "", width = line_str.len());
            println!("{:>width$} = help: {}", "", s, width = line_str.len())
        }
    }
}