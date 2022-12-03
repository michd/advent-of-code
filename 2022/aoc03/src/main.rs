use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let total = input
        .iter()
        .map(|l| Rucksack::new(l.to_string()))
        .fold(0, |acc, rucksack| {
            acc + get_priority(
                get_item_type_in_both(
                    rucksack.compartment_one(),
                    rucksack.compartment_two(),
                ).unwrap()
            )
        });

    format!("{total}")
}

fn process_part_two(input: Vec<String>) -> String {
    let mut groups = vec![];

    for (index, _) in input.iter().enumerate().step_by(3) {
        groups.push(
            vec![
                input.get(index).unwrap(),
                input.get(index + 1).unwrap(),
                input.get(index + 2).unwrap(),
            ]
        );
    }

    let total = groups
        .iter()
        .fold(0, |acc, g| {
            acc + get_priority(
                get_item_type_in_all(
                    g.get(0).unwrap(),
                    g.get(1).unwrap(),
                    g.get(2).unwrap(),
                ).unwrap(),
            )
        });

    format!("{total}")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

fn get_priority(item_type: char) -> u32 {
    let ascii = item_type as u32;

    match ascii {
        // A-Z
        65..=90 => ascii - 64 + 26,
        // a-z
        97..=122 => ascii - 96,
        _ => 0,
    }
}

fn get_item_type_in_both(left: &str, right: &str) -> Option<char> {
    let mut left_chars: Vec<char> = left.chars().collect();
    left_chars.sort();
    left_chars.dedup();

    let mut right_chars: Vec<char> = right.chars().collect();
    right_chars.sort();
    right_chars.dedup();

    for c in left_chars.iter() {
        if right_chars.iter().any(|&cr| cr == *c) {
            return Some(*c)
        }
    }

    None
}

fn get_item_type_in_all(one: &str, two: &str, three: &str) -> Option<char> {
    let mut one_chars: Vec<char> = one.chars().collect();
    one_chars.sort();
    one_chars.dedup();

    let mut two_chars: Vec<char> = two.chars().collect();
    two_chars.sort();
    two_chars.dedup();

    let mut three_chars: Vec<char> = three.chars().collect();
    three_chars.sort();
    three_chars.dedup();

    let mut remaining = one_chars.clone();

    remaining.retain(|i| {
        two_chars.iter().any(|c| *c == *i) && three_chars.iter().any(|c| *c == *i)
    });

    remaining.get(0).copied()
}

// Only used in part one
struct Rucksack {
    item_types: String
}

impl Rucksack {
    fn new(input: String) -> Rucksack {
        Rucksack { item_types: input }
    }

    fn compartment_one(&self) -> &str {
        &self.item_types[0..(self.item_types.len() / 2)]
    }

    fn compartment_two(&self) -> &str {
        &self.item_types[(self.item_types.len() / 2)..]
    }
}

#[cfg(test)]
mod tests {
    use crate::Rucksack;
    use crate::get_priority;
    use crate::get_item_type_in_both;
    use crate::get_item_type_in_all;

    #[test]
    fn rucksack_splits_compartments() {
        assert_eq!(
            Rucksack::new(format!("AABB")).compartment_one(),
            "AA",
        );

        assert_eq!(
            Rucksack::new(format!("AABB")).compartment_two(),
            "BB",
        );
    }

    #[test]
    fn get_priority_reports_correct_values() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('b'), 2);
        assert_eq!(get_priority('y'), 25);
        assert_eq!(get_priority('z'), 26);
        assert_eq!(get_priority('A'), 27);
        assert_eq!(get_priority('B'), 28);
        assert_eq!(get_priority('Y'), 51);
        assert_eq!(get_priority('Z'), 52);
    }

    #[test]
    fn get_item_type_in_both_correct() {
        assert_eq!(
            get_item_type_in_both("vJrwpWtwJgWr", "hcsFMMfFFhFp").unwrap(),
            'p',
        );

        assert_eq!(
            get_item_type_in_both("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL").unwrap(),
            'L',
        );

        assert_eq!(
            get_item_type_in_both("PmmdzqPrV", "vPwwTWBwg").unwrap(),
            'P',
        );
    }

    #[test]
    fn get_item_type_in_all_correct() {
        assert_eq!(
            get_item_type_in_all(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
            ).unwrap(),
            'r',
        );

        assert_eq!(
            get_item_type_in_all(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            ).unwrap(),
            'Z',
        );
    }
}

