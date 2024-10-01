use crate::token::{Token, TokenType};

#[derive(Debug)]
enum TokenizerErr {
    EmptySource,
}

pub struct Tokenizer {
    source: Vec<char>,
}

enum State {
    BeginToken,
    EndToken(TokenType),
    Keyword,
    Option,
    Text,
    Exit,
}

impl Tokenizer {
    pub fn new(source: &str) -> Self {
        return Self {
            source: source.chars().collect(),
        };
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerErr> {
        if self.source.len() == 0 {
            return Err(TokenizerErr::EmptySource);
        }

        let mut tokens: Vec<Token> = Vec::<Token>::new();
        let mut char_iter = self.source.clone().into_iter();
        let mut state = State::BeginToken;
        let mut char_buffer = Vec::<char>::new();
        let mut char = char_iter.next();

        loop {
            state = match state {
                State::BeginToken => match char {
                    Some(c) => {
                        if c.is_alphabetic() {
                            char_buffer.push(c);
                            char = char_iter.next();
                            State::Keyword
                        } else if c.is_whitespace() {
                            char = char_iter.next();
                            State::BeginToken
                        } else if c == '"' {
                            char = char_iter.next();
                            State::Text
                        } else if c == '-' {
                            char = char_iter.next();
                            State::Option
                        } else {
                            State::Exit
                        }
                    }
                    None => State::Exit,
                },
                State::Keyword => match char {
                    Some(c) => {
                        if c.is_alphabetic() {
                            char_buffer.push(c);
                            char = char_iter.next();
                            State::Keyword
                        } else {
                            char = char_iter.next();
                            State::EndToken(TokenType::Keyword)
                        }
                    }
                    None => State::EndToken(TokenType::Keyword),
                },
                State::Text => match char {
                    Some(c) => {
                        if c != '"' {
                            char_buffer.push(c);
                            char = char_iter.next();
                            State::Text
                        } else {
                            char = char_iter.next();
                            State::EndToken(TokenType::Text)
                        }
                    }
                    None => State::EndToken(TokenType::Text),
                },
                State::Option => match char {
                    Some(c) => {
                        if c.is_alphabetic() {
                            char_buffer.push(c);
                            char = char_iter.next();
                            State::EndToken(TokenType::Option)
                        } else {
                            char = char_iter.next();
                            State::EndToken(TokenType::UnknownOption)
                        }
                    }
                    None => State::EndToken(TokenType::Option),
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

    #[test]
    fn should_tokenize_single_keyword() -> Result<(), TokenizerErr> {
        let source = "todo";
        let mut tokenizer = Tokenizer::new(source);
        let tokens: Vec<Token> = tokenizer.tokenize()?;

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
        let source = "todo create";
        let mut tokenizer = Tokenizer::new(source);
        let tokens: Vec<Token> = tokenizer.tokenize()?;

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
        let source = "-d";
        let mut tokenizer = Tokenizer::new(source);
        let tokens: Vec<Token> = tokenizer.tokenize()?;

        assert_eq!(tokens.len(), 1);
        assert_eq!(
            tokens.get(0),
            Some(&Token::new(TokenType::Option, Some("d".chars().collect())))
        );
        Ok(())
    }

    #[test]
    fn should_tokenize_text() -> Result<(), TokenizerErr> {
        let source = "\"rusty todos\"";
        let mut tokenizer = Tokenizer::new(source);
        let tokens: Vec<Token> = tokenizer.tokenize()?;

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
    fn should_tokenize_all_tokens() -> Result<(), TokenizerErr> {
        let source = "todo create -d\"rusty todos\"";
        let mut tokenizer = Tokenizer::new(source);
        let tokens: Vec<Token> = tokenizer.tokenize()?;

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
}
