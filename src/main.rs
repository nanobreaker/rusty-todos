mod command;
mod parser;
mod token;
mod tokenizer;

use color_eyre::Result;
use parser::Parser;
use ratatui::crossterm::event;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use tokenizer::Tokenizer;
use tui_textarea::Input;
use tui_textarea::Key;
use tui_textarea::TextArea;

#[derive(Default)]
struct App<'a> {
    parser: Parser,
    input: TextArea<'a>,
    _output: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let mut textarea = TextArea::default();
    textarea.set_cursor_line_style(Style::default());
    let tokenizer = Tokenizer::new();
    let parser = Parser::new(tokenizer);
    let app_result = App::new(parser, textarea.clone()).run(&mut terminal);
    ratatui::restore();
    app_result
}

impl<'a> App<'a> {
    const fn new(parser: Parser, input: TextArea<'a>) -> App<'a> {
        Self {
            parser,
            input,
            _output: String::new(),
        }
    }

    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            match event::read()?.into() {
                Input {
                    key: Key::Char('q'),
                    ..
                } => break,
                Input {
                    key: Key::Enter, ..
                } => {
                    let input = self.input.lines()[0].as_str();
                    let _result = self.parser.parse(input);
                }
                input => {
                    self.input.input(input);
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, _frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]);
        let [grid_area, input_area] = layout.areas(_frame.area());

        // todo: implement grid area to display todos
        let paragraph = Paragraph::new("rusty todos . . . . .");
        _frame.render_widget(paragraph, grid_area);

        // todo: draw cursor when input is focused
        // todo: focus input when edit mode is selected
        _frame.render_widget(&self.input, input_area);
    }
}
