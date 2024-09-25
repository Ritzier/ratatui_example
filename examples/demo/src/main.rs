mod action;
mod app;
mod config;
mod errors;
mod logging;
mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    crate::logging::init()?;
    crate::errors::init()?;

    println!("hi");
    Ok(())
}
