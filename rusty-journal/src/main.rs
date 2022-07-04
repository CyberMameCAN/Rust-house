// サードパーティ製のクレートを追加
// //// クレート(structopt)の検索 ////
// $ cargo search structopt
// Cargo.tomlに追加 [dependencies]の下
// structopt = "0.3"
// chrono ///////////////////////////
// 時刻データを扱う時に便利なクレート
// anyhow ///////////////////////////
// 独自のエラーの種類を提供。整形出力のプロパティがある
// serde_json //////////////////////
// ジャーナル ファイルの内容を 構造体 のベクトルに解析する
// home //////////////////////
// ホーム ディレクトリを判断するクレート
//
// リリースする時
// $ cargo run --release
//
use anyhow::anyhow;
use std::path::PathBuf;
use structopt::StructOpt;
mod cli;
mod tasks;

use cli::{Action::*, CommandLineArgs};
use tasks::Task;

// 既定のジャーナル ファイルの完全なパスを作成する処理
fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // 「非構造化」
    // それぞれの値をタスク処理関数に「個別に」渡すことが出来る
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    let journal_file = journal_file
                        .or_else(find_default_journal_file)     // .or_else メソッドでは、map メソッドと逆の処理になる
                                                                // バリアントが None の場合にのみ保持する関数が呼び出される
                        .ok_or(anyhow!("Failed to find journal file"))?;

    match action {
        Add {text} => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done {position} => tasks::complete_task(journal_file, position),
    }?;
    // .expect("Failed to perform action")
    // 全ての関数からResult型は返ってこないので、末尾で.expectを呼び出す
    // anyhowクレートを使うので無効にする

    Ok(())
}
