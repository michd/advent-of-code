use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process(input);
    println!("{output}");
}

fn process(input: Vec<String>) -> String {
    format!("Output")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}
