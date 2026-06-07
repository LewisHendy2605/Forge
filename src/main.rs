mod cli;
mod commands;

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
            commands::csv::write_csv(&path, lines, delim, columns)?;
            println!("CSV File generated");
        }
        cli::Commands::Xml { path: _ } => {
            println!("XML File generated");
        }
    }
    Ok(())
}
