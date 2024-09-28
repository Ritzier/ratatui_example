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
    spin_list: Vec<Spin>,
    completed_count: usize,
    maxinum_spin: usize,
}

#[derive(Debug, Clone, Copy)]
struct Spin {
    spinner_index: usize,
    last_tick: Instant,
    start_time: Instant,
}

impl Spin {
    fn new() -> Self {
        Self {
            spinner_index: 0,
            last_tick: Instant::now(),
            start_time: Instant::now(),
        }
    }

    fn update(&mut self) {
        if self.last_tick.elapsed() >= Duration::from_millis(100) {
            self.last_tick = Instant::now();
            self.spinner_index = (self.spinner_index + 1) % SPIN_FRAME.len();
        }
    }

    fn render(&self) -> &str {
        SPIN_FRAME[self.spinner_index]
    }

    fn has_expired(&self) -> bool {
        self.start_time.elapsed() >= Duration::from_secs(3)
    }
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
            spin_list: vec![Spin::new(), Spin::new()],
            completed_count: 0,
            maxinum_spin: 5,
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
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => self.quit = true,
                        KeyCode::Char('a') => self.add_spinner(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn add_spinner(&mut self) {
        if self.spin_list.len() < self.maxinum_spin {
            self.spin_list.push(Spin::new())
        }
    }

    fn update_spinner(&mut self) {
        for spin in &mut self.spin_list {
            spin.update();
        }

        self.spin_list.retain(|spin| {
            if spin.has_expired() {
                self.completed_count += 1;
                false
            } else {
                true
            }
        });
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [title_area, content_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(area);

        self.render_title(title_area, buf);
        self.render_spinners(content_area, buf);
    }
}

impl App {
    fn render_title(&self, area: Rect, buf: &mut Buffer) {
        let text = format!("{}", self.completed_count);
        Paragraph::new(text).centered().render(area, buf);
    }

    fn render_spinners(&self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical(
            self.spin_list
                .iter()
                .map(|_| Constraint::Length(1)) // Each spinner takes 1 line
                .collect::<Vec<_>>(),
        )
        .split(area);

        // Render each spinner
        for (i, spin) in self.spin_list.iter().enumerate() {
            render_spin(layout[i], buf, spin);
        }
    }
}

fn render_spin(area: Rect, buf: &mut Buffer, spin: &Spin) {
    let spinner = spin.render();
    Paragraph::new(spinner)
        .alignment(Alignment::Center) // Align each spinner to the center
        .render(area, buf);
}
