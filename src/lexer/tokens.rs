#[derive(Debug)]
#[allow(dead_code)]
pub enum TokenType {
    IntLiteral,
    FloatLiteral,
    
    Plus,
    Minus,
    Times,
    TimesTimes, // Implement
    Slash,
    Percent,

    LParen,
    RParen,
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
    Keyword,

    EOF
}

impl TryFrom<char> for TokenType {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(TokenType::Plus),
            '-' => Ok(TokenType::Minus),
            '*' => Ok(TokenType::Times),
            '/' => Ok(TokenType::Slash),
            '%' => Ok(TokenType::Percent),
            '=' => Ok(TokenType::Equals),
            '!' => Ok(TokenType::Not),
            '>' => Ok(TokenType::MoreThan),
            '<' => Ok(TokenType::LessThan),
            '(' => Ok(TokenType::LParen),
            ')' => Ok(TokenType::RParen),
            ';' => Ok(TokenType::Semi),
            _ => Err(())
        }
    }
}

impl TokenType {
    pub fn try_from_two(value: char, next: char) -> Result<TokenType, ()> {
        let token = format!("{value}{next}");

        match token.as_str() {
            "**" => Ok(TokenType::TimesTimes),
            "==" => Ok(TokenType::EqualsEquals),
            "!=" => Ok(TokenType::NotEquals),
            "<=" => Ok(TokenType::LessEqualThan),
            ">=" => Ok(TokenType::MoreEqualThan),
            "||" => Ok(TokenType::Or),
            "&&" => Ok(TokenType::And),
            _ => Err(())
        }
    }
}

pub struct Token {
    pub(crate) t: TokenType,
    pub(crate) value: Option<Vec<u8>>
}

impl Token {
    pub fn stringify_value(&self) -> String {
        match &self.value {
            Some(bytes) => String::from_utf8_lossy(bytes).into_owned(),
            None => String::new()
        }
    }
}