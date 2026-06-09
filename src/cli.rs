use clap::{Parser, Subcommand};

fn parse_delim(s: &str) -> Result<u8, String> {
    match s {
        r"\t" => Ok(b'\t'),
        r"\n" => Ok(b'\n'),
        r"\r" => Ok(b'\r'),
        s if s.len() == 1 => Ok(s.as_bytes()[0]),
        _ => Err(format!(
            "delimiter must be a single ASCII character, got {:?}",
            s
        )),
    }
}

#[derive(Parser)]
#[command(name = "datagen", about = "Generate fake data files")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Csv {
        #[arg(short, long, default_value = "output.csv")]
        path: String,

        #[arg(short, long, default_value_t = 10)]
        records: u32,

        #[arg(short, long, default_value = ",", value_parser = parse_delim)]
        delim: u8,

        #[arg(
            short,
            long,
            default_value = "index:index, id:uuid, title:title, first_name:first_name, last_name:last_name, email:email, post_code:post_code, street_name:street_name, building_number:building_number, file_path:file_path, quoted:quoted",
            value_delimiter = ','
        )]
        columns: Vec<String>,
    },

    Tsv {
        #[arg(short, long, default_value = "output.tsv")]
        path: String,

        #[arg(short, long, default_value_t = 10)]
        records: u32,

        #[arg(
            short,
            long,
            default_value = "index:index, id:uuid, title:title, first_name:first_name, last_name:last_name, email:email, post_code:post_code, street_name:street_name, building_number:building_number, file_path:file_path, quoted:quoted",
            value_delimiter = ','
        )]
        columns: Vec<String>,
    },

    Xml {
        #[arg(short, long, default_value = "output.xml")]
        path: String,

        #[arg(short, long, default_value_t = 10)]
        records: u32,

        #[arg(
            short,
            long,
            default_value = "index:index, id:uuid, title:title, first_name:first_name, last_name:last_name, email:email, post_code:post_code, street_name:street_name, building_number:building_number, file_path:file_path, quoted:quoted",
            value_delimiter = ','
        )]
        columns: Vec<String>,
    },

    Json {
        #[arg(short, long, default_value = "output.json")]
        path: String,

        #[arg(short, long, default_value_t = 10)]
        records: u32,

        #[arg(
            short,
            long,
            default_value = "index:index, id:uuid, title:title, first_name:first_name, last_name:last_name, email:email, post_code:post_code, street_name:street_name, building_number:building_number, file_path:file_path, quoted:quoted",
            value_delimiter = ','
        )]
        columns: Vec<String>,
    },
}
