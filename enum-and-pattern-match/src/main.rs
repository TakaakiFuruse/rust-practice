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
    // Matchihg with Option<
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);

    // Plus one can be shorter...
    // if let is a syntax suger of match
    let five_ifl = Some(5);
    let six_ifl = Some(6);

    if let Some(5) = five_ifl {
        println!("five_ifl: {:?}", five_ifl);
    };

    if let Some(7) = six_ifl {
        println!("six ifl is Some(7) !!");
    } else {
        println!("six ifl is not Some(7) !!");
    };

    // === fizbuz ====
    for num in 1..=12 {
        println!("fizbuz: {:?}", fizbuz(num));
        println!("fizbuz2: {:?}", fizbuz2(num));
        let res = fizbuz3(num);
        match res {
            Some(x) => println!("fizbuz3: {}", x),
            None => println!("NONE!!"),
        }
        println!("fizbuz4: {}", fizbuz4(num));
    }
}

fn plus_one(x: Option<u32>) -> Option<u32> {
    // fn plus_one_if(x: u32) -> u32 {
    //     if x != None {
    //         x + 1
    //     }
    // }
    // という処理だと、そもそもNoneが渡せない。
    match x {
        None => None,
        Some(i) => Some(i + 1),
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
        Some("Fiz".to_string())
    } else if num % 2 == 0 {
        Some("Buz".to_string())
    } else {
        Some(num.to_string())
    }
}

fn fizbuz4(num: u32) -> String {
    match (num % 6 == 0, num % 3 == 0, num % 2 == 0) {
        (true, true, true) => "fizbuz".to_string(),
        (false, true, false) => "fiz".to_string(),
        (false, false, true) => "buz".to_string(),
        _ => num.to_string(),
    }
}
