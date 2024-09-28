use std::time::Duration;

use color_eyre::Result;
use rand::{thread_rng, Rng};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    style::{palette::tailwind, Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{block::Title, Block, BorderType, Borders, Gauge, Padding, Paragraph, Widget, Wrap},
    DefaultTerminal,
};

const PROGRESS_MAX: f64 = 100.0;
const GAUGE1_COLOR: Color = tailwind::RED.c800;
const GAUGE2_COLOR: Color = tailwind::GREEN.c800;
const GAUGE3_COLOR: Color = tailwind::BLUE.c800;
const GAUGE4_COLOR: Color = tailwind::ORANGE.c800;
const CUSTOM_LABEL_COLOR: Color = tailwind::SLATE.c200;

#[derive(Debug, Default, Clone, Copy)]
struct App {
    state: AppState,
    progress: f64,
    pop_help: bool,
    pop_quit: bool,
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
            self.update()
        }
        Ok(())
    }

    fn update(&mut self) {
        if self.state != AppState::Started {
            return;
        }

        let mut rng = thread_rng();
        self.progress += rng.gen_range(0.5..1.0);
        self.progress = self.progress.clamp(0.0, PROGRESS_MAX)
    }

    fn handle_events(&mut self) -> Result<()> {
        let timeout = Duration::from_secs_f32(1.0 / 20.0);
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if self.pop_quit {
                        match key.code {
                            KeyCode::Char('y') | KeyCode::Char('q') => self.quit(),
                            _ => self.pop_quit = false,
                        }
                    } else {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.popup_quit(),
                            KeyCode::Char(' ') | KeyCode::Enter => self.toggle_start_pause(),
                            KeyCode::Char('?') => self.toggle_help(),
                            KeyCode::Char('r') => self.reset_progress(),
                            _ => {}
                        }
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

    fn toggle_help(&mut self) {
        self.pop_help = !self.pop_help;
    }

    fn reset_progress(&mut self) {
        self.progress = 0.0;
    }

    fn popup_quit(&mut self) {
        self.pop_quit = !self.pop_quit
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

        self.render_help_popup(area, buf);
        self.render_quit_popup(area, buf);
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
        " to Quit | Press ".into(),
        "<r>".blue().bold(),
        " to reset progress".into(),
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
            .percent(self.progress as u16)
            .render(area, buf);
    }

    fn render_gauge2(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio and custom label");
        let label = Span::styled(
            format!("{:.1}/100", self.progress),
            Style::new().italic().bold().fg(CUSTOM_LABEL_COLOR),
        );
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE2_COLOR)
            .ratio(self.progress / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge3(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (no unicode)");
        let label = format!("{:.1}%", self.progress);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE3_COLOR)
            .ratio(self.progress / 100.0)
            .label(label)
            .render(area, buf);
    }

    fn render_gauge4(&self, area: Rect, buf: &mut Buffer) {
        let title = title_block("Gauge with ratio (unicode)");
        let label = format!("{:.1}%", self.progress);
        Gauge::default()
            .block(title)
            .gauge_style(GAUGE4_COLOR)
            .ratio(self.progress / 100.0)
            .label(label)
            .use_unicode(true)
            .render(area, buf);
    }

    fn render_help_popup(&self, area: Rect, buf: &mut Buffer) {
        if self.pop_help {
            let popup_area = popup_area(area, 40, 40);
            Clear.render(popup_area, buf);

            let key_style = Style::new().dark_gray();
            let desc_style = Style::new().bold().blue();

            let text = vec![
                "".into(),
                Line::from(Span::styled("Keyboard ShortCut", Style::new().green())),
                "".into(),
                Line::from(vec![
                    Span::styled("<Space> | <Enter> ", key_style),
                    Span::styled("Toggle Progress", desc_style),
                ]),
                "".into(),
                Line::from(vec![
                    Span::styled("<Esc> | <q>       ", key_style),
                    Span::styled("Quit", desc_style),
                ]),
                "".into(),
                Line::from(vec![
                    Span::styled("<r>               ", key_style),
                    Span::styled("Reset Progress", desc_style),
                ]),
            ];

            Paragraph::new(text)
                .block(
                    Block::default()
                        .title(Title::from("Title").alignment(Alignment::Center))
                        .border_style(Style::default().fg(Color::Red))
                        .border_type(BorderType::Rounded)
                        .borders(Borders::ALL),
                )
                .alignment(Alignment::Center)
                .render(popup_area, buf)
        }
    }

    fn render_quit_popup(&self, area: Rect, buf: &mut Buffer) {
        if self.pop_quit {
            let text = Line::from(vec![
                Span::styled("Are you sure want to quit? ", Style::default().bold()),
                Span::styled("y/n", Style::default().gray()),
            ]);

            let text_width = text.width() as u16;

            let block_area = Rect::new(
                (area.width.saturating_sub(text_width + 2)) / 2,
                area.height / 2,
                text_width + 2,
                3,
            );
            Clear.render(block_area, buf);

            Paragraph::new(text)
                .block(
                    Block::default()
                        .title(Title::from("Quit?").alignment(Alignment::Left))
                        .border_style(Style::default().fg(Color::Red))
                        .border_type(BorderType::Thick)
                        .borders(Borders::ALL),
                )
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true })
                .render(block_area, buf);
        }
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

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let [area] = horizontal.areas(area);
    let [area] = vertical.areas(area);
    area
}

struct Clear;

impl Widget for Clear {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                buf.get_mut(x, y).reset();
            }
        }
    }
}
