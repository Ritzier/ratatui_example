use ratatui::{
    crossterm::event::KeyCode,
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::Result;

use super::{Command, FocusedBlock};

pub struct Menu {
    list_items: Vec<String>,
    liststate: ListState,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            list_items: vec![String::from("tab1"), String::from("tab2")],
            liststate: ListState::default(),
        }
    }

    pub fn handle_event(&mut self, key: &KeyCode) -> Result<Command> {
        match key {
            KeyCode::Char('q') => Ok(Command::Quit),
            KeyCode::Up | KeyCode::Char('k') => {
                self.liststate.select_previous();
                Ok(Command::None)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.liststate.select_next();
                Ok(Command::None)
            }
            KeyCode::Enter => {
                if let Some(selected) = self.liststate.selected() {
                    match selected {
                        0 => Ok(Command::Switch(FocusedBlock::Tab1)),
                        1 => Ok(Command::Switch(FocusedBlock::Tab2)),
                        _ => Ok(Command::None),
                    }
                } else {
                    Ok(Command::None)
                }
            }
            _ => Ok(Command::None),
        }
    }

    pub fn render(&mut self, area: Rect, frame: &mut Frame) {
        let [title_layout, item_layout] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(area);

        render_title(title_layout, frame);
        self.render_items(item_layout, frame);
    }

    fn render_items(&mut self, area: Rect, frame: &mut Frame) {
        let items: Vec<ListItem> = self
            .list_items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Select an item")
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().reversed())
            .highlight_symbol(">>");

        frame.render_stateful_widget(list, area, &mut self.liststate);
    }
}

fn render_title(area: Rect, frame: &mut Frame) {
    let title = Paragraph::new("Menu").block(Block::default().borders(Borders::ALL).green());

    frame.render_widget(title, area);
}
