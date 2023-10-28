
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File ::open("hello.txt").expect("Failed to open hello.txt");

}
