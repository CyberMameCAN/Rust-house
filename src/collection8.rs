pub fn collection_training() {
    vector_training();
    string_training();
    hash_training();
}

fn vector_training() {
    // let mut v: Vec<i32> = Vec::new();
    // v.pysh(5);
    // v.pysh(6);
    // v.pysh(7);
    let mut v = vec![1, 2, 3, 4, 5];  // !はマクロ。初期値を指定して初期化するのが一般的

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("THe third element is {}.", third),
        None => println!("There is no third element."),
    }

    for i in &mut v {
        *i += 50;  // アスタリスクは「参照外し演算子」というらしい
        println!("{}", i);
    }

    let mut s1 = "foo";
}

fn string_training() {
    let str1 = String::from("abcdefg");
    let len = str1.len();

    let str2 = String::from("vwxyz");
    let comb = format!("{}-{}", str1, str2);
    println!("{}", comb);
}

fn hash_training() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    // ハッシュに追加
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // ハッシュから削除
    //  .remove()

    // let teams = vec![String::from("Blue"), String::from("Yellow")];
    // let initial_scores = vec![10, 50];
    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // 値の取り出し(なぜかめんどくさい)
    // let teams_name = String::from("Blue");
    // let score = scores.get(&teams_name);

    for (key, value) in &scores {
        println!("{} - {}", key, value);
    }

    // 値の上書き
    scores.insert(String::from("Blue"), 99);  // 同じキーが既にあったら上書きになる
    println!("{:?}", scores);

    // キーチェック
    scores.entry(String::from("Red")).or_insert(49);      // キーがないので登録される
    scores.entry(String::from("Yellow")).or_insert(111);  // キーは既にあるので登録されない
    println!("{:?}", scores);

    // 古い値に基づいて値を更新する
    let text = "Hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
