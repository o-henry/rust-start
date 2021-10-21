fn main() {
    // 사용 가능한 값만 나열한 타입을 정의할 때 사용하는 enum
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        _ => (), // 자리지정자
    }

    let some_u7_value = Some(0u8);

    if let Some(3) = some_u7_value {
        println!("three")
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    }
}
