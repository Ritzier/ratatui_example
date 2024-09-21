use color_eyre::Result;
use tui::run;

mod tui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = run(terminal);
    ratatui::restore();
    app
}
