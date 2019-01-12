#![allow(dead_code)]

fn main() {}

// assert_eq と assert_ne がある
#[cfg(test)]
mod tests_assert {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2)
    }

    #[test]
    fn not_works() {
        assert_ne!(1 + 1, 4)
    }
}

// 関数のテストを書く
fn add(a: i32) -> i32 {
    a + 1
}

#[cfg(test)]
mod tests_add {
    use super::*;

    #[test]
    fn will_add_1_to_var() {
        assert_eq!(add(1), 2)
    }

    #[test]
    fn will_not_subtract() {
        assert_ne!(add(1), 0)
    }
}

// Rectangle のメソッドのテストを書く

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 4,
            width: 3,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            length: 8,
            width: 7,
        };
        let smaller = Rectangle {
            length: 4,
            width: 3,
        };
        assert!(!smaller.can_hold(&larger));
    }
}

// 出力が文字を含むかテスト
pub fn greeting(name: &str) -> String {
    format!("Hello {}!!", name)
}

#[cfg(test)]
mod tests_for_greeting {
    use super::*;

    // 文字が入ってるかテスト
    #[test]
    fn contains_name_less_consise() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"))
    }

    // メッセージをカスタムできる。変数の値も出せる
    #[test]
    fn contains_name_more_concise() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting has no name, result: {}",
            result
        )
    }
}

// Should Panic でPanicをチェック

pub struct SendaMituo {
    call: String,
}

impl SendaMituo {
    pub fn call(&self) -> String {
        format!("==={}===", &self.call)
    }
    pub fn panic(&self) -> String {
        panic!("うわこらちょいおま・・・")
    }

    // selfをいれないと SendaMituo::panic_assoc() のように呼ぶことになる
    pub fn panic_assoc() -> String {
        panic!("うわこらちょいおま・・・")
    }
}

#[cfg(test)]
mod tests_SendaMituo {
    use super::*;

    #[test]
    fn call_retuns_string() {
        let obj = SendaMituo {
            call: String::from("せんだ"),
        };
        assert_eq!(obj.call(), "===せんだ===")
    }

    #[test]
    #[should_panic]
    fn panic_returns_panic() {
        let obj = SendaMituo {
            call: String::from("みつお"),
        };
        obj.panic();
    }

    #[test]
    #[should_panic]
    fn panic_returns_panic_assoc() {
        SendaMituo::panic_assoc();
    }

    // should_panicを使うなら expected を渡すほうがメッセージのチェックもできて賢い
    #[test]
    #[should_panic(expected = "うわこらちょいおま・・・")]
    fn panic_returns_panic_consice() {
        let obj = SendaMituo {
            call: String::from("みつお"),
        };
        obj.panic();
    }

    #[test]
    #[should_panic(expected = "うわこらちょいおま・・・")]
    fn panic_returns_panic_assoc_consice() {
        SendaMituo::panic_assoc();
    }

}

// Result Typeを使ってテストしてみる
fn it_works_ifs(num: i32) -> Result<String, String> {
    if num <= 10 {
        Ok(String::from("It's OK!!"))
    } else {
        Err(String::from("BAAAAD!!"))
    }
}

#[cfg(test)]
mod tests_it_works_ifs {
    use super::*;

    #[test]
    fn test_it_works_ifs_under_10() {
        assert_eq!(it_works_ifs(10), Ok(String::from("It's OK!!")))
    }

    #[test]
    fn test_it_works_ifs_over_10() {
        assert_eq!(it_works_ifs(15), Err(String::from("BAAAAD!!")))
    }

    #[test]
    #[ignore]
    fn test_ignored() {
        // This test will be ignored.
        assert_eq!(true, false)
    }
}

// Testing Private Functions
// ふつーに呼び出すだけでなんとかしてくれる
struct NahaNaha {
    text: String,
}

impl NahaNaha {
    pub fn nahanaha_pub(&self) -> String {
        return "なはなは(pub)".to_string();
    }
    fn nahanaha_prv(&self) -> String {
        return "なはなは(prv)".to_string();
    }
}

#[cfg(test)]
mod tests_NahaNaha {
    use super::*;

    #[test]
    fn namanaha_pub_returns_text() {
        let obj = NahaNaha {
            text: String::from("test"),
        };
        assert_eq!(obj.nahanaha_pub(), "なはなは(pub)")
    }
    #[test]
    fn namanaha_prv_returns_text() {
        let obj = NahaNaha {
            text: String::from("test"),
        };
        assert_eq!(obj.nahanaha_prv(), "なはなは(prv)")
    }
}
