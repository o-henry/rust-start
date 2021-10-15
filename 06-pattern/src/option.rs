// 어떤 값의 존재 여부를 표현하는 열거자 Option<T>
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None; // Some 대신 None을 사용하면 러스트에게 Option<T> 열거자의 타입이 무엇인지 알려줘야 한다.
                                           // 왜냐하면 컴파일러는 None 값만으로는 Some값이 어떤 타입을 저장할 것인지 알 수 없기 때문
                                           // 어떤 값이 Null 값을 가질수 있도록 하려면 Option<T> 값을 명시적으로 사용해야 한다.

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;

    value_in_cents(Coin::Quarter(UsState::Alaska));
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
