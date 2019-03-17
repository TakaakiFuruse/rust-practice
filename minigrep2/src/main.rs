use minigrep2;
use minigrep2::Config;
use std::env;
use std::process;
fn main() {
    let args: &Vec<String> = &env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err| {
        print!("Config parse error: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep2::run(config) {
        println!("App error {}", e);
        process::exit(1);
    };
}
