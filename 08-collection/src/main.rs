use std::collections::HashMap;

fn main() {
    // vector는 같은 타입만 저장할 수 있다.
    // enum 은 enum타입으로 평가되ㅑ므로 벡터에 다른 타입을 저장하려면 열거자를 정의해서 사용하면 된다.

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
