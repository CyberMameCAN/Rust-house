use std::io;  // 入出力ライブラリ
use rand::Rng;
use std::cmp::Ordering;

mod general3;

// 「クレート」Rustソースコードを集めたもの

fn main() {
    general3::general3();  // 別ファイルの関数呼び出し
    general3::hensu_declare();

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
