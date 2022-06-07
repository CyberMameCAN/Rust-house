// 所有権（Rustの最も特異な機能）
//
// Rustの各値は、所有者と呼ばれる変数と対応している
// いかなる時も所有者は１つである
// 所有者がスコープから外れたら、値は破棄される
// メンバ変数に書き込んだりする場合は所有権が必要になる

pub fn garbage_collection() {
    // let mut s = String::from("Hello");
    // s.push_str(", world!");
    // println!("{}", s);

    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}, world!", s1);  // s1は既に無効になっている（メモリの２重開放に繋がるため）
    println!("{}, world!", s2);

    let s3 = String::from("こんちくわ");
    let s4 = s3.clone();
    println!("s3={}, s4={}", s3, s4);

    let x = 5;
    let y = x;
    println!("x={}, y={}", x, y);  // これはオッケ（整数のようなコンパイル時に既知のサイズを持つ型はcloneしなくても大丈夫）
}

pub fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
