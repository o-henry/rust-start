use crate::List::{Cons, Nil};
fn main() {
    let b = Box::new(5); // i32 값을 힙 메모리에 저장
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
