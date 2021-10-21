#[derive(Debug)] // 구조체는 디버깅 정보를 출력하기 위한 이 기능을 명시적으로 구현해주어야 한다.
struct Rectangle {
    height: u32,
    width: u32,
}

// method
impl Rectangle {
    // 메서드는 fn 키워드를 통해 정의하며 구조체의 컨텍스트 안에 정의하며 첫번째 매개변수는 항상 self여야 한다.
    fn area(&self) -> u32 {
        self.height * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // 생성자를 구현할때 사용
    // 연관 함수
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

    println!("rect1 is {:#?}", rect1);
    println!("rect1은 rect2를 포함하는가? {}", rect1.can_hold(&rect2));

    println!(
        "The area of the rectangle is {} square pixels. ",
        rect1.area()
    );

    let sq = Rectangle::square(3);
    println!("square {:#?}", sq);
}
