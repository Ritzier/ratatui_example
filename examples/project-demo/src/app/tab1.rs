use ratatui::{crossterm::event::KeyCode, layout::Rect, widgets::Paragraph, Frame};

use super::{Command, FocusedBlock, Result};

pub struct Tab1 {}

impl Tab1 {
    pub fn handle_key(&mut self, key: &KeyCode) -> Result<Command> {
        match key {
            KeyCode::Char('q') => Ok(Command::Switch(FocusedBlock::Menu)),
            _ => Ok(Command::None),
        }
    }

    pub fn render(&self, area: Rect, frame: &mut Frame) {
        let paragraph = Paragraph::new("Tab1");
        frame.render_widget(paragraph, area)
    }
}
