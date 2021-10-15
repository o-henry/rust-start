#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

// method
impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 생성자를 구현할때 사용
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // 초기화
    let rect1 = Rectangle {
        height: 50,
        width: 30,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("rect1 is {:#?}", rect1);
    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));
    println!("rect1은 rect3를 포함하는가? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels. ",
        rect1.area()
    );

    let sq = Rectangle::square(3);
    println!("square {:#?}", sq);
}
