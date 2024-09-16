mod render;
mod state;
mod ui;

use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    DefaultTerminal,
};

use state::AppState;
use ui::SelectedTab;

#[derive(Default)]
pub struct App {
    state: AppState,
    selected_tab: SelectedTab,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('q') => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next()
    }

    fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous()
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting
    }
}
