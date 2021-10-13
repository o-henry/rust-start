fn main() {
    let s1 = gives_ownership();
    println!("test : {}", s1);

    let mut s2 = String::from("hello");

    let s3 = takes_and_gives_back(&mut s2);
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: &mut String) {
    // 추가적인 처리가 가능한가 테스트
    a_string.push_str("Bye");
}
