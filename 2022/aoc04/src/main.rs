use std::io;
use std::io::BufRead;
use std::ops::RangeInclusive;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let full_containment = input.iter().fold(0, |acc, line| {
        let (rangeA, rangeB) = parse_range_pair(line);

        if is_range_contained_in_other(rangeA, rangeB) {
            acc + 1
        } else {
            acc
        }
    });

    format!("{full_containment}")
}

fn process_part_two(input: Vec<String>) -> String {
    let overlaps = input.iter().fold(0, |acc, line| {
        let (rangeA, rangeB) = parse_range_pair(line);

        if overlaps(rangeA, rangeB) { 
            acc + 1
        } else {
            acc
        }
    });

    format!("{overlaps}")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

fn parse_range(str_range: &str) -> RangeInclusive<i32> {
    let ints: Vec<i32> = str_range
        .split('-')
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect();

    ints[0]..=ints[1]
}

fn parse_range_pair(str_ranges: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let pairs: Vec<&str> = str_ranges
        .split(',')
        .collect();

    (parse_range(pairs[0]), parse_range(pairs[1]))
}

fn is_range_contained_in_other(
    rangeA: RangeInclusive<i32>,
    rangeB: RangeInclusive<i32>,
) -> bool {
    let (aStart, aEnd) = (*rangeA.start(), *rangeA.end());
    let (bStart, bEnd) = (*rangeB.start(), *rangeB.end());

    aStart >= bStart && aEnd <= bEnd || bStart >= aStart && bEnd <= aEnd
}

fn overlaps(rangeA: RangeInclusive<i32>, rangeB: RangeInclusive<i32>) -> bool {
    rangeA.contains(rangeB.start()) || rangeA.contains(rangeB.end()) ||
        rangeB.contains(rangeA.start()) || rangeB.contains(rangeA.end())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_range_works() {
        assert_eq!(parse_range("2-4"), 2..=4);
        assert_eq!(parse_range("6-8"), 6..=8);
        assert_eq!(parse_range("2-3"), 2..=3);
        assert_eq!(parse_range("1337-9001"), 1337..=9001);
    }

    #[test]
    fn parse_range_pair_works() {
        assert_eq!(
            parse_range_pair("2-4,6-8"),
            (2..=4, 6..=8),
        );

        assert_eq!(
            parse_range_pair("2-3,4-5"),
            (2..=3, 4..=5),
        );

        assert_eq!(
            parse_range_pair("5-7,7-9"),
            (5..=7, 7..=9),
        );

        assert_eq!(
            parse_range_pair("1337-9001,6052-8080"),
            (1337..=9001, 6052..=8080),
        );
    }

    #[test]
    fn range_containment_check() {
        assert!(is_range_contained_in_other(3..=5, 2..=8));
        assert!(is_range_contained_in_other(2..=8, 3..=5));
        assert!(is_range_contained_in_other(1..=5, 1..=4));
        assert!(is_range_contained_in_other(1..=4, 1..=5));
        assert!(is_range_contained_in_other(1..=5, 3..=5));
        assert!(is_range_contained_in_other(3..=5, 1..=5));
        assert!(is_range_contained_in_other(4..=9, 4..=9));

        assert!( ! is_range_contained_in_other(3..=5, 1..=4));
        assert!( ! is_range_contained_in_other(1..=4, 3..=5));
        assert!( ! is_range_contained_in_other(1..=4, 6..=8));
        assert!( ! is_range_contained_in_other(6..=8, 1..=4));
        assert!( ! is_range_contained_in_other(1..=5, 5..=7));
        assert!( ! is_range_contained_in_other(5..=7, 1..=5));
    }

    #[test]
    fn range_overlap_check() {
        assert!(overlaps(5..=7, 7..=9));
        assert!(overlaps(7..=9, 5..=7));
        assert!(overlaps(2..=8, 3..=7));
        assert!(overlaps(3..=7, 2..=8));
        assert!(! overlaps(3..=5, 6..=9));
    }
}
