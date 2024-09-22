use color_eyre::Result;
use rand::{thread_rng, Rng};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Bar, BarChart, BarGroup, Block},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::new().run(terminal);
    ratatui::restore();
    app
}

struct App {
    should_exit: bool,
    temperatures: Vec<u8>,
}

impl App {
    fn new() -> Self {
        let mut rng = thread_rng();
        let temperatures = (0..24).map(|_| rng.gen_range(50..90)).collect();
        Self {
            should_exit: false,
            temperatures,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                self.should_exit = true;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let [title, vertical, horizontal] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .spacing(1)
        .areas(frame.area());

        frame.render_widget("Barchar".bold().into_centered_line(), title);
        frame.render_widget(vertical_barchart(&self.temperatures), vertical);
        frame.render_widget(horizontal_barchart(&self.temperatures), horizontal);
    }
}

fn vertical_bar(hour: usize, temperature: &u8) -> Bar {
    Bar::default()
        .value(u64::from(*temperature))
        .label(Line::from(format!("{hour:>02}:00")))
        .text_value(format!("{temperature:>3}°"))
        .style(temperature_style(*temperature))
        .value_style(temperature_style(*temperature).reversed())
}

fn vertical_barchart(temperatures: &[u8]) -> BarChart {
    let bars: Vec<Bar> = temperatures
        .iter()
        .enumerate()
        .map(|(hour, value)| vertical_bar(hour, value))
        .collect();
    let title = Line::from("Weather (Vertical)").centered();
    BarChart::default()
        .block(Block::new().title(title))
        .data(BarGroup::default().bars(&bars))
        .bar_width(5)
        .bar_gap(1)
        .direction(Direction::Vertical)
}

fn horizontal_barchart(temperatures: &[u8]) -> BarChart {
    let bars: Vec<Bar> = temperatures
        .iter()
        .enumerate()
        .map(|(hour, value)| horizontal_bar(hour, value))
        .collect();
    let title = Line::from("Weather (Horizontal)").centered();
    BarChart::default()
        .block(Block::new().title(title))
        .data(BarGroup::default().bars(&bars))
        .bar_width(1)
        .bar_gap(0)
        .direction(Direction::Horizontal)
}

fn horizontal_bar(hour: usize, temperature: &u8) -> Bar {
    let style = temperature_style(*temperature);
    Bar::default()
        .value(u64::from(*temperature))
        .label(Line::from(format!("{hour:>02}:00")))
        .text_value(format!("{temperature:>3}°"))
        .style(style)
        .value_style(style.reversed())
}

/// create a yellow to red value based on the value (50-90)
fn temperature_style(value: u8) -> Style {
    let green = (255.0 * (1.0 - f64::from(value - 50) / 40.0)) as u8;
    let color = Color::Rgb(255, green, 0);
    Style::new().fg(color)
}
