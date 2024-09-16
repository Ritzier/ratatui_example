use std::time::Duration;

use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        style::Color,
    },
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Block, BorderType, Widget},
    DefaultTerminal,
};

#[derive(Default)]
struct App {
    quit: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::default().run(terminal);
    ratatui::restore();
    app
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.quit {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_event()?;
        }
        Ok(())
    }

    fn handle_event(&mut self) -> Result<()> {
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
        //match key.code {
        //    KeyCode::Char('q') => self.quit(),
        //    _ => {}
        //}

        if let KeyCode::Char('q') = key.code {
            self.quit()
        }
    }

    fn quit(&mut self) {
        self.quit = true
    }
}

// UI
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let vertical = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [title_bar, inner, footer_bar] = vertical.areas(area);
        self.render_title_bar(title_bar, buf);
        self.render_inner(inner, buf);
        self.render_footer_bar(footer_bar, buf);
    }
}

impl App {
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        "Title".bold().render(area, buf)
    }

    fn render_inner(&self, area: Rect, buf: &mut Buffer) {
        //"Content".render(area, buf)
        Span::default()
            .content("Content")
            .into_centered_line()
            .render(area, buf);
    }

    fn render_footer_bar(&self, area: Rect, buf: &mut Buffer) {
        "Footer".render(area, buf)
    }
}
