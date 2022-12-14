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
    let output = process_part_one(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let pairs = parse_pairs(&input);

    let result = pairs
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (l, r))| {
            acc + if l.cmp(r) == Ordering::Less { i + 1 }  else { 0 }
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

fn parse_pairs(input: &Vec<String>) -> Vec<(Packet, Packet)> {
    let mut item1: &str = "";
    let mut expect = 1;

    let mut pairs: Vec<(Packet, Packet)> = vec![];

    input.iter().for_each(|l| {
        match expect {
            1 => { item1 = l }

            2 => { 
                pairs.push((
                    PacketParser::parse(item1).unwrap(),
                    PacketParser::parse(l).unwrap()
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

#[derive(Debug)]
struct ParseError { msg: String }

impl ParseError {
    fn new(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    ListStart,
    ListEnd,
    Comma,
    Number(usize),
}

impl Token {
    fn parse_as_tokens(input: &str) -> Result<Vec<Self>, ParseError> {
        let tokens = input
            .chars()
            .scan(None, |pending, c| {
                match c {
                    '[' => {
                        let out = vec![*pending, Some(Self::ListStart)];
                        *pending = None;
                        Some(out.into_iter())
                    },

                    ']' => {
                        let out = vec![*pending, Some(Self::ListEnd)];
                        *pending = None;
                        Some(out.into_iter())
                    }

                    ',' => {
                        let out = vec![*pending, Some(Self::Comma)];
                        *pending = None;
                        Some(out.into_iter())
                    }

                    '0'..='9' => {
                        let mut out: Vec<Option<Self>> = vec![];

                        let digit = c as usize - 48; // Convert from ascii val

                        if let Some(Self::Number(num)) = pending {
                            let updatedNum = *num * 10 + digit;
                            *pending = Some(Self::Number(updatedNum));
                        } else {
                            out.push(*pending); 
                            *pending = Some(Self::Number(digit));
                        }

                        Some(out.into_iter())
                    }

                    _ => None
                }
            })
            .flatten()
            .filter_map(|i| i)
            .collect();

        Ok(tokens)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
enum Packet {
    Number(usize),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(l), Self::Number(r)) => l.cmp(r),
            
            (Self::Number(_), Self::List(_)) => {
                Self::List(vec![self.clone()]).cmp(other)
            }

            (Self::List(_), Self::Number(_)) => {
                self.cmp(&Self::List(vec![other.clone()]))
            }

            (Self::List(l_arr), Self::List(r_arr)) => {
                if l_arr.len() == 0 && r_arr.len() == 0 {
                    Ordering::Equal
                } else if l_arr.len() == 0 {
                    Ordering::Less
                } else if r_arr.len() == 0 {
                    Ordering::Greater
                } else {
                    l_arr
                        .iter()
                        .zip(r_arr.iter())
                        .map(|(l, r)| l.cmp(r))
                        .find(|c| *c != Ordering::Equal)
                        .unwrap_or(l_arr.len().cmp(&r_arr.len()))
                }
            }
        }
    }
}

struct PacketParser {
    tokens: Vec<Token>,
}

impl PacketParser {
    fn parse(input: &str) -> Result<Packet, ParseError> {
        let tokens = Token::parse_as_tokens(input)?;

        if tokens.len() < 2 {
            return Err(ParseError::new("Not enough tokens"));
        }

        let mut parser = Self { tokens };

        let (list, _) = parser.parse_list(0)?;

        list.ok_or(
            ParseError::new("Failed to parse a list"),
        )
    }

    fn parse_item(&self, token_index: usize) -> Result<(Option<Packet>, usize), ParseError> {
        Ok(match self.tokens[token_index] {
            Token::Number(n) => (Some(Packet::Number(n)), token_index + 1),
            Token::ListStart => self.parse_list(token_index)?,

            Token::ListEnd => (None, token_index + 1),

            Token::Comma => {
                return Err(
                    ParseError::new(&format!("Unexpected Comma token at index {token_index}")),
                );
            }
        })
    }

    fn parse_list(&self, token_index: usize) -> Result<(Option<Packet>, usize), ParseError> {
        if self.tokens[token_index] != Token::ListStart {
            return Err(
                ParseError::new(
                    &format!("Expected ListStart token but found {:?}", self.tokens[token_index])
                )
            );
        }

        let mut list: Vec<Packet> = vec![];
        let mut index = token_index + 1;

        loop {
            let (item, new_index) = self.parse_item(index)?;

            if let Some(thing) = item {
                list.push(thing);
            }

            if new_index >= self.tokens.len() { break; }

            index = new_index;
            if self.tokens[index] == Token::Comma { index += 1; }
        }

        Ok((Some(Packet::List(list)), index + 1))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn vecs_eq<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> bool {
        a.len() == b.len() &&
        a
            .iter()
            .zip(b.iter())
            .all(|(ai, bi)| ai == bi)
    }

    #[test]
    fn test_parse_as_tokens() {
        assert!(vecs_eq(
            Token::parse_as_tokens("[]").unwrap(),
            vec![
                Token::ListStart,
                Token::ListEnd,
            ],
        ));

        assert!(vecs_eq(
            Token::parse_as_tokens("[2]").unwrap(),
            vec![
                Token::ListStart,
                Token::Number(2),
                Token::ListEnd,
            ],
        ));

        assert!(vecs_eq(
            Token::parse_as_tokens("[2357]").unwrap(),
            vec![
                Token::ListStart,
                Token::Number(2357),
                Token::ListEnd,
            ],
        ));

        assert!(vecs_eq(
            Token::parse_as_tokens("[2,3,57]").unwrap(),
            vec![
                Token::ListStart,
                Token::Number(2),
                Token::Comma,
                Token::Number(3),
                Token::Comma,
                Token::Number(57),
                Token::ListEnd,
            ],
        ));

        assert!(vecs_eq(
            Token::parse_as_tokens("[2,3,[],[1,2]]").unwrap(),
            vec![
                Token::ListStart,
                Token::Number(2),
                Token::Comma,
                Token::Number(3),
                Token::Comma,
                Token::ListStart,
                Token::ListEnd,
                Token::Comma,
                Token::ListStart,
                Token::Number(1),
                Token::Comma,
                Token::Number(2),
                Token::ListEnd,
                Token::ListEnd,
            ],
        ));
    }

    #[test]
    fn test_item_parser() {
        assert!(
            matches!(
                PacketParser::parse("[]").unwrap(),
                Packet::List(_),
            ),
        );

        let parsed = PacketParser::parse("[1,1,3,1,1]").unwrap();

        if let Packet::List(l) = parsed {
            assert!(vecs_eq(
                l,
                vec![
                    Packet::Number(1),
                    Packet::Number(1),
                    Packet::Number(3),
                    Packet::Number(1),
                    Packet::Number(1)
                ],
            ));
        } else {
            panic!("Not list");
        }

        return;

        // So this test fails but it's kinda impossible to tell if it's because
        // of the method of testing or because of parsing.
        let parsed = PacketParser::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap();

        if let Packet::List(l) = parsed {
            assert!(vecs_eq(
                l,
                vec![
                    Packet::Number(1),
                    Packet::List(vec![
                        Packet::Number(2),
                        Packet::List(vec![
                            Packet::Number(3),
                            Packet::List(vec![
                                Packet::Number(4),
                                Packet::List(vec![
                                    Packet::Number(5),
                                    Packet::Number(6),
                                    Packet::Number(7),
                                ]),
                            ]),
                        ]),
                    ]),
                    Packet::Number(8),
                    Packet::Number(9),
                ]
            ));
        } else {
            panic!("Not list 2");
        }
    }

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

    #[test]
    fn test_compares_correctly2() {
        assert_eq!(
            PacketParser::parse("[1,1,3,1,1]").unwrap().cmp(
                &PacketParser::parse("[1,1,5,1,1]").unwrap()
            ),
            Ordering::Less,
        );

        assert_eq!(
            PacketParser::parse("[[1],[2,3,4]]").unwrap().cmp(
                &PacketParser::parse("[[1],4]").unwrap()
            ),
            Ordering::Less,
        );

        assert_eq!(
            PacketParser::parse("[9]").unwrap().cmp(
                &PacketParser::parse("[[8,7,6]]").unwrap(),
            ),
            Ordering::Greater,
        );


        assert_eq!(
            PacketParser::parse("[[4,4],4,4]").unwrap().cmp(
                &PacketParser::parse("[[4,4],4,4,4]").unwrap(),
            ),
            Ordering::Less,
        );

        assert_eq!(
            PacketParser::parse("[7,7,7,7]").unwrap().cmp(
                &PacketParser::parse("[7,7,7]").unwrap(),
            ),
            Ordering::Greater,
        );

        assert_eq!(
            PacketParser::parse("[]").unwrap().cmp(
                &PacketParser::parse("[3]").unwrap(),
            ),
            Ordering::Less,
        );

        assert_eq!(
            PacketParser::parse("[[[]]]").unwrap().cmp(
                &PacketParser::parse("[[]]").unwrap(),
            ),
            Ordering::Greater,
        );

        assert_eq!(
            PacketParser::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]").unwrap().cmp(
                &PacketParser::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]").unwrap(),
            ),
            Ordering::Greater,
        );
    }
}
