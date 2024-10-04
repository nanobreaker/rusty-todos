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
    Text(core::option::Option<String>),
    Option(Option),
}

impl Keyword {
    pub fn from(source: Vec<char>) -> Self {
        let str_slice: &str = &source.iter().collect::<String>();
        match str_slice {
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

impl Option {
    pub fn from(source: Vec<char>) -> Self {
        let str_slice: &str = &source.iter().collect::<String>();
        match str_slice {
            "t" => Option::Title,
            "d" => Option::Description,
            "s" => Option::Start,
            "e" => Option::End,
            _ => Option::Unknown,
        }
    }
}
