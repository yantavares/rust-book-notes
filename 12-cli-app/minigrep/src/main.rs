use minigrep::Config;
use std::env;
use std::process;

// This program searches for a line in a file that contains a given string
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config) {
        // If run returns an error, print it and exit
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
