use chrono::NaiveDateTime;

use crate::{
    command::Command,
    token::{self, Keyword, Text, Token},
    tokenizer::Tokenizer,
};

pub enum ParserErr {
    EmptySource,
    RequiredArgument,
    UnexpectedToken,
}

#[derive(Default)]
pub struct Parser {
    pub tokenizer: Tokenizer,
}

impl Parser {
    pub fn new(tokenizer: Tokenizer) -> Self {
        return Self { tokenizer };
    }

    pub fn parse(&mut self, source: &str) -> Result<Command, ParserErr> {
        let chars: Vec<char> = source.chars().collect();
        let tokens: Vec<Token> = match self.tokenizer.tokenize(chars) {
            Ok(it) => it,
            Err(_) => return Err(ParserErr::EmptySource),
        };

        self.parse_program(&tokens)
    }

    fn parse_program(&mut self, tokens: &Vec<Token>) -> Result<Command, ParserErr> {
        let (program_token, tokens) = tokens.split_first().expect("no tokens");

        match program_token {
            Token::Keyword(Keyword::Todo) => self.parse_todo_command(tokens),
            Token::Keyword(Keyword::User) => self.parse_user_command(tokens),
            Token::Keyword(Keyword::Calendar) => self.parse_calendar_command(tokens),
            _ => Err(ParserErr::UnexpectedToken),
        }
    }

    fn parse_todo_command(&mut self, tokens: &[Token]) -> Result<Command, ParserErr> {
        let (command_token, tokens) = tokens.split_first().expect("no tokens");

        match command_token {
            Token::Keyword(Keyword::Create) => self.parse_todo_create_command(tokens),
            Token::Keyword(Keyword::Read) => todo!(),
            Token::Keyword(Keyword::Update) => todo!(),
            Token::Keyword(Keyword::Delete) => todo!(),
            _ => Err(ParserErr::UnexpectedToken),
        }
    }

    fn parse_todo_create_command(&mut self, tokens: &[Token]) -> Result<Command, ParserErr> {
        let (argument_token, tokens) = tokens.split_first().expect("no tokens");

        let title = match argument_token {
            Token::Text(Text::Some(text)) => text.clone(),
            _ => return Err(ParserErr::RequiredArgument),
        };
        let description = self.extract_option(&token::Option::Description, tokens);
        let start = self
            .extract_option(&token::Option::Start, tokens)
            .map(|text| NaiveDateTime::parse_from_str(text.as_str(), "%d-%m-%Y %H:%M:%S"))
            .map(|result| result.ok())
            .flatten();
        let end = self
            .extract_option(&token::Option::End, tokens)
            .map(|text| NaiveDateTime::parse_from_str(text.as_str(), "%d-%m-%Y %H:%M:%S"))
            .map(|result| result.ok())
            .flatten();

        Ok(Command::TodoCreate {
            title,
            description,
            start,
            end,
        })
    }

    fn parse_user_command(&mut self, _tokens: &[Token]) -> Result<Command, ParserErr> {
        todo!()
    }

    fn parse_calendar_command(&mut self, _tokens: &[Token]) -> Result<Command, ParserErr> {
        todo!()
    }

    fn extract_option(&mut self, option_type: &token::Option, tokens: &[Token]) -> Option<String> {
        tokens
            .windows(2)
            .find(|window| matches!(&window[0], Token::Option(option) if option == option_type))
            .map(|window| match &window[1] {
                Token::Text(Text::Some(text)) => Some(text.clone()),
                _ => None,
            })
            .flatten()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};

    use super::*;
    use core::fmt;

    impl fmt::Debug for ParserErr {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    impl fmt::Debug for Command {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[test]
    fn should_return_todo_create_command() -> Result<(), ParserErr> {
        let tokenizer: Tokenizer = Tokenizer::new();
        let mut parser = Parser::new(tokenizer);
        let source = "todo create \"title\" -d\"description\" -s\"04-10-2024 00:00:00\" -e\"04-10-2024 00:00:00\"";
        let result = parser.parse(source);

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Command::TodoCreate {
                title: "title".to_string(),
                description: Some("description".to_string()),
                start: Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 10, 4).unwrap(),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                )),
                end: Some(NaiveDateTime::new(
                    NaiveDate::from_ymd_opt(2024, 10, 4).unwrap(),
                    NaiveTime::from_hms_opt(0, 0, 0).unwrap()
                ))
            }
        );

        Ok(())
    }
}
