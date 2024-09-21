use ratatui::style::Color;
use ratatui::text::Line;

#[derive(Debug, Clone)]
pub struct Button<'a> {
    pub label: Line<'a>,
    pub state: State,
    pub theme: ButtonTheme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal,
    Selected,
    Active,
}

#[derive(Debug, Clone, Copy)]
pub struct ButtonTheme {
    pub text: Color,
    pub background: Color,
    pub highlight: Color,
    pub shadow: Color,
}

impl<'a> Button<'a> {
    /// Return a Button with initial state: State::Normal
    pub fn new<T: Into<Line<'a>>>(label: T, theme: ButtonTheme, state: State) -> Self {
        Self {
            label: label.into(),
            theme,
            state,
        }
    }
}

pub const BLUE: ButtonTheme = ButtonTheme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight: Color::Rgb(64, 96, 192),
    shadow: Color::Rgb(32, 48, 96),
};

pub const RED: ButtonTheme = ButtonTheme {
    text: Color::Rgb(48, 16, 16),
    background: Color::Rgb(144, 48, 48),
    highlight: Color::Rgb(192, 64, 64),
    shadow: Color::Rgb(96, 32, 32),
};

pub const GREEN: ButtonTheme = ButtonTheme {
    text: Color::Rgb(16, 48, 16),
    background: Color::Rgb(48, 144, 48),
    highlight: Color::Rgb(64, 192, 64),
    shadow: Color::Rgb(32, 96, 32),
};
