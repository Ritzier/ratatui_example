use anyhow::{Context, Result};
use demo2::Tui;

// Original Repository: https://github.com/d-holguin/async-ratatui
#[tokio::main]
async fn main() -> Result<()> {
    if let Err(e) = run_app().await {
        println!("application exited with error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}

pub async fn run_app() -> Result<()> {
    let mut app =
        Tui::new(60.0, 10.0).context("Failed to initialize the terminal user interface (TUI)")?;
    app.run().await?;
    Ok(())
}
