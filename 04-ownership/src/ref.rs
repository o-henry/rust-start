fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(", tester")
    }

    let r2 = &mut s;
    r2.push_str(", world");

    println!("{}", r2);
}

/* 참조자
 * &(ampersands) => reference
 * 어떠한 값을 소유권을 넘기지 않고 참조할 수 있게 해준다.
 * 실제 값 대신 참조자를 파라미터로 갖고 있는 함수는 소유권을 갖고 있지 않기 때문에 소유권을 되돌려주기 위해 값을 다시 반환할 필요도 없다는 뜻
 *
 * 참조는 기본적으로 불변이다.
 */
fn reference() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", world");
}
