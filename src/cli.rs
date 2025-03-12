use anyhow::{bail, Result};
use clap::{Args, Command, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Generator, Shell};
use std::path::PathBuf;

/// Fasta Utilities
#[derive(Parser, Debug)]
#[command(author, version, about, arg_required_else_help(true))]
pub struct Cli {
    /// Generates Shell completion code
    ///
    /// It prints the code to the standard output and the way to
    /// use depends on the Shell. For Fish, redirect to a file
    /// with `.fish` extension in `~/.config/fish/completion`.
    #[arg(long)]
    pub complete: Option<Shell>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Tag(Tag),
    Filter(Filter),
}

fn key_value_parser(arg: &str) -> Result<(String, String)> {
    match arg.split_once('=') {
        None => bail!("Cannot parse 'key:value' argument: {}", arg),
        Some((key, value)) => Ok((key.into(), value.into())),
    }
}

/// Adds tags or a random identifier to a sequence header
///
/// The random string is composed of alphanumeric characters
/// while the tags will added as `tag=value`. The original
/// header is split at the first `space` character the random
/// string is added after a `|`. Each tag is then separated by
/// a space.
#[derive(Args, Debug)]
pub struct Tag {
    #[arg(short, long)]
    pub add_random: bool,
    #[arg(short, long, default_value_t = 5)]
    pub random_length: u8,
    /// Tag and value to add, expected in the tag=value
    #[arg(short, long, value_delimiter = ',', value_parser = key_value_parser)]
    pub tags: Vec<(String, String)>,
    /// Input file
    pub input_file: Option<PathBuf>,
    /// Output file
    pub output_file: Option<PathBuf>,
}

/// Command used to filter fastq sequences
#[derive(Args, Debug)]
pub struct Filter {
    #[arg(short, long, default_value_t = 0)]
    pub length: usize,
    #[arg(short = 'f', long, value_enum, default_value_t = LengthFilter::Ge)]
    pub length_filter: LengthFilter,
    /// Input file
    pub input_file: Option<PathBuf>,
    /// Output file
    pub output_file: Option<PathBuf>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum LengthFilter {
    /// Greater than or equal
    Ge,
    /// Greater than
    Gt,
    /// Less than
    Lt,
    /// Less than or equal
    Le,
    /// Equal
    Eq,
}

/// Generates the completion for the specified shell
///
/// Slightly modified from example
pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
