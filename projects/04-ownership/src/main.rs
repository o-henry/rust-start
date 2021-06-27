fn main() {
    /*
     * String
     */
    let mut base_s = String::from("hello");
    base_s.push_str(", world!");
    //     ________ String은 스트링 리터럴과 다르게 문자열 변경이 가능하다. 이 차이는 메모리를 사용하는 방식에 있다.
    // 해당 타입의 내용물을 컴파일타임 / 런타임에 크기를 알 수 있는지 유무로 선택한다.
    println!("{}", base_s);

    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s);
    //            〰〰 이미 힙에서 drop 되어 재사용 불가능.

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn move_variables() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // 위와 같은 값은 스택에 컴파일 타임에 저장 할 수 있는 타입인데, 이를 Copy 트레잇 이라고 한다.

    let s1 = String::from("hello");
    let s2 = s1; // 포인터, 길이값, 용량값이 복사된다.

    println!("{}, world!", s1);
    //                     〰〰 borrow of moved value: `s1`
    // 얕은 복사와 같아보이지만, s1을 사용할 수 없기 때문에 move라고 한다.
    // 만약 힙 데이터가 복사 되기를 원한다면, clone() 을 사용한다.

    let s3 = s2.clone();
    println!("s2 = {}, s3 = {} world!", s2, s3);
}

/* 소유권 과 함수 */
fn takes_ownership(some_string: String) {
    let some_string = "bye"; // can re-assign → mutable ?!
    println!("some string is : {}", some_string);

    // } 괄호가 닫힐때 자동적으로 drop을 호출 -> 메모리 회수.
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
