mod cli;
mod filter;
mod tag;
mod utils;

use anyhow::Result;
use clap::{CommandFactory, Parser};
use cli::print_completions;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    if let Some(generator) = cli.complete {
        let mut cmd = cli::Cli::command();
        print_completions(generator, &mut cmd);
    } else if let Some(command) = cli.command {
        // Starts the logging
        // possible to define log level with RUST_LOG
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
            .format_timestamp_millis()
            .init();
        let result = match command {
            cli::Commands::Tag(options) => tag::tag_command(options),
            cli::Commands::Filter(options) => filter::filter_command(options),
            _ => todo!(),
        };

        return result;
    }

    Ok(())
}
