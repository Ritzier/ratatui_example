mod action;
mod app;
mod config;
mod logging;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    crate::logging::init()?;

    Ok(())
}
