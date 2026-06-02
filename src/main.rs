use clap::{Parser, Subcommand};
use fake::Fake;
use fake::faker::address::en::*;
use fake::faker::internet::en::*;
use fake::faker::name::en::*;
use fake::uuid::UUIDv4;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

#[derive(Parser)]
#[command(name = "datagen", about = "Generate fake data files")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Csv {
        #[arg(short, long, default_value = "output.csv")]
        path: String,

        #[arg(short, long, default_value_t = 10)]
        lines: u32,

        #[arg(short, long, default_value = ",")]
        delim: String,

        #[arg(
            short,
            long,
            default_value = "id:uuid,name:first_name,email:email",
            value_delimiter = ','
        )]
        columns: Vec<String>,
    },

    Xml {
        #[arg(short, long, default_value = "output.xml")]
        path: String,

        #[arg(short, long, default_value_t = 10)]
        lines: u32,
    },
}

fn open_file(path: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
}

fn write_csv(path: &str, lines: u32, delim: String, columns: Vec<String>) -> Result<(), io::Error> {
    let mut file = open_file(&path)?;
    // let mut headers: Vec<String> = Vec::new();

    // println!("{:?}", columns);

    /*
    for col in columns {
        let (col_name, col_type) = col.split_once(":").unwrap_or((&col, "string"));

        headers.push(col_name.to_string())
    }
    */

    // let mut headers: Vec<String> = Vec::new();

    // Headers
    write!(file, "index{delim}id{delim}name{delim}email{delim}city\n")?;

    // Data
    for i in 1..=lines {
        let name: String = Name().fake();
        let email: String = FreeEmail().fake();
        let city: String = CityName().fake();
        let id: String = UUIDv4.fake();

        write!(
            file,
            "{i}{delim}{id}{delim}{name}{delim}{email}{delim}{city}\n"
        )?;
    }

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Csv {
            path,
            lines,
            delim,
            columns,
        } => match write_csv(&path, lines, delim, columns) {
            Ok(_file) => println!("Opened successfully"),
            Err(e) => println!("Failed: {e}"),
        },
        Commands::Xml { path, lines: _ } => match open_file(&path) {
            Ok(_file) => println!("Opened successfully"),
            Err(e) => println!("Failed: {e}"),
        },
    }
}
