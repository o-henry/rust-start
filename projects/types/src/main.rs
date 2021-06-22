/**
 * data-type
 **/

fn main() {
    let guess: u32 = "42".parse().expect("Not a Number!");

    let x = 2.0;
    let y: f32 = 3.0;
    let t = true;
    let f: bool = false;
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a, b, c) = tup;
    println!("The value of b is : {} ", b);

    let five_hundred = tup.0;
    println!("The value of five_hundred is : {} ", five_hundred);

    let array = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    println!("The value of eleements is: {} ", array[10]);
}
