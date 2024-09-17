// Security
#![forbid(unsafe_code)]

use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("Hello, world!");
}

#[ctor::ctor]
fn foo() {
    let mut test_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("test.txt")
        .unwrap();
    writeln!(test_file, "Hello, world!").unwrap();
}
