// Copyright 2026 Maz
// Licensed under the Apache License, Version 2.0

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub enum TokenType {
    NumberLit,
    FloatLit,
    BoolLit,
    StringLit,
    CharLit,
    Idk,

    Plus,
    Minus,
    Times,
    TimesTimes,
    Slash,
    Percent,
    Inc,
    Dec,

    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Semi,

    Equals,
    EqualsEquals,
    NotEquals,
    LessThan,
    MoreThan,
    LessEqualThan,
    MoreEqualThan,

    Not,
    Or,
    And,

    Identifier,

    Return,
    If,
    Else,

    Number,
    String,
    Bool,
    Float,
    Char,
    Const,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub(crate) t: TokenType,
    pub(crate) v: Option<String>,
}

impl Token {
    pub fn new(t: TokenType) -> Self {
        Self { t, v: None }
    }

    pub fn with_value(t: TokenType, v: String) -> Self {
        Self { t, v: Some(v) }
    }

    pub fn as_str(&self) -> String {
        match &self.v {
            Some(val) => {
                let escaped = val.replace('\n', "\\n");
                format!("{:?}({})", self.t, escaped)
            }
            None => format!("{:?}", self.t),
        }
    }
}

pub const KEYWORDS: &[(&str, TokenType)] = &[
    ("return", TokenType::Return),
    ("if", TokenType::If),
    ("else", TokenType::Else),

    ("Number", TokenType::Number),
    ("Float", TokenType::Float),
    ("String", TokenType::String),
    ("Char", TokenType::Char),
    ("Bool", TokenType::Bool),

    ("const", TokenType::Const),
    ("idk", TokenType::Idk),

    ("true", TokenType::BoolLit),
    ("false", TokenType::BoolLit),
];

pub const ESCAPE_CHARS: &[&str] = &[
    "\\n",
    "\\r",
    "\\t",
    "\\0",
    "\\\\",
    "\\\"",
    "\\\'",
];

pub const SYMBOLS: &[(&str, TokenType)] = &[
    ("**", TokenType::TimesTimes),
    ("==", TokenType::EqualsEquals),
    ("!=", TokenType::NotEquals),
    ("<=", TokenType::LessEqualThan),
    (">=", TokenType::MoreEqualThan),
    ("||", TokenType::Or),
    ("&&", TokenType::And),
    ("++", TokenType::Inc),
    ("--", TokenType::Dec),
    ("+", TokenType::Plus),
    ("-", TokenType::Minus),
    ("*", TokenType::Times),
    ("/", TokenType::Slash),
    ("%", TokenType::Percent),
    ("=", TokenType::Equals),
    (";", TokenType::Semi),
    ("(", TokenType::LParen),
    (")", TokenType::RParen),
    ("{", TokenType::LBrace),
    ("}", TokenType::RBrace),
    ("[", TokenType::LBracket),
    ("]", TokenType::RBracket),
    ("!", TokenType::Not),
    (">", TokenType::MoreThan),
    ("<", TokenType::LessThan),
];
