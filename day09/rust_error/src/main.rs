fn main() {
    // raise_panic();
    // open_error();
    // open_error_match();
    // test_unwrap();
    // test_expect();
    test_broadcast();
}

fn raise_panic() {
    panic!("crash and burn");
}

use std::fs::File;
fn open_error() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file, error {:?}", error);
        }
    };
}

use std::io::ErrorKind;
fn open_error_match() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file, error {:?}", error),
            },
            other_error => panic!("Failed to open file, error {:?}", other_error),
        },
    };
}

fn test_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

fn test_expect() {
    let f = File::open("hello.txt").expect("Failed to open file");
}

fn test_broadcast() {
    let f = error_broadcast();
    match f {
        Err(error) => println!("get error, err={}", error),
        Ok(f) => println!("get file successfully"),
    }
}

use std::io;
fn error_broadcast() -> Result<File, io::Error> {
    let f = File::open("hello.txt")?;
    Ok(f)
}
