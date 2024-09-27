use std::time::{Duration, Instant};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Paragraph, Widget},
    DefaultTerminal,
};

#[derive(Debug)]
struct App {
    quit: bool,
    spinner_index: usize,
    last_tick: Instant,
}

const SPIN_FRAME: [&str; 4] = ["|", "/", "-", "\\"];

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new().run(terminal);
    ratatui::restore();
    app
}

impl App {
    fn new() -> Self {
        Self {
            quit: false,
            spinner_index: 0,
            last_tick: Instant::now(),
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.quit {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.update_spinner();
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_millis(100);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    self.quit = true
                }
            }
        }
        Ok(())
    }

    fn update_spinner(&mut self) {
        if self.last_tick.elapsed() >= Duration::from_millis(100) {
            self.last_tick = Instant::now();
            self.spinner_index = (self.spinner_index + 1) % SPIN_FRAME.len();
        }
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
        let [title_bar, inner_bar] = layout.areas(area);
        render_title(title_bar, buf, &self.spinner_index.to_string());
        render_spin(inner_bar, buf, self.spinner_index);
    }
}

fn render_title(area: Rect, buf: &mut Buffer, title: &str) {
    Paragraph::new(title).render(area, buf)
}

fn render_spin(area: Rect, buf: &mut Buffer, spinner_index: usize) {
    let spinner = SPIN_FRAME[spinner_index];
    Paragraph::new(spinner)
        .alignment(Alignment::Right)
        .render(area, buf);
}
