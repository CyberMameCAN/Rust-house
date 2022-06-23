// ジェネリック
// 引数の型が違っても、関数本体のコードが変わらない場合は、任意の型を受付ける関数を定義すること
//
// トレイト
// 同じような機能の関数の時に考える
// 引数名は「T」が慣例
// Ex) fn largest<T>(list: &[T]) -> T {
//
// // 構造体の宣言
// struct Point<T> {
//     x: T,
//     y: T,
// }
// // 構造体の宣言(複数の型の時)
// struct Point<T, U> {
//     x: T,
//     y: U,
// }
// // enumの宣言
// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
//
// トレイト境界：定義名の後に「<T: トレイト>」

pub fn generic_trainng() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// トレイト ///////////////////
// トレイトをSummaryという名前で定義している
pub trait Summary {
    // このトレイトを実装する型の振る舞い
    fn summarize_author(&self) -> String;
    // デフォルト値の機能
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implの後に実装したいトレイトの名前を書き、forキーワード、トレイトの実装対象の型名
impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("@{})", self.headline)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn test_trait_tweet() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

pub fn test_trait_news() {
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

// 引数としてのトレイト(引数として色々な型を取る方法)
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
