use std::vec::Vec;

fn greetings(var: String) {
    println!("{}", var)
}

// what if .... but this won't compile
// fn greetings_generator(var1: String, var2: String) {
//    fn var2(var: String){
//     var1("{}", var)
//   }
// }

macro_rules! greetings_generator {
    ($var1:ident, $var2:ident) => {
        fn $var2(var: String) {
            $var1!("{}", var)
        }
    };
}

pub enum Gods {
    Father,
    Mother,
    Maiden,
    Crone,
    Warrior,
    Smith,
    Stranger,
}

#[allow(dead_code)]
fn my_gods_order(god: Gods) {
    match god {
        Gods::Father => 1,
        Gods::Mother => 2,
        Gods::Maiden => 3,
        Gods::Crone => 4,
        Gods::Warrior => 5,
        Gods::Smith => 6,
        Gods::Stranger => 7,
    };
}

#[allow(dead_code)]
fn your_gods_order(god: Gods) {
    match god {
        Gods::Father => 7,
        Gods::Mother => 6,
        Gods::Maiden => 5,
        Gods::Crone => 4,
        Gods::Warrior => 3,
        Gods::Smith => 2,
        Gods::Stranger => 1,
    };
}

macro_rules! gods_order1 {
    ($a:ident, $b:ident) => {
        fn gods_matcher1(var: Gods) -> u32 {
            match var {
                Gods::$a => 1,
                Gods::$b => 2,
                _ => 100,
            }
        }
    };
}

macro_rules! gods_order2 {
    ([$(($i:expr, $elm:tt)),*]) => {
        fn gods_matcher2(var: Gods) -> u32 {
            match var {
                $(Gods::$elm => $i,)*
                _ => 0,
            }
        }
    };
}

macro_rules! gods_order3 {
    ($vec:tt) => {
        println!("{:?}", $vec)
    };
}

fn main() {
    // just a normal print statment
    greetings("YOOOOO!".to_string());
    // what if ...
    greetings_generator!(print, greetings2);
    greetings2("YO from greeting2 \n".to_string());

    greetings_generator!(println, greetings3);
    greetings3("YO from greeting3".to_string());

    // パターン1ーIdentをそのまま渡す
    gods_order1!(Father, Mother);
    println!("{}", gods_matcher1(Gods::Father));
    println!("{}", gods_matcher1(Gods::Mother));
    println!("{}", gods_matcher1(Gods::Smith));

    // パターン2ーVecとIdentを渡す
    gods_order2!([(0, Stranger), (1, Warrior)]);
    println!("{}", gods_matcher2(Gods::Stranger));
    println!("{}", gods_matcher2(Gods::Warrior));
    println!("{}", gods_matcher2(Gods::Crone));

    //文字列からVecを作って、IndexとIdentを変数のママ渡せないか?
    let order_setting: Vec<&str> = "Smith Mother".split(" ").collect();
    let mut vv = Vec::new();
    for (i, elm) in order_setting.iter().enumerate() {
        vv.push((i, elm))
    }
    dbg!(&vv);
    gods_order3!(vv);
    // println!("{}", gods_matcher3(DirType::NotSure));
    // println!("{}", gods_matcher3(DirType::VisitedDir));
    // println!("{}", gods_matcher3(DirType::ChildDir));
}
