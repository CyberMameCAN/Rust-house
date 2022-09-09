#![allow(unused)]

use clap::Parser;
use anyhow::{Context, Result};
use indicatif::ProgressBar;
use log::{info, warn};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            // println!("{}", line);
            writeln!(writer, "{}", line);
        }
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(args.path)
        .expect("could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // let xs = vec![1, 2, 3];
    // println!("The list is: {:?}", xs);
    // println!("This is information");
    // eprintln!("This is an error! :(");

    プログレスバーの表示
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        // do_hard_work(); // 処理の追加

        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");

    content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout());

    // ログの出力
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");

    Ok(())
}

#[test]
fn find_matches() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}
