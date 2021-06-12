//     _____ crate : rust's package
extern crate rand;
//       __ The io library -> comes from the standard library (which is known as std)
//      __ :: syntax -> indicates that io is an associated function of the std type. 연관함수 임을 표기합니다.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input ypur guess.");

        //  ___ storing values with mutable(mut) variables(guess)
        let mut guess = String::new();
        //             _ &: The & indicates that this argument is a reference, immutable. 데이터를 여러 번 메모리로 복사하지 않고 접근하기 위한 방법을 제공
        //   _________ read_line : return io::Result -> Ok | Err, 에러 처리를 위한 정보 표현
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadow
        //                                  _____ parse : return Result type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // compare guess to 'parameter'
        //          ___ cmp : return Ordering
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
