use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let mut troop = Troop::parse(&input).unwrap();

    (0..20).for_each(|_| troop.process_round(3));

    format!("{}", troop.monkey_business())
}

fn process_part_two(input: Vec<String>) -> String {
    let mut troop = Troop::parse(&input).unwrap();

    (0..10000).for_each(|_| troop.process_round(1));

    format!("{}", troop.monkey_business())
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

fn parse_start_items(input: &str) -> Option<Vec<usize>> {
    let split = input.split(": ").last()?.split(", ");
    Some(split.map(|i| usize::from_str_radix(i, 10).unwrap()).collect())
}

fn parse_number_at_end(input: &str) -> Option<usize> {
    usize::from_str_radix(input.split(" ").last()?, 10).ok()
}

fn parse_divisor(input: &str) -> Option<usize> {
    parse_number_at_end(input).map(|i| i as usize)
}

fn parse_targets(input1: &str, input2: &str) -> Option<(usize, usize)> {
    let target1 = parse_number_at_end(input1)? as usize;
    let target2 = parse_number_at_end(input2)? as usize;
    Some((target1, target2))
}

#[derive(Debug, PartialEq)]
enum Operand {
    Old,
    Integer(usize)
}

// Only one operand is given because operand 1 is always Operand::Old
#[derive(Debug, PartialEq)]
enum Operation {
    Multiply(Operand),
    Add(Operand),
}

impl Operation {
    fn parse(input: &str) -> Option<Operation> {
        let split = input.split(" ");
        let split_len = split.count();
        let operand_raw = input.split(" ").last()?;
        let operation_raw = input.split(" ").nth(split_len - 2)?;

        let operand = match operand_raw {
            "old" => Operand::Old,
            _ => Operand::Integer(usize::from_str_radix(operand_raw, 10).ok()?),
        };

        match operation_raw {
            "*" => Some(Operation::Multiply(operand)),
            "+" => Some(Operation::Add(operand)),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq)]
struct Monkey {
    // Items at their worry level monkey carries
    // Please pretend with me that this `usize` here and everywhere these
    // values came through weren't a `u128` for some time.
    // Turns out part 2 was designed so you couldn't get away with "let's just throw a larger data
    // type at it". (: 
    items: Vec<usize>,

    // Operation that happens to worry level as monkey inspects item
    worry_operation: Operation,

    // We check if the worry level after worry_operation, _and then divided by 3_ (rounded down)
    // is divisable by this number. If it is, the item, at that worry level, is added to the
    // monkey at target 0. If it isn't, it goes to monkey at target 1.
    divisor: usize,

    // Two possible monkeys an item transfers to
    targets: (usize, usize),

    // Number of times this monkey has inspected any item
    inspection_count: usize
}

impl Monkey {
    fn new(
        starting_items: Vec<usize>,
        worry_operation: Operation,
        divisor: usize,
        targets: (usize, usize)
    ) -> Monkey {
        Monkey {
            items: starting_items,
            worry_operation,
            divisor,
            targets,
            inspection_count: 0
        }
    }

    fn parse(input: &[String]) -> Option<Monkey> {
        if input.len() < 6 {
            return None;
        }

        let starting_items_input = input.iter().nth(1).unwrap();
        let operation_input = input.iter().nth(2).unwrap();
        let divisor_input = input.iter().nth(3).unwrap();
        let target1_input = input.iter().nth(4).unwrap();
        let target2_input = input.iter().nth(5).unwrap();

        let starting_items = parse_start_items(starting_items_input)?;
        let worry_operation = Operation::parse(operation_input)?;
        let divisor = parse_divisor(divisor_input)?;
        let targets = parse_targets(target1_input, target2_input)?;

        Some(Monkey::new(starting_items, worry_operation, divisor, targets))
    }

    fn inspect(&mut self, item: usize, worry_limit: usize) -> usize {
        self.inspection_count += 1;

        let result = match &self.worry_operation {
            Operation::Multiply(operand) => {
                item * match operand {
                    Operand::Old => item,
                    Operand::Integer(i) => *i,
                }
            }

            Operation::Add(operand) => item + match operand {
                Operand::Old => item,
                Operand::Integer(i) => *i,
            }
        };

        result % worry_limit
    }

    fn decide_throw_target(&self, worry_level: usize) -> usize {
        let (m1, m2) = self.targets;
        if worry_level % (self.divisor as usize) == 0 { m1 } else { m2 }
    }

    /// Considers the items held, inspects them, then throws them.
    /// Mutates self.items, self.inspection_count.
    /// Returns vector of tuples, where in each tuple:
    ///  - 0 = target monkey
    ///  - 1 = item's worry level thrown at that monkey
    fn process_round(&mut self, worry_divisor: usize, worry_limit: usize) -> Vec<(usize, usize)> {
        let items = self.items.clone();
        let inspected_items: Vec<usize> = items
            .iter()
            .map(|i| self.inspect(*i, worry_limit))
            .map(|i| i / worry_divisor) // worry level decreases after inspection - or does it?
            .collect();

        let throw_data: Vec<(usize, usize)> = inspected_items
            .iter()
            .map(|i| (self.decide_throw_target(*i), *i))
            .collect();

        self.items.clear();

        throw_data
    }
}

#[derive(Debug)]
struct Troop {
    monkeys: Vec<Monkey>,
    worry_limit: usize,
}

impl Troop {
    fn parse(input: &Vec<String>) -> Option<Troop> {
        let mut monkeys: Vec<Monkey> = vec![];

        // Wow I'm sure this is a disgusting way to read the input, but for
        // some reason this one was giving me a lot of trouble.
        (0..input.len()).step_by(7).for_each(|offset| {
            let monkey_lines: Vec<String> = (0..6)
                .map(|i| input[offset + i].to_string())
                .collect();

            monkeys.push(Monkey::parse(&monkey_lines).unwrap());
        });

        // Worry limit is the maximum worry value for any one item.
        // As soon as worry exceeds a value _all_ the monkeys can divide by, being
        // larger does not make a functional difference. Thus, we use this worry limit
        // in every inspection to make sure our number sizes stay down, keeping our
        // worry levels manageable.
        // I didn't bother with a "lowest common multiple" here and naively just mulitiplied all
        // the divisors together. As it turns out, that fits in a `usize` just fine on my machine.
        let worry_limit = monkeys.iter().fold(1, |acc, m| acc * m.divisor as usize);

        if monkeys.len() == 0 {
            None
        } else {
            Some(Troop { monkeys, worry_limit })
        }
    }

    fn process_round(&mut self, worry_divisor: usize) {
        (0..self.monkeys.len()).for_each(|monkey_index| {
            let monkey = &mut self.monkeys[monkey_index];

            let throw_data = monkey.process_round(worry_divisor, self.worry_limit);

            for (target_monkey_index, item) in throw_data {
                self.monkeys[target_monkey_index].items.push(item);
            }
        });
    }

    fn monkey_business(&self) -> usize {
        let mut inspections: Vec<usize> = self.monkeys
            .iter()
            .map(|m| m.inspection_count)
            .collect();

        inspections.sort();
        inspections.reverse();

        inspections[0] * inspections[1]
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parsing_operation() {
        assert_eq!(
            Operation::parse("  Operation: new = old * 19").unwrap(),
            Operation::Multiply(Operand::Integer(19)),
        );
        assert_eq!(
            Operation::parse("  Operation: new = old + 6").unwrap(),
            Operation::Add(Operand::Integer(6)),
        );
        assert_eq!(
            Operation::parse("  Operation: new = old * old").unwrap(),
            Operation::Multiply(Operand::Old),
        );
    }

    #[test]
    fn test_parse_start_items() {
        assert_eq!(
            parse_start_items("  Starting items: 79, 98").unwrap(),
            vec![79, 98],
        );
        assert_eq!(
            parse_start_items("  Starting items: 54, 65, 75, 74").unwrap(),
            vec![54, 65, 75, 74],
        );
        assert_eq!(
            parse_start_items("  Starting items: 79, 60, 97").unwrap(),
            vec![79, 60, 97],
        );
        assert_eq!(
            parse_start_items("  Starting items: 74").unwrap(),
            vec![74],
        );
    }

    #[test]
    fn test_parse_divisor() {
        assert_eq!(
            parse_divisor("  Test: divisible by 23").unwrap(),
            23,
        );
    }

    #[test]
    fn test_parse_targets() {
        assert_eq!(
            parse_targets(
                "    If true: throw to monkey 2",
                "    If false: throw to monkey 3",
            ).unwrap(),
            (2, 3),
        );
    }

    #[test]
    fn test_parse_monkey() {
        let input = vec![
            "Monkey 0:".to_string(),
            "  Starting items: 79, 98".to_string(),
            "  Operation: new = old * 19".to_string(),
            "  Test: divisible by 23".to_string(),
            "    If true: throw to monkey 2".to_string(),
            "    If false: throw to monkey 3".to_string(),
        ];

        assert_eq!(
            Monkey::parse(&input).unwrap(),
            Monkey {
                items: vec![79, 98],
                worry_operation: Operation::Multiply(Operand::Integer(19)),
                divisor: 23,
                targets: (2, 3),
                inspection_count: 0
            },
        );
    }

    #[test]
    fn test_monkey_inspect() {
        let mut monkey = Monkey {
            items: vec![79, 98],
            worry_operation: Operation::Multiply(Operand::Integer(19)),
            divisor: 23,
            targets: (2, 3),
            inspection_count: 0
        };

        // Note that the inspect function itself does not mutate Monkey.items
        // It only mutates inspection_count.

        assert_eq!(monkey.inspect(79, usize::MAX), 1501);
        assert_eq!(monkey.inspection_count, 1);

        assert_eq!(monkey.inspect(98, usize::MAX), 1862);
        assert_eq!(monkey.inspection_count, 2);
    }

    #[test]
    fn test_monkey_decide_throw_target() {
        let monkey = Monkey {
            items: vec![79, 98],
            worry_operation: Operation::Multiply(Operand::Integer(19)),
            divisor: 23,
            targets: (2, 3),
            inspection_count: 0
        };

        assert_eq!(monkey.decide_throw_target(500), 3);
        assert_eq!(monkey.decide_throw_target(620), 3);

        assert_eq!(monkey.decide_throw_target(322), 2);
    }

    #[test]
    fn test_monkey_process_round() {
        // Note: in this test, the monkeys don't interact yet.
        let mut monkey0 = Monkey {
            items: vec![79, 98],
            worry_operation: Operation::Multiply(Operand::Integer(19)),
            divisor: 23,
            targets: (2, 3),
            inspection_count: 0
        };

        let mut monkey1 = Monkey {
            items: vec![54, 65, 75, 74],
            worry_operation: Operation::Add(Operand::Integer(6)),
            divisor: 19,
            targets: (2, 0),
            inspection_count: 0
        };

        assert_eq!(
            monkey0.process_round(3, usize::MAX),
            vec![ (3, 500), (3, 620) ],
        );
        assert_eq!(monkey0.inspection_count, 2);

        assert_eq!(
            monkey1.process_round(3, usize::MAX),
            vec![ (0, 20), (0, 23), (0, 27), (0, 26) ],
        );
    }

    #[test]
    fn test_troop_process_round() {
        let mut troop = Troop {
            monkeys: vec![
                Monkey { // 0
                    items: vec![79, 98],
                    worry_operation: Operation::Multiply(Operand::Integer(19)),
                    divisor: 23,
                    targets: (2, 3),
                    inspection_count: 0
                },

                Monkey { // 1
                    items: vec![54, 65, 75, 74],
                    worry_operation: Operation::Add(Operand::Integer(6)),
                    divisor: 19,
                    targets: (2, 0),
                    inspection_count: 0
                },

                Monkey { // 2
                    items: vec![79, 60, 97],
                    worry_operation: Operation::Multiply(Operand::Old),
                    divisor: 13,
                    targets: (1, 3),
                    inspection_count: 0,
                },

                Monkey { // 3
                    items: vec![74],
                    worry_operation: Operation::Add(Operand::Integer(3)),
                    divisor: 17,
                    targets: (0, 1),
                    inspection_count: 0,
                },
            ],

            worry_limit: 23 * 19 * 13 * 17,
        };

        // Round 1
        troop.process_round(3);
        assert_eq!(troop.monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(troop.monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(troop.monkeys[2].items, vec![]);
        assert_eq!(troop.monkeys[3].items, vec![]);

        // Round 2
        troop.process_round(3);
        assert_eq!(troop.monkeys[0].items, vec![695, 10, 71, 135, 350]);
        assert_eq!(troop.monkeys[1].items, vec![43, 49, 58, 55, 362]);
        assert_eq!(troop.monkeys[2].items, vec![]);
        assert_eq!(troop.monkeys[3].items, vec![]);

        // Round 3
        troop.process_round(3);
        assert_eq!(troop.monkeys[0].items, vec![16, 18, 21, 20, 122]);
        assert_eq!(troop.monkeys[1].items, vec![1468, 22, 150, 286, 739]);
        assert_eq!(troop.monkeys[2].items, vec![]);
        assert_eq!(troop.monkeys[3].items, vec![]);

        // Round 4
        troop.process_round(3);
        assert_eq!(troop.monkeys[0].items, vec![491, 9, 52, 97, 248, 34]);
        assert_eq!(troop.monkeys[1].items, vec![39, 45, 43, 258]);
        assert_eq!(troop.monkeys[2].items, vec![]);
        assert_eq!(troop.monkeys[3].items, vec![]);

        // Round 20
        (4..20).for_each(|_| troop.process_round(3));
        assert_eq!(troop.monkeys[0].items, vec![10, 12, 14, 26, 34]);
        assert_eq!(troop.monkeys[1].items, vec![245, 93, 53, 199, 115]);
        assert_eq!(troop.monkeys[2].items, vec![]);
        assert_eq!(troop.monkeys[3].items, vec![]);

        // Inspection counts:
        assert_eq!(troop.monkeys[0].inspection_count, 101);
        assert_eq!(troop.monkeys[1].inspection_count, 95);
        assert_eq!(troop.monkeys[2].inspection_count, 7);
        assert_eq!(troop.monkeys[3].inspection_count, 105);

        // And finally, monkey business
        assert_eq!(troop.monkey_business(), 10605);
    }
}
