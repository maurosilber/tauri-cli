use std::io::{self, BufRead};
use std::process;

pub fn main() {
    println!("Write 'exit' to exit.");
    let stdin = io::stdin();
    for line in stdin.lock().lines().flatten() {
        if line.as_str() == "exit" {
            process::exit(0)
        }
    }
}
