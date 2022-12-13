use std::cmp::Ordering;
use std::io;
use std::io::BufRead;
extern crate json;
use json::JsonValue;

// I was going to roll my own parser but it didn't work out, and I have limited
// time today, so I opted for just parsing it as json.
//
// Could rework further to make `compare` output Ordering to begin with, but
// as I mentioned, limited time.

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let pairs = parse_pairs(&input);

    let result = pairs
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (l, r))| {
            acc + if (compare(l, r).unwrap()) { i + 1 }  else { 0 }
        });

    format!("{result}")
}

fn process_part_two(input: Vec<String>) -> String {
    let mut packets: Vec<JsonValue> = input
        .iter()
        .filter(|l| ! l.trim().is_empty())
        .map(|l| json::parse(&l).unwrap())
        .collect();

    // Add divider packets
    packets.push(json::parse("[[2]]").unwrap());
    packets.push(json::parse("[[6]]").unwrap());

    packets.sort_by(compare_ordering);

    let first_pos = packets.iter().position(|p| {
        p.len() == 1 && p[0].len() == 1 && p[0][0] == 2
    }).unwrap() + 1;


    let second_pos = packets.iter().position(|p| {
        p.len() == 1 && p[0].len() == 1 && p[0][0] == 6
    }).unwrap() + 1;

    format!("{}", first_pos * second_pos)
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

fn parse_pairs(input: &Vec<String>) -> Vec<(JsonValue, JsonValue)> {
    let mut item1: &str = "";
    let mut expect = 1;

    let mut pairs: Vec<(JsonValue, JsonValue)> = vec![];

    input.iter().for_each(|l| {
        match expect {
            1 => { item1 = l }

            2 => { 
                pairs.push((
                    json::parse(item1).unwrap(),
                    json::parse(l).unwrap()
                ));
            }

            _ => { expect = 0 }
        }

        expect += 1;
    });

    pairs
}

fn compare(left: &JsonValue, right: &JsonValue) -> Option<bool> {
    match (left, right) {
        (JsonValue::Number(left_num), JsonValue::Number(right_num)) => {
            let (l, r): (f32, f32) = ((*left_num).into(), (*right_num).into());

            if l < r {
                Some(true)
            } else if l > r {
                Some(false)
            } else {
                None
            }
        },

        (JsonValue::Number(_), JsonValue::Array(_)) => {
            compare(&JsonValue::Array(vec![left.clone()]), right)
        },

        (JsonValue::Array(_), JsonValue::Number(_)) => {
            compare(left, &JsonValue::Array(vec![right.clone()]))
        },

        (JsonValue::Array(left_arr), JsonValue::Array(right_arr)) => {
            if left_arr.len() == 0 && right_arr.len() == 0 {
                None
            } else if left_arr.len() == 0 {
                Some(true)
            } else if right_arr.len() == 0 {
                Some(false)
            } else {
                left_arr.iter().zip(right_arr.iter())
                    .map(|(l, r)| compare(l, r))
                    .find(|c| c.is_some())
                    .map(|c| c.unwrap())
                    .or_else(|| {
                        if left_arr.len() < right_arr.len() {
                            Some(true)
                        } else if left_arr.len() > right_arr.len() {
                            Some(false)
                        } else {
                            None
                        }
                    })
            }
        }

        _ => panic!("Encountered unsupported combination")
    }
}

fn compare_ordering(left: &JsonValue, right: &JsonValue) -> Ordering {
    match compare(left, right) {
        Some(true) => Ordering::Less,
        None => Ordering::Equal,
        Some(false) => Ordering::Greater,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_item_parse() {
        assert_eq!(json::parse("[]").unwrap().len(), 0);

        let number_in_list = json::parse("[2]").unwrap();
        assert_eq!(number_in_list.len(), 1);
        assert_eq!(number_in_list[0], 2);

        let number_and_array = json::parse("[3,[4,5]]").unwrap();
        assert_eq!(number_and_array.len(), 2);
        assert_eq!(number_and_array[0], 3);
        assert_eq!(number_and_array[1][0], 4);
        assert_eq!(number_and_array[1][1], 5);

        assert!(matches!(number_and_array[1], JsonValue::Array(_)));
    }

    #[test]
    fn test_compares_correctly() {
        assert_eq!(
            compare(
                &json::parse("[1,1,3,1,1]").unwrap(),
                &json::parse("[1,1,5,1,1]").unwrap(),
            ),
            Some(true),
        );

        assert_eq!(
            compare(
                &json::parse("[[1],[2,3,4]]").unwrap(),
                &json::parse("[[1],4]").unwrap(),
            ),
            Some(true),
        );

        assert_eq!(
            compare(
                &json::parse("[9]").unwrap(),
                &json::parse("[[8,7,6]]").unwrap(),
            ),
            Some(false),
        );

        assert_eq!(
            compare(
                &json::parse("[[4,4],4,4]").unwrap(),
                &json::parse("[[4,4],4,4,4]").unwrap(),
            ),
            Some(true),
        );

        assert_eq!(
            compare(
                &json::parse("[7,7,7,7]").unwrap(),
                &json::parse("[7,7,7]").unwrap(),
            ),
            Some(false),
        );

        assert_eq!(
            compare(
                &json::parse("[]").unwrap(),
                &json::parse("[3]").unwrap(),
            ),
            Some(true),
        );

        assert_eq!(
            compare(
                &json::parse("[[[]]]").unwrap(),
                &json::parse("[[]]").unwrap(),
            ),
            Some(false),
        );

        assert_eq!(
            compare(
                &json::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap(),
                &json::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap(),
            ),
            Some(false),
        );
    }
}
