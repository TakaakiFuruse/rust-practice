#[derive(Debug)]
enum FizBuz {
    Fiz { x: u32, y: u32 },
    Buz(String),
    FizBuz(u32, u32, u32),
    Num(u32),
}

#[derive(Debug)]
struct Message {}

fn main() {
    println!("{:?}", Option::Some(5));
    for num in 1..=12 {
        println!("fizbuz: {:?}", fizbuz(num));
        println!("fizbuz2: {:?}", fizbuz2(num));
        let res = fizbuz3(num);
        match res {
            Some(x) => println!("fizbuz3: {}", x),
            None => println!("NONE!!"),
        }
    }
}

impl Message {
    fn fiz(&self) -> String {
        "fiz".to_string()
    }
    fn fizbuz(&self) -> String {
        "fizbuz".to_string()
    }
    fn buz(&self) -> String {
        "buz".to_string()
    }
    fn num(&self, num: u32) -> String {
        num.to_string()
    }
}

fn fizbuz(num: u32) -> FizBuz {
    if num % 6 == 0 {
        FizBuz::FizBuz(num, num, num)
    } else if num % 3 == 0 {
        FizBuz::Fiz { x: num, y: num }
    } else if num % 2 == 0 {
        FizBuz::Buz(num.to_string())
    } else {
        FizBuz::Num(num)
    }
}

fn fizbuz2(num: u32) -> String {
    if num % 6 == 0 {
        Message {}.fizbuz()
    } else if num % 3 == 0 {
        Message {}.fiz()
    } else if num % 2 == 0 {
        Message {}.buz()
    } else {
        Message {}.num(num)
    }
}

fn fizbuz3(num: u32) -> Option<String> {
    if num % 6 == 0 {
        Some("FizBuz".to_string())
    } else if num % 3 == 0 {
        Some("Buz".to_string())
    } else if num % 2 == 0 {
        Some("Fiz".to_string())
    } else {
        Some(num.to_string())
    }
}
