use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // - 생략 -

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("입력한 값을 읽지 못했습니다.");

    let guess: u32 = guess
        //  _____ : shadowing
        .trim()
        .parse()
        .expect("입력한 값이 올바른 숫자가 아닙니다.");

    println!("입력한 값: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => prinln!("입력한 숫자가 작습니다!"),
        Ordering::Greater => println!("입력한 숫자가 큽니다!"),
        Ordering::Equal => println!("정답!"),
    }
}
