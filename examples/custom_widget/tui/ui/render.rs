use std::time::Duration;

use ratatui::{
    crossterm::event::{self, Event},
    layout::{Constraint, Layout, Rect},
    widgets::Paragraph,
    DefaultTerminal, Frame,
};

use super::handle_key_event;
use crate::tui::{Button, State, BLUE, GREEN, RED};

pub fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut selected_button: usize = 0;
    let mut button_states = [State::Selected, State::Normal, State::Normal];
    loop {
        terminal.draw(|frame| draw(frame, button_states))?;
        if !event::poll(Duration::from_millis(100))? {
            continue;
        }
        match event::read()? {
            Event::Key(key) => {
                if key.kind != event::KeyEventKind::Press {
                    continue;
                }
                if handle_key_event(key, &mut button_states, &mut selected_button).is_break() {
                    break;
                }
            }
            _ => (),
        }
    }

    Ok(())
}

fn draw(frame: &mut Frame<'_>, states: [State; 3]) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Max(3),
        Constraint::Length(1),
        Constraint::Min(0),
    ]);
    let [title, buttons, help, _] = vertical.areas(frame.area());
    frame.render_widget(
        Paragraph::new("Custom Widget Example (mouse enabled)"),
        title,
    );
    render_buttons(frame, buttons, states);
    frame.render_widget(Paragraph::new("←/→: select, Space: toggle, q: quit"), help)
}

fn render_buttons(frame: &mut Frame<'_>, area: Rect, states: [State; 3]) {
    let horizontal = Layout::horizontal([
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Length(15),
        Constraint::Min(0), // ignore remaining area(15),
    ]);
    let [red, green, blue, _] = horizontal.areas(area);

    frame.render_widget(Button::new("Red", RED, states[0]), red);
    frame.render_widget(Button::new("Green", GREEN, states[1]), green);
    frame.render_widget(Button::new("Blue", BLUE, states[2]), blue);
}
