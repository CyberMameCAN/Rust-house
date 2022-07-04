// (※)シンタックスシュガー
// 位か2つの関数は同等の意味になる
// fn function_1() -> Result(Success, Failure) {
//     match operation_that_might_fail() {
//         Ok(success) => success,
//         Err(failure) => return Err(failure),
//     }
// }
//
// fn function_2() -> Result(Success, Failure) {
//     operation_that_might_fail()?
// }
//
use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::Deserialize;
use serde::Serialize;
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

// Utc::now() メソッドを使用して、現在のタイムスタンプをキャプチャしている。
impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task {text, created_at}
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at) // {:<50}: 50 個のスペースが埋め込まれた左揃えの文字列
                                                        // [{}]: タスクが作成された日時 (かっこ内)
    }
}

// from_reader()でパラメータを渡し、Vec<Task>を受け取る
// 特定のエラーが発生したときに、match 式で guards を使用して空の Vec を作成する
// Vec は空の to-do リストを表す
fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?;
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,                 // 正常、tasksを返す
        Err(e) if e.is_eof() => Vec::new(), // EOF
        Err(e) => Err(e)?,                  // エラー
    };
    file.seek(SeekFrom::Start(0))?;  // カーソルのポインタを先頭に戻す処理
    Ok(tasks)
}

pub fn add_task(journal_path: PathBuf, task: Task) -> Result<()> {
    let file = OpenOptions::new()  // OpenOptionsでファイルを開く
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(journal_path)?;  // ?はシンタックスシュガー(※)

    let mut tasks = collect_tasks(&file)?;

    tasks.push(task);                       // ファイルへ書き込む
    serde_json::to_writer(file, &tasks)?;

    Ok(())  // 正常終了（からのタプルを返却）
}

pub fn complete_task(journal_path: PathBuf, task_position: usize) -> Result<()> {
    let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(journal_path)?;

    let mut tasks = collect_tasks(&file)?;

    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid Task ID"));
    }
    tasks.remove(task_position - 1);

    file.set_len(0)?;

    serde_json::to_writer(file, &tasks)?;

    Ok(())
}

pub fn list_tasks(journal_path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(journal_path)?;

    let tasks = collect_tasks(&file)?;

    if tasks.is_empty() {
        println!("Task list is empty!");
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{}: {}", order, task);
            order += 1;
        }
    }

    Ok(())
}
