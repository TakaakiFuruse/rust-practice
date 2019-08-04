use std::vec::Vec;

pub enum DirType {
    VisitedDir,
    ParentDir,
    ChildDir,
    CurrentDir,
    NotSure,
}

macro_rules! dir_type_order1 {
    ($a:ident, $b:ident) => {
        fn dirtype_matcher1(var: DirType) -> u32 {
            match var {
                DirType::$a => 1,
                DirType::$b => 2,
                _ => 100,
            }
        }
    };
}

macro_rules! dir_type_order2 {
    ([$(($i:expr, $elm:tt)),*]) => {
        fn dirtype_matcher2(var: DirType) -> u32 {
            match var {
                $(DirType::$elm => $i,)*
                _ => 0,
            }
        }
    };
}

macro_rules! dir_type_order3 {
    ($vec:tt) => {
        println!("{:?}", $vec)
    };
}

fn main() {
    // パターン1ーIdentをそのまま渡す
    dir_type_order1!(VisitedDir, ChildDir);
    println!("{}", dirtype_matcher1(DirType::NotSure));
    println!("{}", dirtype_matcher1(DirType::VisitedDir));
    println!("{}", dirtype_matcher1(DirType::ChildDir));

    // パターン2ーVecとIdentを渡す
    dir_type_order2!([(0, VisitedDir), (1, ChildDir)]);
    println!("{}", dirtype_matcher2(DirType::NotSure));
    println!("{}", dirtype_matcher2(DirType::VisitedDir));
    println!("{}", dirtype_matcher2(DirType::ChildDir));

    //文字列からVecを作って、IndexとIdentを変数のママ渡せないか?
    let order_setting: Vec<&str> = "VisitedDir ChildDir".split(" ").collect();
    let mut vv = Vec::new();
    for (i, elm) in order_setting.iter().enumerate() {
        vv.push((i, elm))
    }
    dbg!(&vv);
    dir_type_order3!(vv);
    // println!("{}", dirtype_matcher3(DirType::NotSure));
    // println!("{}", dirtype_matcher3(DirType::VisitedDir));
    // println!("{}", dirtype_matcher3(DirType::ChildDir));
}
