#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 別にselfでも動く。Ownership取りたくないのでこうする
    // self は Rectangle object
    fn area(&self) -> u32 {
        println!("self -> {:?}", self);
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    // Code 1 - just use vars.
    let width1 = 30;
    let height1 = 50;

    println!("Area is {} pixels", area(width1, height1));

    // Code 2 - use tuples.

    let rect1 = (30, 50);

    println!("Area is {} pixels", area2(rect1));

    // Code 2 - use structs
    let rect2 = Rectangle{width: 30, height: 50};
    // in order to keep ownership, make rect as imuutable borrow.
    println!("Area is {} pixels", area3(&rect2));

    // Using Method
    println!("Area is {} pixels", rect2.area());

    let rect3 = Rectangle{width: 40, height: 50};
    let rect4 = Rectangle{width: 30, height: 60};
    let rect5 = Rectangle{width: 30, height: 50};

    println!("rect2 can hold rect3 ? -> {}", rect2.can_hold(&rect3));
    println!("rect2 can hold rect4 ? -> {}", rect2.can_hold(&rect4));
    println!("rect2 can hold rect5 ? -> {}", rect2.can_hold(&rect5));

    // SELFを取らないメソッドで正方形を作る
    let sq = Rectangle::square(3);

    println!("Square is {:?}", sq);

}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
