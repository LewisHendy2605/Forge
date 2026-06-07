mod cli;
mod forge;

use clap::Parser;

// Parse cli params and call associated commands
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();
    let start = std::time::Instant::now();

    match cli.command {
        cli::Commands::Csv {
            path,
            records,
            delim,
            columns,
        } => {
            forge::csv::write(&path, records, delim, columns)?;
        }
        cli::Commands::Xml {
            path,
            records,
            columns,
        } => {
            forge::xml::write(&path, records, columns)?;
        }
    }

    println!("Time Elapsed: {:.2?}", start.elapsed());
    Ok(())
}
