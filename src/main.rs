// Rustの代表的標準ライブラリ (Microsoft learn より)　////////////////////////////////////////
// std - Rust 標準ライブラリ。 Rust の演習では、次のモジュールを確認します。
// std::collections - コレクション型の定義。HashMap など。
// std::env - お使いの環境を操作する関数。
// std::fmt - 出力形式を制御する機能。
// std::fs - ファイル システムを操作する関数。
// std::io - 入力/出力を操作する定義と機能。
// std::path - ファイル システム パス データの操作をサポートする定義と関数。
// structopt - コマンド ライン引数を簡単に解析するためのサードパーティ製のクレート。
// chrono - 日付と時刻のデータを処理するサードパーティ製のクレート。
// regex - 正規表現を処理するサードパーティ製のクレート。
// serde - Rust データ構造のシリアル化および逆シリアル化操作のサードパーティ製のクレート。
//
// Cargo と呼ばれる Rust ビルド ツール兼依存関係マネージャー
// cargo new コマンドを使用して、新しいプロジェクト テンプレートを作成する。
// cargo build コマンドを使用して、プロジェクトをビルドする。
// cargo run コマンドを使用して、プロジェクトをビルドして実行する。
// cargo test コマンドを使用して、プロジェクトをテストする。
// cargo check コマンドを使用して、プロジェクトの種類を確認する。
// cargo doc コマンドを使用して、プロジェクトのドキュメントをビルドする。
// cargo publish コマンドを使用して、crates.io にライブラリを発行する。
// Cargo.toml ファイルにクレート名を追加して、依存クレートをプロジェクトに追加する。
//
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=39cc3f3ab484636b8df0b3d49f8666c4
//
// struct、loop
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=df72129e5bc4643d4c3733a96c40e1bd
//
//   Option<T> 列挙型の宣言の <T> の部分は、型 T がジェネリックであり、
//   Option 列挙型の Some バリアントに関連付けられている。
//   None と Some は型ではなく、Option<T> 型のバリアント(列挙型に列挙した値)である。
//   これは特に、関数が引数として Some または None を取ることはできず、Option<T> のみを受け取ることを意味する。
// enum Option<T> {
//     None,     // The value doesn't exist
//     Some(T),  // The value exists
// }
//
// let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
// for &index in [0, 2, 99].iter() {
//     // この分岐はmatchアームと呼称。
//     match fruits.get(index) {    // Vec::get
//         // こういう分岐が出来る
//         Some(&"coconut") => println!("Coconuts are awesome!!!"),
//         Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
//         None => println!("There is no fruit! :("),
//     }
// }
//
//   if let 式が便利なのは、1 つのパターンに一致させる必要がある場合に、
//   match 式のすべての定型コードが不要になること。
// let a_number: Option<u8> = Some(7);
// if let Some(7) = a_number {
//     println!("That's my lucky number!");
// }
//
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=2b1715c7a59d81cf2ac3962f2e209a69
//
//   Result<T, E> エラーを返して伝達するための列挙型が用意されている
// enum Result<T, E> {
//     Ok(T):  // A value T was obtained.
//     Err(E): // An error of type E was encountered instead.
// }
// 値が "存在しない" 可能性を記述する Option 型
// 処理が"失敗"する可能性がある場合に最適な Result 型
//
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=127b82b29e6d6430a1507d4a6dd52ee9



use std::io;  // 入出力ライブラリ
use rand::Rng;
use std::cmp::Ordering;

mod general3;
mod ownership;
mod struct5;
mod collection8;
mod error9;
mod generic10;

// 「クレート」Rustソースコードを集めたもの

fn main() {
    println!("- - - - - - - - - -");
    generic10::generic_trainng();
    generic10::test_trait_tweet();
    generic10::test_trait_news();

    error9::error_training();

    collection8::collection_training();
    struct5::structure_handson();

    ownership::garbage_collection();
    let s1 = String::from("hello");
    let (s2, len) = ownership::calculate_length(s1);
    println!("The lenght of '{}' is {}.", s2, len);

    general3::general3();  // 別ファイルの関数呼び出し
    general3::hensu_declare();
    let x = general3::another_function(5, 'h');  // 返り値がある場合
    println!("Return value * value is {}", x);
    general3::noname_function();
    general3::control_structure();

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);  // 答えの表示


    let mut i = 0;
    loop {
        println!("Please input your guess.");

        // 変数の入力
        let mut guess = String::new(); // mut は可変の変数宣言
        // let apples = 5;  // 不変の変数 const?

        // ここは1行で書くことも出来る（メソッドチェーン？）が、下記のように分解したほうが可読性が良い
        // Swiftっぽくて良きかな。
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");  // 失敗した時に実行（これを付けないとコンパイルでerrorが出る）

        // ここのguessはシャドーイング、既に宣言していたguessを新しい値で覆い隠す
        // （Rustはこういうことが許可されている。型が違う複数の変数名を定義する必要がないメリット）
        let guess: u32 = match guess.trim().parse() {   // parse()は文字列を色々な数値型に変換できるので、u32を使って型を指定
            Ok(num) => num,
            Err(_) => continue,
        };

        i += 1;
        // let i = i + 1;  // これもシャドーイング(変数宣言時はmutは使わない)
        println!("{} 回目 You guessed: {}", i, guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
   }
}
