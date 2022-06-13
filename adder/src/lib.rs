// 機能をテストする一般的な方法は、テスト下にあるコードの結果を
// コードが返すと期待される値と比較して、等しいと確かめること

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess {value}
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
}

#[cfg(test)]
mod tests {
    use super::*; // (参照：第7章)

    #[test]
    fn larger_can_hold_smaller() {  // テスト名
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
    // fn it_works() {
    //     let result = 2 + 2;
    //     assert_eq!(result, 4);
    // }
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // fn exploration() {
    //     assert_eq!(2+2, 4);
    // }
    // #[test]
    // fn another() {
    //     panic!("Make this test fail.");
    // }
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),  // 文字列に含まれているかだけのテスト
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
