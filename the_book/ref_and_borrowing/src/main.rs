// ==Code Without Borrow===
// fn main() {
//     let s1 = String::from("hello");
//     let (s1, len) = calculate_length(s1);
//     println!("{} {}", s1, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

fn main() {
    // ==Code With Borrow===
    let s1 = String::from("hello");
    // "&" are references, allow you to refer to some value without taking ownership
    let len = calculate_length(&s1);
    print!("{}, {}", s1, len);

    // === Mutable Ref ===
    let mut s2 = String::from("hello"); // Make s2 as mut
    change(&mut s2); // Create mutable ref here

    // === Multi-Mutable Ref is not allowed ===

    // This fails
    let mut s3 = String::from("hello");
    let ss1 = &mut s3;
    let ss2 = &mut s3;
    println!("{}, {}", ss1, ss2);

    // This is OK.
    let mut s3 = String::from("hello");
    {
        let _ss1 = &mut s3;
    }
    let ss2 = &mut s3;
    println!("{}", ss2);

    // Can't mix mut and immut refs.
    let mut s4 = String::from("hello");
    let s41 = &s4;
    let s42 = &s4;
    let s43 = &mut s4;
    println!("{}, {}, {}", s41, s42, s43);

    // ==== Dangling Pointer ====
    // This fails
    let ref1 = dangle();

    // Thi is OK
    let _ref1 = non_dangle();

    // === slice type ===
    //

    // === This panicks Rust ===
    // Clear s5 before printing out.
    // Rust panicks
    // We have no s5 anymore but reffering to it.

    let mut s5 = String::from("hello world");
    let word_index = first_word(&s5);
    s5.clear();
    println!("{}", &s5[..word_index]);

    // === This does not panick Rust ===
    // Only compile error.

    let mut s5 = String::from("hello world");
    let word = first_word2(&s5); // <- immutable borrow
    s5.clear(); // <- mutable borrow
    println!("{}", word); // <- immutable borrow

    // &str
    //     You can only ever interact with str as a borrowed type aka &str.
    //     This is called a string slice, an immutable view of a string.
    //     This is the preferred way to pass strings around, as we shall see.

    // &String
    //     This is a reference to a String, also called a borrowed type.
    //     This is nothing more than a pointer which you can pass around without giving up ownership.
    //     Turns out a &String can be coerced to a &str:

    // 渡すなら&strにしといたほうがラク
    // If we have a string slice, we can pass that directly.
    // If we have a String, we can pass a slice of the entire String.
    // ArrayのSliceも同様
    let my_string = String::from("hello world");

    let _word = first_word3(&my_string[..]);

    let my_string_literal = "hello world";

    let _word = first_word3(&my_string_literal[..]);

    let _word = first_word3(my_string_literal);
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn non_dangle() -> String {
    let s = String::from("hello");
    s
}

fn dangle() -> &String {
    let s = String::from("hello");

    // Here we returning.
    // Returning s But memories will be freed.
    // It's reffering nothing.
    &s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    // Also create mut ref here.
    some_string.push_str(", world!")
}
