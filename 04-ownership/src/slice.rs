/**
 * 슬라이스도 소유권을 갖지 않는 타입이다.
 * 컬렉션 전체가 아닌 내부의 연속된 요소들을 참조할 수 있다.
 **/

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
