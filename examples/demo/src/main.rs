use clap::Parser;
use cli::Cli;

mod action;
mod app;
mod cli;
mod config;
mod errors;
mod logging;
mod tui;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    crate::logging::init()?;
    crate::errors::init()?;

    let _args = Cli::parse();

    println!("hi");
    Ok(())
}
