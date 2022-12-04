use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_one(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    format!("Output")
}

fn process_part_two(input: Vec<String>) -> String {
    format!("Output")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn testing_works() {
        assert_eq!(2,  1 + 1);
    }
}
