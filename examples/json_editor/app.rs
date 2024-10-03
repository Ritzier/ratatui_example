use std::collections::HashMap;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::Backend,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Widget, Wrap},
    Terminal,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

type Result<T> = std::result::Result<T, Error>;

#[derive(PartialEq, Eq)]
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,
    pub value_input: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>,
}

impl App {
    pub fn default() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn run<B: Backend>(mut self, terminal: &mut Terminal<B>) -> Result<()> {
        while self.current_screen != CurrentScreen::Exiting {
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            self.event_handlers()?;
        }

        Ok(())
    }

    fn event_handlers(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match self.current_screen {
                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('e') => {
                            self.current_screen = CurrentScreen::Editing;
                            self.currently_editing = Some(CurrentlyEditing::Key);
                        }
                        KeyCode::Char('q') => self.current_screen = CurrentScreen::Exiting,
                        _ => {}
                    },
                    CurrentScreen::Exiting => match key.code {
                        KeyCode::Char('y') => self.current_screen = CurrentScreen::Exiting,
                        _ => {}
                    },
                    CurrentScreen::Editing => match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        self.currently_editing = Some(CurrentlyEditing::Value)
                                    }
                                    CurrentlyEditing::Value => {
                                        self.save_key_value();
                                        self.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        self.key_input.pop();
                                    }
                                    CurrentlyEditing::Value => {
                                        self.value_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            self.current_screen = CurrentScreen::Main;
                            self.currently_editing = None;
                        }
                        KeyCode::Char(value) => {
                            if let Some(editing) = &self.currently_editing {
                                match editing {
                                    CurrentlyEditing::Key => {
                                        self.key_input.push(value);
                                    }
                                    CurrentlyEditing::Value => {
                                        self.value_input.push(value);
                                    }
                                }
                            }
                        }
                        KeyCode::Tab => {
                            self.toggle_editing();
                        }

                        _ => {}
                    },
                }
            }
        }
        Ok(())
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key)
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [title_block, containt, bottom_block] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .areas(area);

        App::render_title(title_block, buf);
        self.render_containt(containt, buf);
        self.render_bottom(bottom_block, buf);

        self.render_popup_editing(area, buf);
        self.render_popup_exit(area, buf);
    }
}

impl App {
    fn render_title(area: Rect, buf: &mut Buffer) {
        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Create New Json",
            Style::default().fg(Color::Green),
        ))
        .block(title_block);

        title.render(area, buf);
    }

    fn render_containt(&self, area: Rect, buf: &mut Buffer) {
        let mut list_items = Vec::<ListItem>::new();

        for key in self.pairs.keys() {
            list_items.push(ListItem::new(Line::from(Span::styled(
                format!("{: <25} : {}", key, self.pairs.get(key).unwrap()),
                Style::default().fg(Color::Yellow),
            ))));
        }

        List::new(list_items).render(area, buf);
    }

    fn render_bottom(&self, area: Rect, buf: &mut Buffer) {
        let current_navigation_text = vec![
            match self.current_screen {
                CurrentScreen::Main => {
                    Span::styled("Normal Mode", Style::default().fg(Color::Green))
                }
                CurrentScreen::Editing => {
                    Span::styled("Editing Mode", Style::default().fg(Color::Yellow))
                }
                CurrentScreen::Exiting => Span::styled("Exiting", Style::default().fg(Color::Red)),
            }
            .to_owned(),
            Span::styled(" | ", Style::default().fg(Color::White)),
            {
                if let Some(editing) = &self.currently_editing {
                    match editing {
                        CurrentlyEditing::Key => {
                            Span::styled("Editing Json Key", Style::default().fg(Color::Green))
                        }
                        CurrentlyEditing::Value => Span::styled(
                            "Editing Json Value",
                            Style::default().fg(Color::LightGreen),
                        ),
                    }
                } else {
                    Span::styled("Not Editing Anything", Style::default().fg(Color::DarkGray))
                }
            },
        ];

        let mode_footer = Paragraph::new(Line::from(current_navigation_text))
            .block(Block::default().borders(Borders::ALL));

        let current_keys_hint = {
            match self.current_screen {
                CurrentScreen::Main => Span::styled(
                    "(q) to quit / (e) to make new pair",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Editing => Span::styled(
                    "(ESC) to cancel / (TAB) to switch boxes/enter to complete",
                    Style::default().fg(Color::Red),
                ),
                CurrentScreen::Exiting => Span::styled(
                    "(q) to quit / (e) to make new pair",
                    Style::default().fg(Color::Red),
                ),
            }
        };

        let keys_notes_footer = Paragraph::new(Line::from(current_keys_hint))
            .block(Block::default().borders(Borders::ALL));

        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        mode_footer.render(footer_chunks[0], buf);
        keys_notes_footer.render(footer_chunks[1], buf);
    }

    fn render_popup_editing(&self, area: Rect, buf: &mut Buffer) {
        if let Some(editing) = &self.currently_editing {
            let popup_block = Block::default()
                .title("Enter a new key-value pair")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let area = centered_rect(60, 25, area);
            popup_block.render(area, buf);

            let popup_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(area);

            let mut key_block = Block::default().title("Key").borders(Borders::ALL);
            let mut value_block = Block::default().title("Value").borders(Borders::ALL);

            let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

            match &editing {
                CurrentlyEditing::Key => key_block = key_block.style(active_style),
                CurrentlyEditing::Value => value_block = value_block.style(active_style),
            }

            let key_text = Paragraph::new(self.key_input.clone()).block(key_block);
            key_text.render(popup_chunks[0], buf);

            let value_text = Paragraph::new(self.value_input.clone()).block(value_block);
            value_text.render(popup_chunks[1], buf);
        }
    }

    fn render_popup_exit(&self, area: Rect, buf: &mut Buffer) {
        if let CurrentScreen::Exiting = self.current_screen {
            Clear.render(area, buf);

            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Would you like to output the buffer as json? (y/n)",
                Style::default().fg(Color::Red),
            );

            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });

            let area = centered_rect(60, 25, area);
            exit_paragraph.render(area, buf);
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
