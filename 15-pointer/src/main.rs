use crate::List::{Cons, Nil};
use std::ops::Deref;
fn main() {
    let b = Box::new(5); // i32 값을 힙 메모리에 저장
    println!("b = {}", b);

    // 컴파일 타임에 크기를 알 수 없는 타입 중 하나인 재귀 타입, 러스트는 재귀타입의 값을 저장하는데 필요한 공간을 판단할 수 없다.
    // 하지만 박스의 크기는 정해져 있으므로, 재귀 타입을 선언할때 박스를 활용해야 한다.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let z = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 역참조 -> 변수 y의 값을 검증하려면 *y처럼 역참조를 통해 변수가 가리키는 값의 참조를 따라가야 한다.
    assert_eq!(5, *z);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
