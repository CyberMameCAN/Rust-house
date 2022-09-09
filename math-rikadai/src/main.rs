// https://www.youtube.com/watch?v=MYi_bU_MHpQ
// 大学入試だけど、中学生も解ける！！（東京理科大）
//
// 【問】ある2桁の正の整数mを2乗すると
//       下2桁が36になる。そのmを求める。
//
use std::collections::HashMap;

fn question(x: u64) -> u64 {
    let x_times;
    let under2keta: u64;

    let fix_num = 100;   // 下2桁を00にするための変数

    // （条件）二乗する
    x_times = x.pow(2);
    // 文字列に変換してから後ろ2文字を取得しようと思ったが止めた
    // let x2_str: String = x_times.to_string();

    // 下2桁を取得
    // 512 -> 500, 1024 -> 1000 のような数値にする(下2桁を00へ変換)
    let x_roundup = (x_times / fix_num) * fix_num as u64;   // 小数以下を切り捨てて、桁を基に戻す
    under2keta = x_times - x_roundup;                       // Ex) 72 = 572 - 500 これで下2桁が返るはず

    return under2keta
}

fn main() {
    let target_number = 36;  // 下2桁がこの数値となるmを求める
    // ある2桁の整数 [0..99]とかしたほうがいいのかも
    let mut i = 10;
    let max_i = 99;

    // let mut ans = vec![];  // (配列の場合)
    let mut ans = HashMap::new();  // 辞書型使う

    loop {
        let rtn = question(i);  // ループの中でmutはなくても、同じ変数名は使えるようだ
        if rtn == target_number {
            // ans.push(i);  // i^2の下2桁が36に等しかったらiを退避(配列の場合)
            ans.insert(i, i.pow(2));  // i^2の下2桁が36に等しかったらiを退避
        }

        if i > max_i {
            break;
        }
        i += 1;
    }

    // println!("Ans. is {:?}.", ans);  // (配列の場合)
    for (k, v) in &ans {
        println!("{} -> {}", k, v);
    }
}
