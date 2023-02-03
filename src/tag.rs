
use crate::{cli, utils::{file_or_stdin, file_or_stdout}};
use anyhow::{Result, Context};

pub fn tag_command(options: cli::Tag) -> Result<()> {
    
    let input_file = file_or_stdin(&options.input_file).context("Problem opening input file")?;
    let output_file = file_or_stdout(&options.output_file).context("Problem opening output file")?;
    
    
    Ok(())
}
