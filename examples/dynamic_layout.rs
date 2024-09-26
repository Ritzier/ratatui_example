use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Paragraph, Widget},
    DefaultTerminal,
};

#[derive(Default)]
struct App {
    should_quit: bool,
}

//impl Widget for &App {
//    fn render(&self, area: Rect, buf: &mut Buffer) {
//
//    }
//}

impl App {
    fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| {
                frame.render_widget(Paragraph::new("Hello world!"), frame.area());
            })?;

            self.handle_event()?;
        }

        Ok(())
    }

    fn handle_event(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press || key.code == KeyCode::Char('q') {
                self.should_quit = true
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::default().run(terminal);
    ratatui::restore();
    app
}
