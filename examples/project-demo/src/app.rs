use ratatui::{
    crossterm::{
        self,
        event::{self, Event, KeyCode, KeyEventKind},
    },
    layout::Rect,
    DefaultTerminal, Frame,
};

use crate::Result;

use menu::Menu;
use tab1::Tab1;
use tab2::Tab2;

mod menu;
mod tab1;
mod tab2;

enum Command {
    Quit,
    Switch(FocusedBlock),
    None,
}

#[derive(PartialEq)]
enum FocusedBlock {
    Menu,
    Tab1,
    Tab2,
}

pub struct App {
    should_quit: bool,
    focused_block: FocusedBlock,
    menu: Menu,
    tab1: Tab1,
    tab2: Tab2,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            should_quit: false,
            focused_block: FocusedBlock::Menu,
            menu: Menu::new(),
            tab1: Tab1 {},
            tab2: Tab2 {},
        })
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        startup()?;

        while !self.should_quit {
            terminal.draw(|frame| {
                self.render(frame.area(), frame);
            })?;

            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                self.handle_key_event(&key.code)?;
            }
        }

        shutdown()
    }

    fn handle_key_event(&mut self, key: &KeyCode) -> Result<()> {
        let command = match self.focused_block {
            FocusedBlock::Menu => self.menu.handle_event(key)?,
            FocusedBlock::Tab1 => self.tab1.handle_key(key)?,
            FocusedBlock::Tab2 => self.tab2.handle_key(key)?,
        };

        match command {
            Command::Quit => self.should_quit = true,
            Command::Switch(tab) => self.focused_block = tab,
            Command::None => {}
        }

        Ok(())
    }

    fn render(&mut self, area: Rect, frame: &mut Frame) {
        match self.focused_block {
            FocusedBlock::Menu => self.menu.render(area, frame),
            FocusedBlock::Tab1 => self.tab1.render(area, frame),
            FocusedBlock::Tab2 => self.tab2.render(area, frame),
        }
    }
}

fn startup() -> Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen)?;
    Ok(())
}

fn shutdown() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
