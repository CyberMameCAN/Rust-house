use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // 文字列のベクトル型

    // let query: &String = &args[1];
    // let filename: &String = &args[2];
    let config: Config = Config::new(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e: Box<dyn Error>) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
