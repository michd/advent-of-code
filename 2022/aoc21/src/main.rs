use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{}", output);
}

fn process_part_one(input: Vec<String>) -> String {
    let troop = Troop::parse(&input);
    let root_yell = troop.get_monkey_yell("root");
    format!("{}", root_yell)
}

fn process_part_two(input: Vec<String>) -> String {
    let troop = Troop::parse(&input);
    let humn_yell = troop.get_inverse_monkey_yell("humn");
    format!("{}", humn_yell)
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operation {
    fn from_str(input: &str) -> Operation {
        match input {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            // Using else here so I don't have to make this method return Option<Calculation>
            // and deal with another unwrap
            _ => Operation::Divide,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Calculation<'a> {
    op1: &'a str,
    op2: &'a str,
    operation: Operation
}

impl Calculation<'_> {
    fn get_other_operand(&self, not_this: &str) -> &str {
        if self.op1 == not_this {
            self.op2
        } else {
            self.op1
        }
    }
}

#[derive(Debug, PartialEq)]
enum Yell<'a> {
    Number(isize),
    Calc(Calculation<'a>),
}

impl Yell<'_> {
    fn parse(input: &str) -> Yell {
        let split = input.split(" ").collect::<Vec<&str>>();

        if split.len() == 1 {
            return Yell::Number(isize::from_str_radix(input, 10).unwrap())
        }

        let op1 = split[0];
        let op2 = split[2];

        Yell::Calc(
            Calculation { op1, op2, operation: Operation::from_str(split[1]) },
        )
    }
}

#[derive(Debug, PartialEq)]
struct Monkey<'a> {
    name: &'a str,
    yells: Yell<'a>
}

impl Monkey<'_> {
    fn parse(input: &str) -> Monkey {
        let split = input.split(": ").collect::<Vec<&str>>();

        Monkey {
            name: split[0],
            yells: Yell::parse(split[1])
        }
    }
}

#[derive(Debug, PartialEq)]
struct Troop<'a> {
    monkeys: HashMap<&'a str, Monkey<'a>>,
    // Maps to monkeys that use a given hash in an equation
    parents: HashMap<&'a str, &'a str>,
}

impl Troop<'_> {
    fn parse(input: &Vec<String>) -> Troop {
        // Store monkeys in a hashmap by name so they can be looked up quickly.
        let monkeys = input
                .iter()
                .map(|l| Monkey::parse(l))
                .map(|m| (m.name, m))
                .collect::<HashMap<&str, Monkey>>();

        // Each monkey has at most one monkey that uses this one in an operation.
        // In part 2 we need to work back up the tree so we need to be able to find a "parent"
        // efficiently. So as part of parsing we find the parents where applicable and store them
        // in a separate hash map.
        let parents = monkeys
            .keys() // for each monkey name
            .filter_map(|m| { // Gather a parent, if any
                monkeys
                    .iter()
                    .find(|(_, y)| {
                        // Find an equation where either one of the
                        // of the operands matches the current monkey (m)
                        // Monkeys that just yell a fixed number have no operands, so they
                        // will enver match as a parent of another
                        match y.yells {
                            Yell::Number(_) => false,

                            Yell::Calc(Calculation { op1, op2, operation: _ }) => {
                                &op1 == m || &op2 == m
                            }
                        }
                    })
                    .map(|(parent, _)| (*m, *parent))
            })
            .collect();

        Troop {
            monkeys,
            parents,
        }
    }

    // Part 1 (and used in 2): work down a tree of operations until we have concrete values
    // to operate on, return result of operation
    fn get_monkey_yell(&self, monkey_name: &str) -> isize {
        let monkey =  &self.monkeys[monkey_name];

        match monkey.yells {
            Yell::Number(num) => num,
            Yell::Calc(Calculation { op1, op2, operation }) => {
                let op1_n = self.get_monkey_yell(&op1);
                let op2_n = self.get_monkey_yell(&op2);

                match operation {
                    Operation::Add => op1_n + op2_n,
                    Operation::Subtract => op1_n - op2_n,
                    Operation::Multiply => op1_n * op2_n,
                    Operation::Divide => op1_n / op2_n,
                }
            }
        }
    }

    // Part 2, work backward to solve this monkey's value for its parent's known value
    // This works back to root to find its value (since there the value is the other operand's value
    // which is determined the usual way, working down the tree).
    // Once we've retrieved the parent monkey's value, we can use the value of the other operand of
    // current monkey's operation along with the operation to solve for X.
    fn get_inverse_monkey_yell(&self, monkey_name: &str) -> isize {
        let parent_monkey_name = self.parents[monkey_name];
        let parent_monkey = &self.monkeys[parent_monkey_name];

        if parent_monkey_name == "root" {
            return if let Yell::Calc(c) = parent_monkey.yells {
                let other = &self.monkeys[c.get_other_operand(monkey_name)];
                let root_yells = self.get_monkey_yell(other.name);
                root_yells
            } else {
                // Won't happen
                0
            }
        }

        if let Yell::Calc(c) = parent_monkey.yells {
            // Get other operand of the calculation
            let other = &self.monkeys[c.get_other_operand(monkey_name)];

            // name of monkey we're trying to solve for
            let x_name = monkey_name;

            // Name of operand 1 in the operation (to determine position of X in operation,
            // which is relevant for solving for X in subtractions and divisions
            let op1_name = c.op1;

            // Parent
            let b = self.get_inverse_monkey_yell(parent_monkey_name);

            // other operand, solve by going down the tree as in part 1
            let a = self.get_monkey_yell(other.name);

            // Solve for x
            match c.operation {
                // b = x + a
                // b = a + x
                Operation::Add => b - a,

                // b = x * a
                // b = a * x
                Operation::Multiply => b / a,

                Operation::Subtract => {
                    if x_name == op1_name {
                        // b = x - a
                        b + a
                    } else {
                        // b = a - x
                        a - b
                    }
                }

                Operation::Divide => {
                    if x_name == op1_name {
                        // b = x / a
                        b * a
                    } else {
                        // b = a / x
                        a / b
                    }
                }
            }
        } else {
            // Won't happen
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_monkey_parse() {
        assert_eq!(
            Monkey::parse("dbpl: 5"),
            Monkey {
                name: "dbpl",
                yells: Yell::Number(5),
            },
        );

        assert_eq!(
            Monkey::parse("root: pppw + sjmn"),
            Monkey {
                name: "root",
                yells: Yell::Calc(
                    Calculation {
                        op1: "pppw",
                        op2: "sjmn",
                        operation: Operation::Add
                    }
                ),
            }
        );

        assert_eq!(
            Monkey::parse("root: pppw - sjmn"),
            Monkey {
                name: "root",
                yells: Yell::Calc(
                    Calculation {
                        op1: "pppw",
                        op2: "sjmn",
                        operation: Operation::Subtract
                    }
                ),
            }
        );
        assert_eq!(
            Monkey::parse("root: pppw * sjmn"),
            Monkey {
                name: "root",
                yells: Yell::Calc(
                    Calculation {
                        op1: "pppw",
                        op2: "sjmn",
                        operation: Operation::Multiply
                    }
                ),
            }
        );
        assert_eq!(
            Monkey::parse("root: pppw / sjmn"),
            Monkey {
                name: "root",
                yells: Yell::Calc(
                    Calculation {
                        op1: "pppw",
                        op2: "sjmn",
                        operation: Operation::Divide
                    }
                ),
            }
        );
    }

    #[test]
    fn test_get_monkey_yell() {
        let monkeys_input = vec![
            "root: pppw + sjmn".to_string(),
            "dbpl: 5".to_string(),
            "cczh: sllz + lgvd".to_string(),
            "zczc: 2".to_string(),
            "ptdq: humn - dvpt".to_string(),
            "dvpt: 3".to_string(),
            "lfqf: 4".to_string(),
            "humn: 5".to_string(),
            "ljgn: 2".to_string(),
            "sjmn: drzm * dbpl".to_string(),
            "sllz: 4".to_string(),
            "pppw: cczh / lfqf".to_string(),
            "lgvd: ljgn * ptdq".to_string(),
            "drzm: hmdt - zczc".to_string(),
            "hmdt: 32".to_string(),
        ];

        let troop = Troop::parse(&monkeys_input);

        assert_eq!(troop.get_monkey_yell("root"), 152);
    }

    #[test]
    fn test_get_inverse_monkey_yell() {
        let monkeys_input = vec![
            "root: pppw + sjmn".to_string(),
            "dbpl: 5".to_string(),
            "cczh: sllz + lgvd".to_string(),
            "zczc: 2".to_string(),
            "ptdq: humn - dvpt".to_string(),
            "dvpt: 3".to_string(),
            "lfqf: 4".to_string(),
            "humn: 5".to_string(),
            "ljgn: 2".to_string(),
            "sjmn: drzm * dbpl".to_string(),
            "sllz: 4".to_string(),
            "pppw: cczh / lfqf".to_string(),
            "lgvd: ljgn * ptdq".to_string(),
            "drzm: hmdt - zczc".to_string(),
            "hmdt: 32".to_string(),
        ];

        let troop = Troop::parse(&monkeys_input);

        assert_eq!(troop.get_inverse_monkey_yell("humn"), 301);
    }
}
