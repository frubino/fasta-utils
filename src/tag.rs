
use rand::{distributions::Alphanumeric, Rng};
use crate::{cli, utils::{file_or_stdin, file_or_stdout}};
use anyhow::{Result, Context};
use bio_rascal::fasta::FastaReader;

pub fn tag_command(options: cli::Tag) -> Result<()> {
    
    let input_file = file_or_stdin(&options.input_file).context("Problem opening input file")?;
    let mut output_file = file_or_stdout(&options.output_file).context("Problem opening output file")?;
    
    let fasta_reader = FastaReader::from_reader(input_file);
    
    for mut record in fasta_reader {
        if options.add_random {
            record.id.push('|');
            for c in  rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(options.random_length as usize)
                .map(char::from) {
                    record.id.push(c);
                }
        }
        if !options.tags.is_empty() {
            for tag in &options.tags {
                if !record.attributes.is_empty() {
                    record.attributes.push(' ');
                }
                record.attributes.push_str(tag.0.as_str());
                record.attributes.push('=');
                record.attributes.push_str(tag.1.as_str());
            }
        }
        record.to_file(&mut output_file)?;
    }
    
    Ok(())
}
