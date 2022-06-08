// <T> はジェネリック型引数という（10章を参照）////////////
// あらゆる型のデータを一つだけ持てる
// Some列挙子: あらゆる型のデータを一つだけ持てる
//
// emum Option<T> {
//     Some(T),
//     None,
// }
//
// let some_number = Some(S);
// let some_string = Some("a string");
//
// let absent_number: Option<i32> = None;
//
// match 制御フロー演算子 ////////////////////////////////
// enumとmatchの組み合わせはRustではよく見かける
#[derive(Debug)]

enum UsState {
    Alabama,
    Alaska,
    Hawai,
    // etc...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {  // こういう使い方も出来る(文字列表示して、1を返す)
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // 入力がNoneだったらNoneを返し、
    // 入力に値があったら+1をして返す
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

// if let を使う ///////////////////////////////////

// 3にまっちしたときだけ処理
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (), // 「他は処理しない」という意味
}
// これは冗長な表現・・・らしい。以下のようにif letを使えば簡潔に書ける
// ただし、matchの強制的な包括的チェックがされないので注意
// if let ~ elseという使い方もある
if let Some(3) = some_u8_value {
    println!("three");
}
