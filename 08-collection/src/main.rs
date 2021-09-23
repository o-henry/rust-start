fn main() {
    read();
}

fn basic() {
    // 빈 벡터 생성
    // let v: Vec<i32> = Vec::new();
    // let mv = vec![1, 2, 3]; // 타입 추론

    let mut v = Vec::new();

    // 요소 추가
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
} // drop

fn read() {
    let mut v = vec![1, 2, 3, 4, 5];

    // 저장된 값에 대한 참조를 리턴한다.
    let third: &i32 = &v[2];
    println!("세 번째 원소: {}", third);

    // get 메서드는 Option<&T> 타입의 값을 리턴한다.
    match v.get(2) {
        Some(third) => println!("세 번째 원소: {}", third),
        None => println!("세 번째 원소가 없습니다."),
    }

    let does_not_exist = &v[100]; // panic 존재하지 않는 값을 가리키기 때문에 강제종료된다.
    let does_not_exist = v.get(100); // 사용자 친화적

    for i in &mut v {
        *i += 50;
    }
}

fn use_enum_vec() {
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // 같은 타입만 저장할 수 있는 벡터는 enum을 enum타입으로 보기 때문에 활용가능하다.
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("블루")),
        SpreadSheetCell::Float(10.12),
    ];
}

fn string_type() {
    let mut s = String::from("foo");
    let s1 = "bar";
    s.push_str(s1);
    println!("s1: {}", s1);
}

// hash map === hash table === dictionary === associative array
fn hash_map() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
}
