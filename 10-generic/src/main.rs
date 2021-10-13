fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("가장 큰 숫자: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = generic_largest(&char_list);
    println!("가장 큰 문자: {}", result);
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generic_largest<T>(list: &[T]) -> T {}
