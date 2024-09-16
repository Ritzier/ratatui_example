use appstate::AppState;
use ratatui::DefaultTerminal;
use selected_tab::SelectedTab;

mod appstate;
mod handle;
mod selected_tab;
mod widgets;

#[derive(Default)]
pub struct App {
    selected_tab: SelectedTab,
    appstate: AppState,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running() {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_event()?;
        }
        Ok(())
    }

    fn running(&self) -> bool {
        self.appstate.is_running()
    }

    pub fn quit(&mut self) {
        self.appstate = AppState::Quitting
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }
}
