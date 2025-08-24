mod config;
mod error;
mod gitignore;
mod stats;
mod tree;

use config::Config;
use error::TreeResult;
use std::process;
use tree::TreeGenerator;

fn main() {
    let config = Config::parse_args();

    if let Err(err) = run(config) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run(config: Config) -> TreeResult<()> {
    let mut generator = TreeGenerator::new(&config);
    generator.generate()?;
    Ok(())
}
