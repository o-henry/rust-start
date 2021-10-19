/**
 * data-type
 **/

fn main() {
    let guess: u32 = "42".parse().expect("Not a Number!");

    let x = 2.0f32;
    let y: f32 = 3.0;
    let t = true;
    let f: bool = false;
    let tup: (i32, f64, u8) = (500, 6.4, 1); // tuple type

    let (a, b, c) = tup;
    println!("The value of b is : {} ", b); // 6.4

    let five_hundred = tup.0; // dot notation 으로 접근 가능하나, 좋은 접근은 아니다. 어떤 값에 접근하는지 모르기 때문
    println!("The value of five_hundred is : {} ", five_hundred); // 500

    let array: [i32; 5] = [1, 2, 3, 4, 5]; // 배열의 자료형은 [T;N]로 표현하며, T는 원소의 자료형 N은 컴파일 타임에 주어지는 고정된 길이다.
    let array = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // println!("The value of eleements is: {} ", array[10]);
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
    // return (x, y);
}

fn main() {
    // 리턴 값의 튜플을 리턴
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // 튜플을 두 변수명으로 분해
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}
