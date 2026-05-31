// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

use crate::file::SourceFile;
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
    pub source_file: SourceFile,
    pub span: Span,
    pub message: String,
    pub suggested_fix: Option<String>,
}

impl LexerError {
    pub fn new(source_file: SourceFile, span: Span, kind: LexerErrorKind, message: String, suggested_fix: Option<String>) -> Self {
        Self {
            kind,
            source_file,
            span,
            message,
            suggested_fix,
        }
    }

    pub fn display(&self) {
        println!("Error[{:?}]: {}", self.kind, self.message);
        println!(" --> {}:{}:{}", self.source_file.file_name, self.span.start.line, self.span.start.col);

        let line_str = self.span.start.line.to_string();

        println!("{:>width$} |", "", width = line_str.len());

        println!("{} | {}", line_str, self.source_file.get_line(self.span.start.line));

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