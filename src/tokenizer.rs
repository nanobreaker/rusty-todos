use std::char;

use crate::token::{Token, TokenType};

pub enum TokenizerErr {
    Error,
}

pub struct Tokenizer {}

enum State {
    BeginToken,
    EndToken(TokenType),
    Keyword,
    Option,
    Text,
    Exit,
}

enum CharCode {
    Alphabetic,
    Whitespace,
    QuotationMark,
    Dash,
    NotSupported,
    Empty,
}

impl CharCode {
    fn map(char: &Option<char>) -> Self {
        match char {
            Some(c) => {
                if c.is_alphabetic() {
                    CharCode::Alphabetic
                } else if c.is_whitespace() {
                    CharCode::Whitespace
                } else if *c == '"' {
                    CharCode::QuotationMark
                } else if *c == '-' {
                    CharCode::Dash
                } else {
                    CharCode::NotSupported
                }
            }
            None => CharCode::Empty,
        }
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        return Self {};
    }

    pub fn tokenize(&mut self, source: Vec<char>) -> Result<Vec<Token>, TokenizerErr> {
        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut char_iter = source.into_iter();
        let mut char_buffer = Vec::<char>::new();
        let mut char = char_iter.next();
        let mut state = State::BeginToken;

        loop {
            state = match state {
                State::BeginToken => match CharCode::map(&char) {
                    CharCode::Alphabetic => {
                        char_buffer.push(char.unwrap());
                        char = char_iter.next();
                        State::Keyword
                    }
                    CharCode::Whitespace => {
                        char = char_iter.next();
                        State::BeginToken
                    }
                    CharCode::QuotationMark => {
                        char = char_iter.next();
                        State::Text
                    }
                    CharCode::Dash => {
                        char = char_iter.next();
                        State::Option
                    }
                    _ => State::Exit,
                },
                State::Keyword => match CharCode::map(&char) {
                    CharCode::Alphabetic => {
                        char_buffer.push(char.unwrap());
                        char = char_iter.next();
                        State::Keyword
                    }
                    _ => State::EndToken(TokenType::Keyword),
                },
                State::Text => match CharCode::map(&char) {
                    CharCode::QuotationMark => {
                        char = char_iter.next();
                        State::EndToken(TokenType::Text)
                    }
                    _ => {
                        char_buffer.push(char.unwrap());
                        char = char_iter.next();
                        State::Text
                    }
                },
                State::Option => match CharCode::map(&char) {
                    CharCode::QuotationMark => State::EndToken(TokenType::Option),
                    CharCode::Empty => State::EndToken(TokenType::Option),
                    CharCode::Whitespace => {
                        char = char_iter.next();
                        State::Option
                    }
                    _ => {
                        char_buffer.push(char.unwrap());
                        char = char_iter.next();
                        State::Option
                    }
                },
                State::EndToken(token_type) => {
                    let token = Token::new(token_type, Some(char_buffer.clone()));
                    tokens.push(token);
                    char_buffer.clear();
                    State::BeginToken
                }
                State::Exit => {
                    break;
                }
            }
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;
    use core::fmt;

    impl fmt::Debug for Token {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl fmt::Debug for TokenizerErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[test]
    fn should_return_no_tokens_when_source_is_empty() -> Result<(), TokenizerErr> {
        let mut tokenizer = Tokenizer::new();
        let source = "".chars().collect();
        let tokens: Vec<Token> = tokenizer.tokenize(source)?;

        assert_eq!(tokens.len(), 0);

        Ok(())
    }

    #[test]
    fn should_tokenize_single_keyword() -> Result<(), TokenizerErr> {
        let mut tokenizer = Tokenizer::new();
        let source = "todo".chars().collect();
        let tokens: Vec<Token> = tokenizer.tokenize(source)?;

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.get(0),
            Some(&Token::new(
                TokenType::Keyword,
                Some("todo".chars().collect())
            ))
        );

        Ok(())
    }

    #[test]
    fn should_tokenize_multiple_keywords() -> Result<(), TokenizerErr> {
        let mut tokenizer = Tokenizer::new();
        let source = "todo create".chars().collect();
        let tokens: Vec<Token> = tokenizer.tokenize(source)?;

        assert_eq!(tokens.len(), 2);
        assert_eq!(
            tokens.get(0),
            Some(&Token::new(
                TokenType::Keyword,
                Some("todo".chars().collect())
            ))
        );
        assert_eq!(
            tokens.get(1),
            Some(&Token::new(
                TokenType::Keyword,
                Some("create".chars().collect())
            ))
        );
        Ok(())
    }

    #[test]
    fn should_tokenize_option() -> Result<(), TokenizerErr> {
        let mut tokenizer = Tokenizer::new();
        let source = "-d".chars().collect();
        let tokens: Vec<Token> = tokenizer.tokenize(source)?;

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.get(0),
            Some(&Token::new(TokenType::Option, Some("d".chars().collect())))
        );
        Ok(())
    }

    #[test]
    fn should_tokenize_text() -> Result<(), TokenizerErr> {
        let mut tokenizer = Tokenizer::new();
        let source = "\"rusty todos\"".chars().collect();
        let tokens: Vec<Token> = tokenizer.tokenize(source)?;

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.get(0),
            Some(&Token::new(
                TokenType::Text,
                Some("rusty todos".chars().collect())
            ))
        );
        Ok(())
    }

    #[test]
    fn should_tokenize_all_tokens_1() -> Result<(), TokenizerErr> {
        let mut tokenizer = Tokenizer::new();
        let source = "todo create -d\"rusty todos\"".chars().collect();
        let tokens: Vec<Token> = tokenizer.tokenize(source)?;

        assert_eq!(tokens.len(), 4);
        assert_eq!(
            tokens.get(0),
            Some(&Token::new(
                TokenType::Keyword,
                Some("todo".chars().collect())
            ))
        );
        assert_eq!(
            tokens.get(1),
            Some(&Token::new(
                TokenType::Keyword,
                Some("create".chars().collect())
            ))
        );
        assert_eq!(
            tokens.get(2),
            Some(&Token::new(TokenType::Option, Some("d".chars().collect())))
        );
        assert_eq!(
            tokens.get(3),
            Some(&Token::new(
                TokenType::Text,
                Some("rusty todos".chars().collect())
            ))
        );
        Ok(())
    }

    #[test]
    fn should_tokenize_all_tokens_2() -> Result<(), TokenizerErr> {
        let mut tokenizer = Tokenizer::new();
        let source = "todo update -d \"rusty todos\" -start \"02.10.2024\""
            .chars()
            .collect();
        let tokens: Vec<Token> = tokenizer.tokenize(source)?;

        assert_eq!(tokens.len(), 6);
        assert_eq!(
            tokens.get(0),
            Some(&Token::new(
                TokenType::Keyword,
                Some("todo".chars().collect())
            ))
        );
        assert_eq!(
            tokens.get(1),
            Some(&Token::new(
                TokenType::Keyword,
                Some("update".chars().collect())
            ))
        );
        assert_eq!(
            tokens.get(2),
            Some(&Token::new(TokenType::Option, Some("d".chars().collect())))
        );
        assert_eq!(
            tokens.get(3),
            Some(&Token::new(
                TokenType::Text,
                Some("rusty todos".chars().collect())
            ))
        );
        assert_eq!(
            tokens.get(4),
            Some(&Token::new(
                TokenType::Option,
                Some("start".chars().collect())
            ))
        );
        assert_eq!(
            tokens.get(5),
            Some(&Token::new(
                TokenType::Text,
                Some("02.10.2024".chars().collect())
            ))
        );
        Ok(())
    }
}
