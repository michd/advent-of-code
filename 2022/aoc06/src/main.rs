use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let start = locate_start_of_packet(&input[0], 4);
    format!("{start}")
}

fn process_part_two(input: Vec<String>) -> String {
    // Lucky guess lol
    let start = locate_start_of_packet(&input[0], 14);
    format!("{start}")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

fn locate_start_of_packet(input: &str, marker_length: usize) -> usize {
    for start in 0..input.chars().count() {
        let slice = &input[start..(start + marker_length)];
        let mut chars: Vec<char> = slice.chars().collect();
        chars.sort();
        chars.dedup();
        if chars.len() == marker_length {
            return start + marker_length
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn locates_start_of_packet() {
        assert_eq!(
            locate_start_of_packet(
                "bvwbjplbgvbhsrlpgdmjqwftvncz",
                4,
            ),
            5,
        );

        assert_eq!(
            locate_start_of_packet(
                "nppdvjthqldpwncqszvftbrmjlhg",
                4,
            ),
            6,
        );


        assert_eq!(
            locate_start_of_packet(
                "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
                4,
            ),
            10,
        );

        assert_eq!(
            locate_start_of_packet(
                "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
                4,
            ),
            11,
        );

    }
}
