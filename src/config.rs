use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path to the SQLite database file
    #[arg(short = 'f', long)]
    db_file: Option<String>,

    /// Use an in-memory database
    #[arg(short = 'm', long)]
    in_memory: bool,
}

#[derive(Clone)]
pub struct Config {
    pub starting_db: StartingDb,
}

#[derive(Clone)]
pub enum StartingDb {
    InMemory,
    File(String),
    None,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();
        let starting_db = if args.in_memory {
            StartingDb::InMemory
        } else {
            match args.db_file {
                Some(file) => StartingDb::File(file),
                None => StartingDb::None,
            }
        };
        Config { starting_db }
    }
}
