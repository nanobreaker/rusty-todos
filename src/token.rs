#[derive(PartialEq)]
pub enum TokenType {
    Keyword,
    Text,
    Option,
}

#[derive(PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<Vec<char>>,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<Vec<char>>) -> Self {
        return Self { token_type, value };
    }
}
