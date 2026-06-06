use clap::{Parser, Subcommand};
use fake::Fake;
use fake::faker::address::en::*;
use fake::faker::creditcard::en::CreditCardNumber;
use fake::faker::filesystem::ar_sa::FileExtension;
use fake::faker::filesystem::en::FileName;
use fake::faker::filesystem::en::FilePath;
use fake::faker::http::en::RfcStatusCode;
use fake::faker::internet::en::*;
use fake::faker::lorem::en::Word;
use fake::faker::name::en::*;
use fake::faker::phone_number::ar_sa::PhoneNumber;
use fake::faker::time::ar_sa::Date;
use fake::faker::time::ar_sa::DateTime;
use fake::faker::time::ar_sa::Time;
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
            default_value = "index:index, id:uuid, title:title, first_name:first_name, last_name:last_name, email:email, post_code:post_code, street_name:street_name, building_number:building_number, file_path:file_path",
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

// Opens file and returns file handle
fn open_file(path: &str) -> Result<File, io::Error> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
}

// Generates and writes csv file from input params
fn write_csv(path: &str, lines: u32, delim: String, columns: Vec<String>) -> Result<(), io::Error> {
    let mut file = open_file(&path)?;
    let mut headers: Vec<String> = Vec::new();
    let mut types: Vec<String> = Vec::new();

    // Parse schema
    for col in columns {
        let (col_name, col_type) = col.split_once(":").unwrap_or((&col, "string"));

        headers.push(col_name.to_string());
        types.push(col_type.to_string());
    }

    // Data
    let mut rows: Vec<String> = Vec::new();
    for i in 1..=lines {
        // Build row
        let mut row: Vec<String> = Vec::new();
        for t in &types {
            let col = match t.as_str() {
                "index" => i.to_string(),
                "uuid" => UUIDv4.fake(),
                "char" => {
                    let w: String = Word().fake();
                    w.chars().next().unwrap_or('a').to_string()
                }

                "post_code" => PostCode().fake(),
                "zip_code" => ZipCode().fake(),
                "country_name" => CountryName().fake(),
                "city_name" => CityName().fake(),
                "street_name" => StreetName().fake(),
                "building_number" => BuildingNumber().fake(),

                "title" => Title().fake(),
                "full_name" => Name().fake(),
                "first_name" => FirstName().fake(),
                "last_name" => LastName().fake(),
                "username" => Username().fake(),
                "password" => Password(8..20).fake(),

                "ip" => IPv4().fake(),
                "email" => SafeEmail().fake(),
                "phone" => PhoneNumber().fake(),
                "credit_card_number" => CreditCardNumber().fake(),

                "date" => Date().fake(),
                "time" => Time().fake(),
                "date_time" => DateTime().fake(),

                "file_path" => FilePath().fake(),
                "file_name" => FileName().fake(),
                "file_ext" => FileExtension().fake(),

                "http_status_code" => RfcStatusCode().fake(),
                "rfc_status_code" => RfcStatusCode().fake(),

                _ => String::new(),
            };

            row.push(col)
        }

        // Add to rows
        rows.push(row.join(&delim))
    }

    write!(file, "{}\n", headers.join(&delim))?;
    write!(file, "{}", rows.join("\n"))?;

    Ok(())
}

// Parse cli params and
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Csv {
            path,
            lines,
            delim,
            columns,
        } => match write_csv(&path, lines, delim, columns) {
            Ok(_file) => println!("CSV File generated"),
            Err(e) => println!("Failed: {e}"),
        },
        Commands::Xml { path, lines: _ } => match open_file(&path) {
            Ok(_file) => println!("XML File generated"),
            Err(e) => println!("Failed: {e}"),
        },
    }
}
