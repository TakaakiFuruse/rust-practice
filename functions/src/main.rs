fn main() {
    another_function("JOHN", 3);
    let b = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("var b is {}", b);
    let kekka = tashizan(21, 21);
    println!("tashizan result is {}", kekka);
}

fn another_function(x: &'static str, y: i32) {
    println!("HELLO!! {} {}!!", x, y)
}

fn tashizan(x: i32, y: i32) -> i32 {
    // x + y; causes compile error.
    // need to return some value/s
    x + y
}
