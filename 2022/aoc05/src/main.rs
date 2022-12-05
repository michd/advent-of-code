use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process(input);
    println!("{output}");
}

fn process(input: Vec<String>) -> String {
    let stacks = parse_stacks(&input);
    let instructions = parse_instructions(&input);

    let mut machine = StackMachine::new(stacks);

    for inst in instructions {
        machine.execute_move_9001(&inst);
    }

    let top_crates = machine.get_top_crates().iter().map(|c| c.to_string()).collect::<Vec<String>>().join("");


    format!("{top_crates}")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

fn get_stack_count(raw_stacks: &Vec<String>) -> usize {
    raw_stacks.last().unwrap().chars().filter(|c| *c != ' ').count()
}

fn get_max_stack_height(raw_stacks: &Vec<String>) -> usize {
    raw_stacks.len() - 1
}

fn get_items_at_level(level: &String, stack_count: usize) -> Vec<Option<char>> {
    (1..(stack_count * 4 - 1)).step_by(4).map(|i| {
        let c = level.chars().nth(i);

        match c {
            None => None,
            Some(' ') => None,
            _ => c,
        }
    }).collect()
}

fn parse_stacks(input: &Vec<String>) -> Vec<Vec<char>> {
    let relevant_input: Vec<String> = input
        .iter()
        .take_while(|l| !l.trim().is_empty())
        .map(|s| s.to_owned())
        .collect();

    let stack_count = get_stack_count(&relevant_input);

    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count);

    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    for level in relevant_input[..relevant_input.len() - 1].iter().rev() {
        let items = get_items_at_level(level, stack_count);
        for (i, item) in items.iter().enumerate() {
            if let Some(c) = item {
                stacks[i].push(*c);
            }
        }
    }

    stacks
}

fn parse_instructions(input: &Vec<String>) -> Vec<MoveInstruction> {
    input
        .iter()
        .skip_while(|l| !l.trim().is_empty())
        .skip_while(|l| l.trim().is_empty())
        .map(|l| MoveInstruction::new(l))
        .collect()
}

#[derive(Debug, PartialEq)]
struct MoveInstruction {
    amount: usize,
    origin: usize,
    target: usize,
}

impl MoveInstruction {
    fn new(input: &str) -> Self {
        let split: Vec<&str> = input.split(' ').collect();
        let amount = usize::from_str_radix(split.get(1).unwrap(), 10).unwrap();
        let origin = usize::from_str_radix(split.get(3).unwrap(), 10).unwrap();
        let target = usize::from_str_radix(split.get(5).unwrap(), 10).unwrap();

        Self { amount, origin, target }
    }
}

struct StackMachine {
    stacks: Vec<Vec<char>>,
}

impl StackMachine {
    fn new(stacks: Vec<Vec<char>>) -> StackMachine {
        StackMachine { stacks }
    }

    fn move_one_crate(&mut self, origin: usize, target: usize) {
        let c = self.stacks[origin - 1].pop().unwrap();
        self.stacks[target - 1].push(c);
    }

    // Part one
    fn execute_move_9000(&mut self, instruction: &MoveInstruction) {
        for _ in 0..instruction.amount {
            self.move_one_crate(instruction.origin, instruction.target);
        }
    }

    // Part two
    fn execute_move_9001(&mut self, instruction: &MoveInstruction) {
        // split_off, append
        let origin_len = self.stacks[instruction.origin - 1].len();
        let split_index = origin_len - instruction.amount;

        let mut sub_stack = self.stacks[instruction.origin - 1].split_off(split_index);
        self.stacks[instruction.target - 1].append(&mut sub_stack);
    }

    fn get_top_crates(&self) -> Vec<char> {
        self.stacks.iter().map(|s| *s.last().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn counts_stacks_correctly() {
        let input = vec![
            "[D]        ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
        ];

        assert_eq!(get_stack_count(&input), 3);


        let input = vec![
            "    [G] [R]                 [P]    ".to_string(),
            "    [H] [W]     [T] [P]     [H]    ".to_string(),
            "    [F] [T] [P] [B] [D]     [N]    ".to_string(),
            "[L] [T] [M] [Q] [L] [C]     [Z]    ".to_string(),
            "[G] [L] [F] [D] [M] [V] [T] [J] [H]".to_string(),
            " 1   2   3   4   5   6   7   8   9 ".to_string(),
        ];

        assert_eq!(get_stack_count(&input), 9);
    }

    #[test]
    fn get_items_at_level_works() {
        let input = format!("[N] [C]    ");
        assert_eq!(get_items_at_level(&input, 3), vec![Some('N'), Some('C'), None]);

        let input = format!("    [C]    ");
        assert_eq!(get_items_at_level(&input, 4), vec![None, Some('C'), None, None]);
    }

    #[test]
    fn parse_stacks_works_correctly() {
        let input = vec![
            "[D]        ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
        ];

        assert_eq!(
            parse_stacks(&input),
            vec![
                vec!['Z', 'N', 'D'],
                vec!['M', 'C'],
                vec!['P'],
            ],
        );
    }

    #[test]
    fn build_move_instruction() {
        assert_eq!(
            MoveInstruction::new("move 1 from 2 to 1"),
            MoveInstruction { amount: 1, origin: 2, target: 1 },
        );
        assert_eq!(
            MoveInstruction::new("move 3 from 21 to 7"),
            MoveInstruction { amount: 3, origin: 21, target: 7 },
        );
    }

    #[test]
    fn stack_machine_moves_crates_9000() {
        let stacks = vec![
            vec!['Z', 'N', 'D'],
            vec!['M', 'C'],
            vec!['P'],
        ];

        let mut machine = StackMachine::new(stacks);

        let instruction = MoveInstruction { amount: 2, origin: 1, target: 3 };

        machine.execute_move_9000(&instruction);

        assert_eq!(machine.get_top_crates(), vec!['Z', 'C', 'N']);
    }

    #[test]
    fn stack_machine_moves_crates_9001() {
        let stacks = vec![
            vec!['Z', 'N', 'D'],
            vec!['M', 'C'],
            vec!['P'],
        ];

        let mut machine = StackMachine::new(stacks);

        let instruction = MoveInstruction { amount: 2, origin: 1, target: 3 };

        machine.execute_move_9001(&instruction);

        assert_eq!(machine.get_top_crates(), vec!['Z', 'C', 'D']);
    }
}
