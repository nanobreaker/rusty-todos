use crate::{token::Token, tokenizer::Tokenizer};

pub enum ParserErr {
    Error,
}

pub struct Parser {
    pub tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        return Self { tokenizer };
    }

    pub fn parse(&mut self, source: &str) -> Result<String, ParserErr> {
        let chars: Vec<char> = source.chars().collect();
        let tokens: Vec<Token> = match self.tokenizer.tokenize(chars) {
            Ok(it) => it,
            Err(err) => return Err(ParserErr::Error),
        };

        Ok("text".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::fmt;

    impl fmt::Debug for ParserErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[test]
    fn should_return_no_command_when_source_is_empty() -> Result<(), ParserErr> {
        let tokenizer: Tokenizer = Tokenizer::new();
        let mut parser = Parser::new(tokenizer);
        let source = "";
        let result = parser.parse(source);

        assert!(result.is_ok());

        Ok(())
    }
}
