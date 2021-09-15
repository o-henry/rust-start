fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // return 함으로써, 소유권을 밖으로 이동 합니다.
}
