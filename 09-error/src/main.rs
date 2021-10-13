/*
 * Result<T, E> 회복 가능한 에러
 * panic! 회복 불가능한 에러
 */

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    open_file();
}

fn panic() {
    let v = vec![1, 2, 3];
    v[99];
}

fn open_file() {
    let f = File::open("hello.txt"); // Result<File, std::io::Error>

    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일을 생성하지 못했습니다: {:?}", e),
            },
            other_error => panic!("파일을 열지 못했습니다: {:?}", other_error),
        },
    };
}

fn unwrap() {
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("파일을 열 수 없습니다.");
}

fn propagation() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn shortcut_propagation() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
