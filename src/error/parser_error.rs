// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

use crate::parser::Parser;

#[derive(Debug)]
pub enum ParserErrorKind {
    UnexpectedToken
}

pub struct ParserError {
    pub kind: ParserErrorKind,
    pub message: String,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, message: String) -> Self {
        Self {kind, message}
    }
}