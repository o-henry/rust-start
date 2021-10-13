fn main() {
    /*
     * 소유권
     * 스택 / 힙
     * 변수의 범위
     * 메모리와 할당
     * Move / Clone / Copy
     * Drop
     * 소유권과 함수
     *
     *
     * String type => 힙에 할당되는 타입으로, 컴파일 시점에 알 수 없는 크기의 문자열을 저장할 수 있다.
     * 힙 메모리에 저장된 변수의 데이터는 소유권이 다른 변수로 옮겨지지 않았다면 범위를 벗어날때 drop함수에 의해 제거된다.
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
    println!("{}", x); // i32는 Copy 되므로, x는 스코프 밖에서도 사용 가능.

    let _s1 = gives_ownership();
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);
}

fn move_variables() {
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // 위와 같은 값은 스택에 컴파일 타임에 저장 할 수 있는 타입인데, 이를 Copy 트레잇 이라고 한다.

    let s1 = String::from("hello");
    let s2 = s1; // 힙 메모리에 실제 데이터가 아닌 String 타입의 데이터(포인터, 길이값, 용량)이 스택에 복사된다.

    println!("{}, world!", s1);
    //                     〰〰 borrow of moved value: `s1`
    // s1을 무효화함으로써 메모리상 오류를 방지한다.
    // 얕은 복사와 같아보이지만, s1을 사용할 수 없기 때문에 move라고 한다.
    // 만약 힙 데이터가 복사 되기를 원한다면, clone() 을 사용한다.

    let s3 = s2.clone();
    println!("s2 = {}, s3 = {} world!", s2, s3);
}

/* 소유권 과 함수 */
// 변수를 함수에 전달하면 대입과 마찬가지로 값의 이동이나 복사가 이루어진다.
fn takes_ownership(some_string: String) {
    println!("some string is : {}", some_string);
} // } 괄호가 닫힐때 자동적으로 drop을 호출 -> 메모리 해제.

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// return 값도 소유권을 이전한다.
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
