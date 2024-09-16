use std::time::Duration;

use color_eyre::Result;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use super::{selected_tab::SelectedTab, App};

impl App {
    pub fn handle_event(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(());
        }
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => {}
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('1') => self.selected_tab = SelectedTab::Tab1,
            KeyCode::Char('2') => self.selected_tab = SelectedTab::Tab2,
            KeyCode::Char('3') => self.selected_tab = SelectedTab::Tab3,
            KeyCode::Char('4') => self.selected_tab = SelectedTab::Tab4,

            KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
            KeyCode::Char('l') | KeyCode::Right => self.next_tab(),

            KeyCode::Char('q') => self.quit(),
            _ => {}
        }
    }
}
