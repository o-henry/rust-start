use std::io;
// 플레이어가 입력한 값을 읽어오려면 입력/출력 라이브러리를 가져와야 한다.

fn main() {
    println!("숫자를 맞혀봅시다!");

    println!("정답이라고 생각하는 숫자를 입력하세요.");

    let mut guess = String::new();
    //                      ____: :: 문법은 new 함수가 String 타입의 연관 함수(정적 메서드) 임을 의미한다.

    io::stdin()
        .read_line(&mut guess)
        .expect("입력한 값을 읽지 못했습니다.");

    println!("입력한 값: {}", guess);
}
