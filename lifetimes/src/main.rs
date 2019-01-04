fn main() {
    // // 動く例
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("{}", result);

    // // これも問題ない
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("{}", result);
    }

    // これが
    let mut data = vec![1, 2, 3];
    let x = &data[0];
    data.push(4);
    println!("{}", x);

    // こういうふうに処理される
    'a: {
        let mut data: Vec<i32> = vec![1, 2, 3];
        'b: {
            // 'b is as big as we need this borrow to be
            // (just need to get to `println!`)
            let x: &'b i32 = Index::index::<'b>(&'b data, 0);
            'c: {
                // Temporary scope because we don't need the
                // &mut to last any longer.
                // ここで data の lifetimeが c になる
                // bの終わりまで生きなくなる。エラー!
                Vec::push(&'c mut data, 4);
            }
            println!("{}", x);
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// これでは動かない
// A の中にできた B というLifetimeで、resultが死んでしまうから。
fn longest2<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}

// StringをStaticにすることでLifetimeをいじくる
// こうすると動く
static STRING: &str = "really long string";

#[allow(unused_variables)]
fn longest3<'a>(x: &str, y: &str) -> &'a str {
    let result = STRING;
    result
}

// こうすると
fn as_str(data: &u32) -> &str {
    let s = format!("{}", data);
    &s
}

// 内部では実はこうなる
fn as_str<'a>(data: &'a u32) -> &'a str {
    'b: {
        let s = format!("{}", data);
        // sのLiftimeは一番大きいA。
        // が、sはB(Aより小さいLifetime)が終わると死ぬ。
        // これでは値が返せない。エラー!!
        return &'a s;
    }
}


// #[allow(unused_variables)]
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

static STRING: &str = "really long string";

#[allow(unused_variables)]
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = STRING;
    result
}
