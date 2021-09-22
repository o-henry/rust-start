fn main() {
    println!("Hello, world!");

    another_function(7);

    let x = five();
    println!("The value of x is : {}", x);

    let y = plus_one(5);
    println!("The value of y is : {}", y);
}

fn another_function(x: i32) {
    // 매개변수의 타입을 반드시 정의해야 한다.
    println!("Hello alien! x is : {}", x)
}

// 구문과 표현식
fn statment() {
    // statement라는 이름의 함수 구문
    let x = 6; // 구문

    let y = {
        let x = 3;
        x + 1
    };
}

fn five() -> i32 {
    5 // 표현식
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
