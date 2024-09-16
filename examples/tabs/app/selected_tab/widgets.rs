use ratatui::buffer::Buffer;
use ratatui::style::palette::tailwind;
use ratatui::style::Stylize;
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::{layout::Rect, widgets::Widget};

use super::SelectedTab;

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Self::Tab1 => self.render_tab0(area, buf),
            Self::Tab2 => self.render_tab1(area, buf),
            Self::Tab3 => self.render_tab2(area, buf),
            Self::Tab4 => self.render_tab3(area, buf),
        }
    }
}

impl SelectedTab {
    pub fn title(self) -> Line<'static> {
        format!("  {self}  ").fg(tailwind::SLATE.c200).into()
    }

    fn render_tab0(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Here is the first tab!")
            .block(self.block())
            .render(area, buf)
    }

    fn render_tab1(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Here is the second tab!")
            .block(self.block())
            .render(area, buf)
    }

    fn render_tab2(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Here is the third tab!")
            .block(self.block())
            .render(area, buf)
    }

    fn render_tab3(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Here is the fourth tab")
            .block(self.block())
            .render(area, buf)
    }

    fn block(self) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::PROPORTIONAL_TALL)
            .padding(Padding::horizontal(1))
            .border_style(self.palette().c700)
    }

    pub const fn palette(self) -> tailwind::Palette {
        match self {
            Self::Tab1 => tailwind::BLUE,
            Self::Tab2 => tailwind::EMERALD,
            Self::Tab3 => tailwind::INDIGO,
            Self::Tab4 => tailwind::RED,
        }
    }
}
