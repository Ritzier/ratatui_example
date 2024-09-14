use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    widgets::{Block, Clear, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

#[derive(Default)]
pub struct App {
    show_popup: bool,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('p') => self.show_popup = !self.show_popup,
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let vertical = Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [instructions, content] = vertical.areas(area);

        let text = if self.show_popup {
            "Press p to close the popup"
        } else {
            "Press p to open the popup"
        };
        let paragraph = Paragraph::new(text.slow_blink())
            .centered()
            .wrap(Wrap { trim: true });
        frame.render_widget(paragraph, instructions);

        let block = Block::bordered().title("Content").on_blue();
        frame.render_widget(block, content);

        if self.show_popup {
            let block = Block::bordered().title("Popup");
            let area = popup_area(area, 60, 20);
            frame.render_widget(Clear, area);
            frame.render_widget(block, area);
        }
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
