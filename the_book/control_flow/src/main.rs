fn main() {
    // loop
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        };
    };

    // 20じゃないとPanicする
    assert_eq!(result, 20);

    // whileで上と同じことをする。
    // Nestが少ないので良い
    let mut counter2 = 0;
    while counter2 != 10 {
        counter2 += 1;
    }
    let counter2 = counter2 * 2;
    assert_eq!(counter2, 20);

    // for
    for num in 1..13 {
        // ふつーにFizBuz
        if num % 6 == 0 {
            println!("SIX!!");
        } else if num % 3 == 0 {
            println!("THREE!");
        } else if num % 2 == 0 {
            println!("TWO!!")
        } else {
            println!("{}", num);
        };

        // 結果を変数に代入しても良い
        // return types を合わせないと動かない
        let result = if num % 6 == 0 {
            "SIX!!".to_string()
        } else if num % 3 == 0 {
            "THREE!".to_string()
        } else if num % 2 == 0 {
            "TWO!!".to_string()
        } else {
            num.to_string()
        };
        println!("rsult is {}", result)
    }
}
