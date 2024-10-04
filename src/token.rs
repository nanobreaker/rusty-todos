#[derive(PartialEq, Clone)]
pub enum Keyword {
    Todo,
    User,
    Calendar,
    Create,
    Read,
    Update,
    Delete,
    Unknown,
}

#[derive(PartialEq, Clone)]
pub enum Text {
    Some(String),
    None,
}

#[derive(PartialEq, Clone)]
pub enum Option {
    Title,
    Description,
    Start,
    End,
    Unknown,
}

#[derive(PartialEq, Clone)]
pub enum Token {
    Keyword(Keyword),
    Text(Text),
    Option(Option),
}

impl Into<Keyword> for Vec<char> {
    fn into(self) -> Keyword {
        let keyword: &str = &self.iter().collect::<String>();
        match keyword {
            "todo" => Keyword::Todo,
            "user" => Keyword::User,
            "calendar" => Keyword::Calendar,
            "create" => Keyword::Create,
            "read" => Keyword::Read,
            "update" => Keyword::Update,
            "delete" => Keyword::Delete,
            _ => Keyword::Unknown,
        }
    }
}

impl Into<Text> for Vec<char> {
    fn into(self) -> Text {
        if self.is_empty() {
            Text::None
        } else {
            Text::Some(self.into_iter().collect())
        }
    }
}

impl Into<Option> for Vec<char> {
    fn into(self) -> Option {
        let option: &str = &self.iter().collect::<String>();
        match option {
            "t" => Option::Title,
            "d" => Option::Description,
            "s" => Option::Start,
            "e" => Option::End,
            _ => Option::Unknown,
        }
    }
}
