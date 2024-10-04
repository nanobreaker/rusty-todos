use chrono::NaiveDateTime;

#[derive(PartialEq)]
pub enum Command {
    TodoCreate {
        title: String,
        description: Option<String>,
        start: Option<NaiveDateTime>,
        end: Option<NaiveDateTime>,
    },
    TodoRead {
        ids: Vec<i8>,
    },
    TodoUpdate {
        ids: Vec<i8>,
        title: Option<String>,
        description: Option<String>,
        start: Option<NaiveDateTime>,
        end: Option<NaiveDateTime>,
    },
    TodoDelete {
        ids: Vec<i8>,
    },
    UserRead,
    CalendarRead,
}
