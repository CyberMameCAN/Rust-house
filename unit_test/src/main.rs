fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn main() {
    println!("{}", add(48, 5));
}

#[cfg(test)]  // サブモジュール化？
mod add_function_tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]  //  エラー(panic ?)をテストする時に付ける（「一致しない」というテストが合格する）
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore = "not yet reviewed by the Q.A. team"]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!();
    }

    #[test]
    fn is_false_when_odd() {
        assert!();
    }
}
