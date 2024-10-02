mod parser;
mod token;
mod tokenizer;

use color_eyre::Result;
use parser::Parser;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use std::io;
use tokenizer::Tokenizer;

#[derive(Debug, Default)]
struct App {
    input: String,
    output: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

impl App {
    const fn new() -> Self {
        Self {
            input: String::new(),
            output: String::new(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => {
                        // todo: focus input by combination of Shift+':'
                        // todo: handle input submit
                    }
                }
            }
        }
    }

    fn draw(&self, _frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(3)]);
        let [grid_area, input_area] = layout.areas(_frame.area());

        // todo: implement grid area to display todos
        let paragraph = Paragraph::new("rusty todos . . . . .");
        _frame.render_widget(paragraph, grid_area);

        // todo: draw cursor when input is focused
        // todo: focus input when edit mode is selected
        let input = Paragraph::new(self.input.as_str())
            .style(Style::default())
            .block(Block::bordered().title("Input"));
        _frame.render_widget(input, input_area);
    }
}
