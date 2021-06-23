fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");

    forloop();
    range();
}

// 가장 보편적이고 안전하고 간결한 반복문 for
fn forloop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is : {}", element)
    }
}

fn range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!")
}
