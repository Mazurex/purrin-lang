use crate::cursor::ParserCursor;
use crate::lexer::tokens::{Token, TokenType};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub cursor: ParserCursor,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let cursor = ParserCursor::new(tokens.clone());

        Self { tokens, cursor }
    }

    pub fn parse_tokens(&self) {
        while let Some(token) = self.cursor.peek() {
            if token.t == TokenType::EOF {
                break;
            }


        }
    }
}
