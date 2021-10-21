// std 는 러스트의 표준 라이브러리
// 그중 collections 은 매우 유용한 데이터 구조다.
// 내장 배열, 튜플과 달리 컬렉션이 가리키는 데이터는 힙 메모리에 저장된다.
// 즉 컴파일 타임에 알 수 없는 데이터가 필요한 경우 고려할 수 있다.
use std::collections::HashMap;

fn main() {
    // vector는 같은 타입만 저장할 수 있다.
    // enum 은 enum타입으로 평가되ㅑ므로 벡터에 다른 타입을 저장하려면 열거자를 정의해서 사용하면 된다.
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let mut v2 = vec![1, 2, 3, 4]; // 타입 추론

    for i in &v2 {
        println!("{}", i);
    }

    for i in &mut v2 {
        *i += 50;
        println!("{}", i)
    }

    match v.get(2) {
        Some(third) => println!("세 번째 원소: {}", third),
        None => println!("세 번째 원소가 없습니다."),
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("블루")),
    ];

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("{}", s);

    let mut scores = HashMap::new();

    scores.insert(String::from("블루"), 10);
    scores.insert(String::from("레드"), 50);
}
