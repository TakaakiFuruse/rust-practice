#![allow(dead_code)]

// ========== display ==============
use std::fmt;
struct DisplayTest(i32);

#[derive(Debug)]
struct DisplayTest2(i32);

impl fmt::Display for DisplayTest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let test1 = DisplayTest(12);
    let test2 = DisplayTest2(14);

    // 12
    //     DisplayTest2(
    //         14
    //     )
    println!("{}", test1);
    println!("{:#?}", test2);
}

// =========== debug ================
#[derive(Debug)]
struct PrintTest1(i32);

struct PrintTest2(i32);

#[derive(Debug)]
struct Printtest3<'a> {
    name: &'a str,
    age: i32,
}

fn debug() {
    // "()[]*.\n" "test" "aa\'s"
    println!("{1:?} {0:?} {var:?}", "test", "()[]*.\n", var = "aa's");

    // #[derive(Debug)] あるのでプリントできる
    println!("{:?}", PrintTest1(3));

    //  `PrintTest2` cannot be formatted using `{:?}`
    // println!("{:?}", PrintTest2(3));

    // == pritty print ==
    // Printtest3 {
    //     name: "デーモン小暮閣下",
    //     age: 100054
    // }
    let name = "デーモン小暮閣下";
    let age = 100054;
    let person1 = Printtest3 { name, age };
    let person2 = Printtest3 {
        name: "name",
        age: 1234,
    };
    println!("{:#?}", person1);
    println!("{:#?}", person2.name);
    println!("{:#?}", person2.age);
}

fn formatted_print() {
    let pi = 3.1415;
    let pii = format!("{:.*}", 2, 3.1415921);

    println!("Pi is roughly {}, {}", pi, pii); // -> Pi is roughly 3.1415, 3.14
    println!("{0}, {1}, {2}", "せんだ", "みつお", "なはなは"); // -> せんだ, みつお, なはなは

    // ペン パイナップル and アッポーペン
    println!(
        "{pen} {pineapple} and {applepen}",
        pen = "ペン",
        pineapple = "パイナップル",
        applepen = "アッポーペン"
    );

    //「    pen」
    println!("{number:>width$}", number = "pen", width = 6);
}
