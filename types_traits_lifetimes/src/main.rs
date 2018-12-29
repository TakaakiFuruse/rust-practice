use types_traits_lifetimes::NewsArticle;
use types_traits_lifetimes::Summary;

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

// これでは動かない
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list.iter() {
//         if item > largest {
//             largest = item
//         }
//     }
//     largest
// }

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

    let number_list = vec![341, 52340, 2235235, 112300, 55526];
    let result = largest_i32(&number_list);

    println!("largest num: {}", result);

    let char_list = vec!['y', 'm', 's', 'q'];
    let result = largest_char(&char_list);

    println!("largest char {}", result);

    // OK
    println!("{:?}", Point1 { x: 1, y: 2 });

    // だめ、2つのタイプが違う
    // println!("{:?}", Point1 { x: 1, y: 2.0 });

    // OK, タイプを追加した
    println!("{:?}", Point2 { x: 1, y: 2.0 });

    // mixup
    let p1 = Point3 { x: 5, y: 1.4 };
    let p2 = Point2 { x: 4, y: 3.14 };

    println!("{}", p1.x());
    println!("{:?}", p1.mixup(p2));

    let news = NewsArticle {
        headline: "IMPORTANT!!".to_string(),
        location: "TOKYO".to_string(),
        author: "ME".to_string(),
        content: "SUPER IMPORTANT!!".to_string(),
    };

    println!("{}", news.summarize());
}
