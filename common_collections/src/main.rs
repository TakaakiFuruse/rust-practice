fn main() {
    // ===== Vector -- a growable array-- ===

    // 直接Newしてもいいし
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("{:?}", v);

    // マクロを使ってもいい
    let mutv = vec![1, 2, 3];
    println!("{:?}", mutv);

    // []で値を取り出せるが
    // 存在しないインデックスを指定するとPanicになる
    let third: &i32 = &v[2];
    println!("{}", third);

    // Get を使うとSomeかNoneがかえる
    let get_third = v.get(2);
    println!("{:?}", get_third);

    let get_100 = v.get(100);
    println!("{:?}", get_100);

    // Swithでパターンわけできる
    let my_num = 2;
    match v.get(my_num) {
        Some(_) => {
            println!("{:?}", Some(my_num));
        }
        None => {
            println!("NONE");
        }
    }

    let my_num = 100;
    match v.get(my_num) {
        Some(_) => {
            println!("{:?}", Some(100));
        }
        None => {
            println!("NONE");
        }
    }

    // if let
    if let Some(i) = v.get(2) {
        println!("{:?}", Some(i));
    } else {
        println!("none");
    };

    if let Some(i) = v.get(100) {
        println!("{:?}", Some(i));
    } else {
        println!("none");
    }

    // iteration
    let v_immute = vec![1, 2, 3, 4];
    for num in v_immute {
        println!("{}", num);
    }

    // mutable iteration
    for num in &mut v {
        *num += 1
    }

    println!("{:?}", v);

    // ===== String Type =====

    // おなじこと
    let senda = "せんだ".to_string();
    let mituo = String::from("みつお");
    println!("{}, {}", senda, mituo);

    // 足し算できる
    let mut senda_mituo = senda + &mituo;
    println!("{:?}", senda_mituo);

    // StringをPush
    senda_mituo.push_str("みつお");
    println!("{:?}", senda_mituo);

    // CharをPush
    senda_mituo.push('み');
    println!("{:?}", senda_mituo);

    // String interporation by format!
    let senda = "せんだ".to_string();
    let mituo = "みつお".to_string();
    let nahanaha = "なはなは".to_string();

    let say_it = format!("{} - {} - {}", senda, mituo, nahanaha);
    println!("{}", say_it);

    // スライスすれば取れるが、うまくやらないとエラー
    let hello = "Здравствуйте";
    let s = &hello[0..6];
    println!("{}", s);

    // Hash Map
    // All keys must be the same type.
    // All values must be the same type.

    // Creation of HashMap
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 10);

    println!("{:?}", scores);

    // Create HashMap from Vector
    let teams = vec!["Blue".to_string(), "Yellow".to_string()];
    let initials_scores = vec![10, 50];

    // iterでIter Objectを作って、ZIPでくっつける

    // Iter(["Blue", "Yellow"])
    // +
    // Iter([10, 50])
    // =
    // Zip { a: Iter(["Blue", "Yellow"]), b: Iter([10, 50]), index: 0, len: 2 }
    // collect はReturnTypeを読んで適当な値を返してくれる。

    let scores: HashMap<_, _> = teams.iter().zip(initials_scores.iter()).collect();
    println!("{:?}", scores);

    // HashMap takes owntership of whatever passed vars.
    let mut ownership_test = HashMap::new();
    let field_name = "せんだ".to_string();
    let field_value = "みつお".to_string();

    // WITHOUT BORROWING, ERROR!!
    ownership_test.insert(&field_name, &field_value);

    println!("{:?}", field_name);
    println!("{:?}", field_value);
    println!("{:?}", ownership_test);

    // get で取得可能。戻りはSomeかNone
    println!("{:?}", ownership_test.get(&String::from("せんだ")));
    println!("{:?}", ownership_test.get(&String::from("なはなは")));

    for (k, v) in ownership_test {
        println!("{}: {}", k, v);
    }

    // 更新
    let mut u_i_test = HashMap::new();

    u_i_test.insert(String::from("せんだ"), String::from("みつお"));
    u_i_test.insert(String::from("せんだ"), String::from("なはなは"));

    println!("{:?}", u_i_test.get(&String::from("せんだ"))); // -> なはなは

    // Update or Create
    // .entryの戻値 ↓
    // Entry(OccupiedEntry { key: "せんだ", value: "なはなは" })
    // Entry(VacantEntry("なはなは"))
    //

    u_i_test
        .entry(String::from("せんだ"))
        .or_insert(String::from("なはなは"));
    u_i_test
        .entry(String::from("なはなは"))
        .or_insert(String::from("なはなは"));

    println!("{:?}", u_i_test);

    // == or_insert ==
    // pub fn or_insert(self, default: V) -> &'a mut V
    // Ensures a value is in the entry by inserting the default if empty,
    // and returns a mutable reference to the value in the entry.
    //
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").or_insert(12);

    assert_eq!(map["poneyland"], 12);

    *map.entry("poneyland").or_insert(12) += 10;
    assert_eq!(map["poneyland"], 22);

    // or_insertの戻値を利用してWordCount

    let text = "AAA BBB CCC DDD AAA";
    let mut count_map = HashMap::new();
    for word in text.split_whitespace() {
        *count_map.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", count_map);
}
