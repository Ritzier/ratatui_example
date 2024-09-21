use ratatui::crossterm::event::{MouseEvent, MouseEventKind};

use crate::tui::State;

pub fn handle_mouse_event(
    mouse: MouseEvent,
    button_states: &mut [State; 3],
    selected_button: &mut usize,
) {
    match mouse.kind {
        MouseEventKind::Moved => {
            let old_selected_button = *selected_button;
            // Get button from mouse click position x
            *selected_button = match mouse.column {
                x if x < 15 => 0,
                x if x < 30 => 1,
                _ => 2,
            };

            if old_selected_button != *selected_button {
                if button_states[old_selected_button] != State::Activate {}
            }
        }

        _ => (),
    }
}
