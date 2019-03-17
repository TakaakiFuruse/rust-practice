use minigrep2::run;
use minigrep2::Config;
use std::env;
use std::process;
fn main() {
    let args: &Vec<String> = &env::args().collect();
    let config = Config::new(args).unwrap_or_else(|err| {
        print!("Config parse error: {}", err);
        process::exit(1)
    });
    run(config);
}
