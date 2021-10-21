// p152
// 구조체, 열거자, 혹은 기타 아이템을 use 구문으로 가져올때는 전체 경로를 사용하는것이 관용적이다.
use std::collections::HashMap;

use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("{}", secret_number);
}
