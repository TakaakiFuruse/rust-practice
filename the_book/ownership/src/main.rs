fn main() {
    // Ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Stack -> stack of trays, Last In First Out. Quick
    // Heap -> Restaurant seat location, slow.

    // Scope
    // Garbage CollectionだとCollectionのタイミングが難しい。
    // そもそもそんなものがない。
    let s1 = "hello";
    {
        // s2宣言前
        let s2 = "YO"; // s2宣言
        println!("{}", s2); // s2有効になる
    } // s2のメモリー開放
    println!("{}", s1);
    // println!("{}", s2); // s2無効なので呼べない

    // str1 owns hello
    // str1 points to hello
    let str1 = String::from("hello");

    // now str2 owns hello
    // str2 points hello
    // no double pointing
    let _str2 = str1;

    println!("{}", _str2); // print str1 does not work

    // Rust will never automatically create “deep” copies of your data.

    // Clone
    // This works. But, It's heap. It's expansive
    let str3 = String::from("hello");
    let str4 = s1.clone();

    println!("{} {}", str3, str4);

    // This is OK.
    // WHy?
    // === Copy traits and Drop trits.===
    //If a type has the Copy trait, an older variable is still usable after assignment.
    // Rust won’t let us annotate a type with the Copy trait if the type,
    // or any of its parts, has implemented the Drop trait.
    //
    // Int, Bool, Char, Float, Tuple are Copy Traits
    let int1 = 1;
    let _int2 = int1;

    let bool1 = true;
    let _bool2 = bool1;

    let float1 = 3.14;
    let _float2 = float1;

    let char1 = '1';
    let _char2 = char1;

    let tup1 = (1, 2, 3);
    let _tup2 = tup1;

    println!("{} {} {} {} {:?}", int1, bool1, float1, char1, tup1);

    // === Ownership and functions===
    //
    // This takes ownership.
    let test1 = String::from("test");
    takes_ownership(test1);
    // This does not work. Func has test1's ownership.
    // println!("{}", test1);

    let x = 5;
    makes_copy(x);
    // This works because x's trait is Copy.
    println!("{}", x);

    // This returns ownership.
    let test2 = String::from("test");
    let test3 = returns_ownership1(test2);
    println!("{}", test3);

    // This too returns ownership.
    let test4 = returns_ownership2();
    println!("{}", test4);
}

fn takes_ownership(a_str: String) {
    println!("{}", a_str);
}

fn returns_ownership1(a_str: String) -> String {
    a_str
}

fn returns_ownership2() -> String {
    let st = String::from("test");
    st
}

fn makes_copy(an_int: i32) {
    println!("{}", an_int);
}
