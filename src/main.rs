mod app;
mod cli;
mod core;
mod effects;
mod logging;
mod stats;
mod tui;
mod ui;

use clap::Parser;

use crate::app::App;
use crate::cli::Cli;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    // Session logging (Asciiverse): off unless RUST_LOG is set, e.g.
    // `RUST_LOG=debug blackguard 2>blackguard.log`.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("off")).init();
    log::info!(
        "blackguard {} started, {:?}",
        env!("CARGO_PKG_VERSION"),
        cli
    );
    let app = App::new(cli.resolve_seed(), cli.rule.into());
    tui::run(app)
}
