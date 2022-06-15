pub fn general3() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

pub fn hensu_declare() {
    let x = 2.0;  // f64
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;

    let c = 'z';  // シングルクォーテーション

    // タプル
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;        // 束縛と
    println!("Tuple0: {}", x);  // 分解
    println!("Tuple1: {}", tup.1);  // こちらは配列の使い方

    // 配列
    let a = [1, 2, 3, 4, 5];
    println!("配列の2番目 {}", a[1]);
}

// 引数や返り値
pub fn another_function(x: i32, with_label: char) -> i32 {
    println!("The value of x is: {}{}", x, with_label);

    x * x  // ここにセミコロンは入らない
}

// 無名関数
pub fn noname_function() {
    let y = {
        let x = 3;
        x + 1       // ここにセミコロンは入らない(returnするから)
    };

    println!("Noname Function value is {}", y);
}

// 制御構文
pub fn control_structure() {
    let mut isGood = false;

    if isGood {
        println!("Good!!!");
    } else {
        isGood = true;
        println!("No Good...");
    }

    let number = if isGood {10} else {0};
    println!("Your id is {}", number);

    // ループ
    // continueも使える
    let mut count = 0;
    'counting_up: loop {  // ここは初めて見る使い方。先頭にシングルクォーテーションでラベルを定義
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;  // 内側loopのためのbreak
            }
            if count == 2 {
                break 'counting_up;  // ラベル付きのbreak. 先頭にシングルクォーテーション.
                                     // 外側のcounting_upラベルを付けたloopのためのbreak
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for av in &a {  // こちらの使い方が好み
        println!("Array value is {}", av);
    }

    // こういう範囲指定も出来る
    for number in (1..4).rev() {  // 1,2,3 rev()で逆から
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
