mod cli;
mod forge;

use clap::Parser;

// Parse cli params and call associated commands
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    match cli.command {
        cli::Commands::Csv {
            path,
            lines,
            delim,
            columns,
        } => {
            forge::csv::write(&path, lines, delim, columns)?;
            println!("CSV File generated");
        }
        cli::Commands::Xml {
            path,
            records,
            columns,
        } => {
            forge::xml::write(&path, records, columns)?;
            println!("XML File generated");
        }
    }
    Ok(())
}
