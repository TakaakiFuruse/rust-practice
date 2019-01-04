#![allow(dead_code)]

use types_traits::Mituo;
use types_traits::NahaNaha;
use types_traits::NewsArticle;
use types_traits::SayIt;
use types_traits::Senda;
use types_traits::ShoutOut;
use types_traits::Summary;
use types_traits::Tweet;

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest_char = list[0];

    for &item in list.iter() {
        if item > largest_char {
            largest_char = item;
        }
    }

    largest_char
}

// PartialOrd + Copy なTraitだけ扱うようにする。
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];
    for item in list.iter() {
        if item > &largest {
            largest = *item
        }
    }
    largest
}

// return a reference to a T value in the slice
fn largest2<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = &item
        }
    }
    largest
}

#[derive(Debug)]
struct Point1<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
struct Point3<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point3<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<V, W>(self, other: Point2<V, W>) -> Point3<T, W> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 56];

    let result = largest_i32(&number_list);

    println!("largest num: {}", result);

    let number_list = vec![341, 52340, 2_235_235, 112_300, 55526];
    let result = largest_i32(&number_list);

    println!("largest num: {}", result);

    let char_list = vec!['y', 'm', 's', 'q'];
    let result1 = largest_char(&char_list);

    println!("largest char {}", result1);

    println!("{}", largest(&number_list));
    println!("{}", largest2(&number_list));

    // OK
    println!("{:?}", Point1 { x: 1, y: 2 });

    // だめ、2つのタイプが違う
    // println!("{:?}", Point1 { x: 1, y: 2.0 });

    // OK, タイプを追加した
    println!("{:?}", Point2 { x: 1, y: 2.0 });

    // mixup
    let p1 = Point3 { x: 5, y: 1.4 };
    let p2 = Point2 { x: 4, y: 2.14 };

    println!("{}", p1.x());
    println!("{:?}", p1.mixup(p2));

    let news = NewsArticle {
        headline: "IMPORTANT!!".to_string(),
        location: "TOKYO".to_string(),
        author: "ME".to_string(),
        content: "SUPER IMPORTANT!!".to_string(),
    };

    println!("{}", news.summarize());
    println!("{}", news.read_more());
    println!("{}", news.paywall());

    let tweet = Tweet {
        username: "ME".to_string(),
        content: "super important".to_string(),
        reply: true,
        retweet: false,
    };

    println!("{}", tweet.summarize());
    println!("{}", tweet.read_more());
    println!("{}", tweet.paywall());

    // 下と同じ結果になる
    notify(&tweet);
    notify(&news);

    // 上と同じ結果になる
    // notify_verbose(&tweet);
    // notify_verbose(&news)

    // Using Trait Bounds to Conditionally Implement Methods
    let senda = Senda {
        shout: "せんだ".to_string(),
    };
    let mituo = Mituo {
        shout: "みつお".to_string(),
    };
    let nahanaha = NahaNaha {
        shout: "なはなは".to_string(),
    };

    // ひとつだけSayItがない状態
    println!(
        "{}, {}, {}",
        senda.the_call(),
        mituo.the_call(),
        nahanaha.the_call()
    );

    let shout1 = ShoutOut { obj: senda };
    let shout2 = ShoutOut { obj: mituo };
    let shout3 = ShoutOut { obj: nahanaha };

    // kimoi は全部あるが
    println!("{}, {}, {}", shout1.kimoi(), shout2.kimoi(), shout3.kimoi());

    // shoutout は shout3 にはない
    println!("{}, {}", shout1.shoutout(), shout2.shoutout(),);

    // 下をやるとエラー。タイプがついてこないので、メソッドがない。
    // println!("{}", shout3.shoutout());
    // = note: the method `shoutout` exists but the following trait bounds were not satisfied:
    // `types_traits::NahaNaha : types_traits::SayIt`
}

// traits as arguments (syntax sugar for verbose one)
pub fn notify(item: &impl Summary) {
    println!("New {} !!", item.article_type());
}

// trait bounds
pub fn notify_verbose<T: Summary>(item: &T) {
    println!("Breaking new {}!!", item.article_type());
}

// traits as arguments (2つ書くのはめんどい)
// pub fn notify2(item1: impl Summary, item2: imple Summary) {
//     println!("New {} {}!!", item1.article_type(), item2.article_type());
// }

// trait bounds (こっちのほうが簡潔)
// pub fn notify_verbose2<T: Summary>(item1: T, item2: T) {
//     println!("Breaking new {} {}!!", item1.article_type(), item2.article_type());
// }

// where clause (defining multiple traits in pretty way)
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {

// Expressing bounds with a `where` clause
// fn some_function<T, U>(t: T, u: U) -> i32
// where T: Display + Clone,
//       U: Clone + Debug
// {}
