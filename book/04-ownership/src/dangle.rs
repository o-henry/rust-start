/**
 * 죽은 참조(포인터)
 * 이미 해제되어 다른 정보를 저장하도록 변경된 메모리를 계속 참조하는 포인터
 * 러스트에서는 컴파일러가 해당 데이터에 대한 참조를 실행하기전에 범위를 벗어났는가를 확인해준다.
 **/

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s
} // drop s

fn no_dangle() -> String {
    let s = String::from("hello");
    s // return 함으로써, 소유권을 밖으로 이동 합니다.
}
