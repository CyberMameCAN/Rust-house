use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // 任意の型を返す・・・らしい・・・
    let contents: String = fs::read_to_string(path: config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    fn pub new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // let query: &String = &args[1];
        let query: String = args[1].clone();     // 文字列の所有権を渡したくないためcloneを使う
        // let filename: &String = &args[2];
        let filename: String = args[2].clone();

        Ok(Config {query, filename})
    }
}

