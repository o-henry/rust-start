use rand::Rng;
use std::io;

fn main() {
    println!("숫자를 맞혀봅시다!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("사용자가 맞혀야할 숫자: {}", secret_number);

    println!("정답이라고 생각하는 숫자를 입력하세요.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("입력한 값을 읽지 못했습니다.");

    println!("입력한 값: {}", guess);
}
