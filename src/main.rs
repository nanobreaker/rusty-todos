mod token;
mod tokenizer;

use ratatui::DefaultTerminal;
use ratatui::Frame;
use std::io;

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, _frame: &mut Frame) {}

    fn handle_events(&mut self) -> io::Result<()> {
        Ok(())
    }
}
