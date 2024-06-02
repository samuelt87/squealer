use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    db_file: Option<String>,
}

#[derive(Clone)]
pub struct Config {
    pub db_file: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        let args = Args::parse();
        Config {
            db_file: args.db_file,
        }
    }
}
