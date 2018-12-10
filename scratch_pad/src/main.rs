fn main() {
    let arr: [u32; 11] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    for num in arr.iter() {
        println!("{}", fizbuz(&num));
    }
}

fn fizbuz(num: &u32) -> String {
    if num % 6 == 0 {
        "fizbuz".to_string()
    } else if num % 3 == 0 {
        "fiz".to_string()
    } else if num % 2 == 0 {
        "buz".to_string()
    } else {
        num.to_string()
    }
}
