use color_eyre::Result;

use app::App;

mod app;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app = App::default().run(terminal);
    ratatui::restore();
    app
}
