// Define a function which takes a generic `F` argument
// bounded by `Fn`, and calls it
// 関数を引数として取り、即座に実行する関数を定義
// fn call_me<F: Fn()>(f: F) {
//     f();
// }

// fn twice<F: Fn()>(f: F, x: i32) {
//     f(f(x))
// };

// fn add_two(x) {
//     4 + 2
// }

fn fibonacci(x: i32) -> i32 {
    // matchも使える(select ~ case)
    if x == 0 {
        0
    } else if x == 1 {
        1
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}

fn main() {
    // この数値を変更 /////////////////////////////
    let order = 11;  // 何番目（ゼロ基数）
    // ///////////////////////////////////////////
    let mut fibs = vec![];  // 可変の配列 ベクター

    let mut q = order;
    loop {
        fibs.push(fibonacci(q));

        q -= 1;
        if q < 0 {
            break;
        }
    }
    println!("fibonacci {}th number is {:?}.", order + 1, fibs);
    // call_me(add_two);
}


