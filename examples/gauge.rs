use std::time::Duration;

use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Gauge, Padding, Paragraph, Widget},
    DefaultTerminal,
};

const GAUGE1_COLOR: Color = tailwind::RED.c800;
const GAUGE2_COLOR: Color = tailwind::GREEN.c800;
const GAUGE3_COLOR: Color = tailwind::BLUE.c800;
const GAUGE4_COLOR: Color = tailwind::ORANGE.c800;
const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;

#[derive(Debug, Default, Clone, Copy)]
struct App {
    state: AppState,
    progress_columns: u16,
    progress1: u16,
    progress2: f64,
    progress3: f64,
    progress4: f64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Started,
    Paused,
    Quitting,
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
        while self.state != AppState::Quitting {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
            self.update(terminal.size()?.width)
        }
        Ok(())
    }

    fn update(&mut self, terminal_width: u16) {
        if self.state != AppState::Started {
            return;
        }

        // progress1 and progress2 help show the difference between ratio and percentage measuring
        // the same thing, but converting to either a u16 or f64. Effectively, we're showing the
        // difference between how a continous gauge acts for floor and rounded values.
        self.progress_columns = (self.progress_columns + 1).clamp(0, terminal_width);
        self.progress1 = self.progress_columns * 100 / terminal_width;
        self.progress2 = f64::from(self.progress_columns) * 100.0 / f64::from(terminal_width);

        // progress and progress4 similarly show the difference between unicode and non-unicode
        // gauges measuring the same thing.
        self.progress3 = (self.progress3 + 0.1).clamp(40.0, 100.0);
        self.progress4 = (self.progress4 + 0.1).clamp(40.0, 100.0)
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                        KeyCode::Char(' ') | KeyCode::Enter => self.toggle_start_pause(),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn toggle_start_pause(&mut self) {
        if self.state == AppState::Started {
            self.state = AppState::Paused
        } else {
            self.state = AppState::Started;
        }
    }

    fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);
        let [header_area, gauge_area, footer_area] = layout.areas(area);

        let layout = Layout::vertical([Constraint::Ratio(1, 4); 4]);
        let [gauge1_area, gauge2_area, gauge3_area, gauge4_area] = layout.areas(gauge_area);

        render_header(header_area, buf);
        render_footer(footer_area, buf);

        self.render_gauge1(gauge1_area, buf);
        self.render_gauge2(gauge2_area, buf);
        self.render_gauge3(gauge3_area, buf);
        self.render_gauge4(gauge4_area, buf);
    }
}

fn render_header(area: Rect, buf: &mut Buffer) {
    Paragraph::new("Ratatui Gauge Example")
        .bold()
        .alignment(Alignment::Center)
        .fg(CUSTOM_LABEL_COLOR)
        .render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    let footer_text = Line::from(vec![
        "Press ".into(),
        "<Space>".blue().bold(),
        " or ".into(),
        "<Enter>".blue().bold(),
        " to Start/Pause | Press ".into(),
        "<Esc>".blue().bold(),
        " or ".into(),
        "<q>".blue().bold(),
        " to Quit".into(),
    ]);

    Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .fg(CUSTOM_LABEL_COLOR)
        .render(area, buf);
}

impl App {
    fn render_gauge1(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with percentage");
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE1_COLOR)
            .percent(self.progress1)
            .render(area, buf);
    }

    fn render_gauge2(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio and custom label");
        let label = Span::styled(
            format!("{:.1}/100", self.progress2),
            Style::new().italic().bold().fg(CUSTOM_LABEL_COLOR),
        );
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE2_COLOR)
            .ratio(self.progress2 / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge3(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (no unicode)");
        let label = format!("{:.1}%", self.progress3);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE3_COLOR)
            .ratio(self.progress3 / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge4(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (unicode)");
        let label = format!("{:.1}%", self.progress4);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE4_COLOR)
            .ratio(self.progress4 / 100.0)
            .label(label)
            .use_unicode(true)
            .render(area, buf);
    }
}

fn title_block(title: &str) -> Block {
    let title = Line::from(title).centered();
    Block::new()
        .borders(Borders::NONE)
        .padding(Padding::vertical(1))
        .title(title)
        .fg(CUSTOM_LABEL_COLOR)
}
