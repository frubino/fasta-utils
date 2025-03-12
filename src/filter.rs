use crate::{
    cli,
    utils::{file_or_stdin, file_or_stdout},
};
use anyhow::{Context, Result};
use bio_rascal::fasta::FastaReader;
pub fn filter_command(options: cli::Filter) -> Result<()> {
    let input_file = file_or_stdin(&options.input_file).context("Problem opening input file")?;
    let mut output_file =
        file_or_stdout(&options.output_file).context("Problem opening output file")?;

    let fasta_reader = FastaReader::from_reader(input_file);

    for seq_record in fasta_reader {
        match options.length_filter {
            cli::LengthFilter::Eq => {
                if seq_record.seq.len() != options.length {
                    continue;
                }
            }
            cli::LengthFilter::Ge => {
                if seq_record.seq.len() < options.length {
                    continue;
                }
            }
            cli::LengthFilter::Gt => {
                if seq_record.seq.len() <= options.length {
                    continue;
                }
            }
            cli::LengthFilter::Le => {
                if seq_record.seq.len() > options.length {
                    continue;
                }
            }
            cli::LengthFilter::Lt => {
                if seq_record.seq.len() >= options.length {
                    continue;
                }
            }
        };
        seq_record.to_file(&mut output_file)?;
    }
    Ok(())
}
