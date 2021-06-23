fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    println!("{}", s); // 이미 힙에서 drop 되어 재사용 불가능.

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    let some_string = "bye"; // can re-assign → mutable ?!
    println!("some string is : {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

// 4.2 부터
