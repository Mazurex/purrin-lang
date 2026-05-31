// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

use std::fmt::Write;
use crate::file::SourceFile;
use crate::span::Span;

#[derive(Debug)]
pub enum LexerErrorKind {
    UnexpectedCharacter,
    UnterminatedString,
    UnterminatedChar,
    InvalidChar,
    InvalidNumber,
    InvalidFloat,

    UnterminatedMultilineComment,
}

pub struct LexerError {
    pub kind: LexerErrorKind,
    pub source_file: SourceFile,
    pub span: Span,
    pub message: String,
    pub suggested_fix: Option<String>,
}

impl LexerError {
    pub fn new(
        source_file: SourceFile,
        span: Span,
        kind: LexerErrorKind,
        message: String,
        suggested_fix: Option<String>,
    ) -> Self {
        Self {
            kind,
            source_file,
            span,
            message,
            suggested_fix,
        }
    }

    pub fn kind_as_str(&self) -> &'static str {
        match self.kind {
            LexerErrorKind::UnexpectedCharacter => "unexpected character",
            LexerErrorKind::UnterminatedString => "unterminated string",
            LexerErrorKind::UnterminatedChar => "unterminated character",
            LexerErrorKind::InvalidChar => "invalid character",
            LexerErrorKind::InvalidNumber => "invalid number",
            LexerErrorKind::InvalidFloat => "invalid float",
            LexerErrorKind::UnterminatedMultilineComment => "unterminated comment",
        }
    }

    pub fn format(&self) -> String {
        let mut out = String::new();

        let _ = writeln!(out, "Error[{:?}]: {}", self.kind, self.message);
        let _ = writeln!(
            out,
            " --> {}:{}:{}",
            self.source_file.file_name,
            self.span.start.line,
            self.span.start.col
        );

        let line_str = self.span.start.line.to_string();

        let _ = writeln!(
            out,
            "{:>width$} |",
            "",
            width = line_str.len()
        );

        let source_line = self.source_file.get_line(self.span.start.line);

        let _ = writeln!(
            out,
            "{} | {}",
            line_str,
            source_line
        );

        let padding = self.span.start.col;
        let span_width = self
            .span
            .end
            .col
            .saturating_sub(self.span.start.col)
            .max(1);

        let _ = writeln!(
            out,
            "{:>width$} | {}{}",
            "",
            " ".repeat(padding),
            format!("\x1b[31m{}\x1b[0m", "^".repeat(span_width)),
            width = line_str.len()
        );

        if let Some(s) = &self.suggested_fix {
            let _ = writeln!(out, "{:>width$} |", "", width = line_str.len());
            let _ = writeln!(out, "{:>width$} = help: {}", "", s, width = line_str.len());
        }

        out
    }
}
