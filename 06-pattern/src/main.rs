fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;

    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // 자리지정자
    let u8_value = 0u8;
    match u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // 자리지정자
    }

    // if let 문법은 주어진 값에 대해 하나의 패턴만을 검사하고 나머지값은 무시한다.
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three")
    }
}

// 별도의 구조체 선언 없이 enum 값에 데이터를 직접 바인딩
// enum 값에는 어떤 종류의 데이터도 저장할 수 있다.
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 어떤 값의 존재여부를 표현하는 열거자 Option<T>
// enum Option<T> {
//     Some(T),
//     None,
// }

// match
// 일련의 패턴과 값을 비교해 일치하는 패턴에 지정된 코드를 실행.
// 패턴은 리터럴, 변수명, 와일드카드 등 다양한 값으로 구성가능하다.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// match 키워드 다음 비교할 표현식 작성
// if와 다르게 bool 값이 아닌 어떤 타입의 값도 사용 가능하다.
// pattern => value
// 모든 경우를 처리해야 한다.

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("행운의 페니!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
