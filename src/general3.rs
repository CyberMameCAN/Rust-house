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
