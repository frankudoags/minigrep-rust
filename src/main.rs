use std::env;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    println!("Searching for {}\n", config.query);
    println!("In file {}\n", config.filename);

    if let Err(e) = minigrep::Config::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
