/**
 * variables
 * 상수 `mut` 사용 불가
 * 선언되어 있는 영역내에서 유효하며, 도메인 전체에서 사용 가능
 **/

const MAX_POINTS: u32 = 100_000; // 상수는 항상 타입을 추가해야한다

fn main() {
    let x = 5; // x is imutable
    println!("The value of x is: {}", x);
    x = 6; // cannot assign twice to immutable variable `x`
           // 컴파일 시점에서 불변 변수로 선언한 변수에 값을 할당할 수 없다는 에러가 발생한다.
    println!("The value of x is: {}", x);

    let mut y = 6;
    println!("The value of y is: {}", y);
    y = 7;
    println!("The value of y is: {}", y);

    // shadowing => 궁금한점, shadowing이 되면 기존 변수 데이터는 삭제 되는지
    let spaces = " abcd  ";
    let spaces = spaces.len(); // 같은 이름의 변수선언 가능
    println!("spaces: {}", spaces)
}
