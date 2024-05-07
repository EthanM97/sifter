use std::env;
use std::process;

use sifter::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    eprintln!("Searching for {}", config.query);
    eprintln!("In file: {}", config.filename);

    if let Err(e) = sifter::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
