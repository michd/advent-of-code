use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    //process_part_one(input);
    process_part_two(input);
}

fn process_part_one(input: Vec<String>) {
    let ops = instructions_to_ops_queue(
        input.iter().map(|l| Instruction::parse(l).unwrap()).collect()
    );

    let mut cpu = Cpu::new(ops);

    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut sum_intensity: isize = 0;

    loop {
        let cycle_res = cpu.cycle();

        if cycle_res.is_none() {
            break;
        }

        let (cycle, x, _) = cycle_res.unwrap();

        if interesting_cycles.iter().any(|c| *c == cycle) {
            sum_intensity += cycle as isize * x;
        }
    }

    println!("{sum_intensity}");
}

fn process_part_two(input: Vec<String>) {
    let ops = instructions_to_ops_queue(
        input.iter().map(|l| Instruction::parse(l).unwrap()).collect()
    );

    let mut cpu = Cpu::new(ops);

    println!("Output:");
    loop {
        let cycle_res = cpu.cycle();

        if cycle_res.is_none() {
            break;
        }

        let (cycle, _, pixel) = cycle_res.unwrap();

        print!(
            "{}{}",
            if pixel { "#" } else { "." }, // Draw pixel if needed
            if cycle % 40 == 0 { "\n" } else { "" }, // Wrap back to new line
        );
    }

    println!("--- end output ---");
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

/// Takes all operations contains in instructions and makes a flat operations
/// vec, which is ordered for convenience as a queue in Cpu
fn instructions_to_ops_queue(instructions: Vec<Instruction>) -> Vec<Operation> {
    let mut all_ops: Vec<Operation> = vec![];
    for i in instructions.iter().rev() {
        let ops: Vec<Operation> = i.operations.iter().rev().map(|o| *o).collect();
        all_ops.extend(ops);
    }

    all_ops
}

// Operations take single cycle
#[derive(Debug, Copy, Clone, PartialEq)]
enum Operation {
    Noop,
    AddX(isize)
}

#[derive(Debug, Clone)]
struct Instruction {
    operations: Vec<Operation>
}

impl Instruction {
    fn parse(input: &str) -> Option<Instruction> {
        let s: Vec<&str> = input.split(" ").collect();

        match s[0] {
            "noop" => Some(Instruction { operations: vec![ Operation::Noop ] }),

            // addx parses to a two-operation instruction to make it 2 cycles
            "addx" => {
                let operand = isize::from_str_radix(s[1], 10).ok()?;
                Some(Instruction {
                    operations: vec![
                        Operation::Noop,
                        Operation::AddX(operand),
                    ]
                })
            },
            _ => None
        }
    }

    fn parse_all(input: &Vec<&str>) -> Vec<Instruction> {
        input
            .iter()
            .filter_map(|l| {
                Instruction::parse(l)
            })
            .collect()
    }
}

#[derive(Debug)]
struct Cpu {
    pc: isize,
    register_x: isize,
    operation_queue: Vec<Operation>,
}

impl Cpu {
    fn new(operation_queue: Vec<Operation>) -> Cpu {
        Cpu {
            pc: 0,
            register_x: 1,
            operation_queue,
        }
    }

    // Returns: current cycle, X value during, whether this cycle draws a pixel
    fn cycle(&mut self) -> Option<(isize, isize, bool)> {
        self.pc += 1;

        let draw_pixel = ((self.register_x - 1)..=(self.register_x + 1)).contains(&((self.pc-1) % 40));

        // Output first:
        let (out_cycle, x) = (self.pc, self.register_x);


        let op = self.operation_queue.pop()?;

        match op {
            Operation::Noop => {},
            Operation::AddX(inc) => {
                self.register_x += inc;
            },
        }

        Some((out_cycle, x, draw_pixel))
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_instruction_parse() {
        let ops = Instruction::parse("noop").unwrap().operations;
        assert_eq!(ops[0], Operation::Noop);

        let ops = Instruction::parse("addx 3").unwrap().operations;
        assert_eq!(ops[0], Operation::Noop);
        assert_eq!(ops[1], Operation::AddX(3));

        let ops = Instruction::parse("addx -5").unwrap().operations;
        assert_eq!(ops[0], Operation::Noop);
        assert_eq!(ops[1], Operation::AddX(-5));
    }

    #[test]
    fn test_instructions_to_ops_queue() {
        let instructions = vec![
            Instruction { operations: vec![ Operation::Noop ] },
            Instruction { operations: vec![ Operation::Noop, Operation::AddX(1) ] }
        ];

        let ops = instructions_to_ops_queue(instructions);

        assert_eq!(ops[0], Operation::AddX(1));
        assert_eq!(ops[1], Operation::Noop);
        assert_eq!(ops[2], Operation::Noop);
    }

    #[test]
    fn test_cpu_exec() {
        let ops = instructions_to_ops_queue(
            Instruction::parse_all(&vec![
                "noop",
                "addx 3",
                "addx -5",
                "noop",
            ])
        );

        let mut cpu = Cpu::new(ops);

        assert_eq!(cpu.pc, 0);
        assert_eq!(cpu.register_x, 1);

        assert_eq!((1, 1, true), cpu.cycle().unwrap());  // noop
        assert_eq!((2, 1, true), cpu.cycle().unwrap());  // noop (add3)
        assert_eq!((3, 1, true), cpu.cycle().unwrap());  // add3
        assert_eq!((4, 4, true), cpu.cycle().unwrap());  // noop (add-1)
        assert_eq!((5, 4, true), cpu.cycle().unwrap());  // add-1
        assert_eq!((6, -1, false), cpu.cycle().unwrap()); //noop
    }

    #[test]
    fn test_example_part_one() {
        let ops = instructions_to_ops_queue(
            Instruction::parse_all(&vec![
                "addx 15",
                "addx -11",
                "addx 6",
                "addx -3",
                "addx 5",
                "addx -1",
                "addx -8",
                "addx 13",
                "addx 4",
                "noop",
                "addx -1",
                "addx 5",
                "addx -1",
                "addx 5",
                "addx -1",
                "addx 5",
                "addx -1",
                "addx 5",
                "addx -1",
                "addx -35",
                "addx 1",
                "addx 24",
                "addx -19",
                "addx 1",
                "addx 16",
                "addx -11",
                "noop",
                "noop",
                "addx 21",
                "addx -15",
                "noop",
                "noop",
                "addx -3",
                "addx 9",
                "addx 1",
                "addx -3",
                "addx 8",
                "addx 1",
                "addx 5",
                "noop",
                "noop",
                "noop",
                "noop",
                "noop",
                "addx -36",
                "noop",
                "addx 1",
                "addx 7",
                "noop",
                "noop",
                "noop",
                "addx 2",
                "addx 6",
                "noop",
                "noop",
                "noop",
                "noop",
                "noop",
                "addx 1",
                "noop",
                "noop",
                "addx 7",
                "addx 1",
                "noop",
                "addx -13",
                "addx 13",
                "addx 7",
                "noop",
                "addx 1",
                "addx -33",
                "noop",
                "noop",
                "noop",
                "addx 2",
                "noop",
                "noop",
                "noop",
                "addx 8",
                "noop",
                "addx -1",
                "addx 2",
                "addx 1",
                "noop",
                "addx 17",
                "addx -9",
                "addx 1",
                "addx 1",
                "addx -3",
                "addx 11",
                "noop",
                "noop",
                "addx 1",
                "noop",
                "addx 1",
                "noop",
                "noop",
                "addx -13",
                "addx -19",
                "addx 1",
                "addx 3",
                "addx 26",
                "addx -30",
                "addx 12",
                "addx -1",
                "addx 3",
                "addx 1",
                "noop",
                "noop",
                "noop",
                "addx -9",
                "addx 18",
                "addx 1",
                "addx 2",
                "noop",
                "noop",
                "addx 9",
                "noop",
                "noop",
                "noop",
                "addx -1",
                "addx 2",
                "addx -37",
                "addx 1",
                "addx 3",
                "noop",
                "addx 15",
                "addx -21",
                "addx 22",
                "addx -6",
                "addx 1",
                "noop",
                "addx 2",
                "addx 1",
                "noop",
                "addx -10",
                "noop",
                "noop",
                "addx 20",
                "addx 1",
                "addx 2",
                "addx 2",
                "addx -6",
                "addx -11",
                "noop",
                "noop",
                "noop",
            ])
        );

        let interesting_cycles = vec![
            // Cycle count, X _during_
            (20, 420),
            (60, 1140),
            (100, 1800),
            (140, 2940),
            (180, 2880),
            (220, 3960)
        ];

        let mut cpu = Cpu::new(ops);

        loop {
            let cycle_res = cpu.cycle();

            if cycle_res.is_none() {
                break;
            }

            let (cycle, x, _) = cycle_res.unwrap();

            let interesting = interesting_cycles.iter().find(|(i_c, _)| *i_c == cycle);

            match interesting {
                Some((_, signal_level)) => {
                    println!("Checking at cycle: {cycle}");
                    assert_eq!(cycle as isize * x, *signal_level);
                }

                _ => {
                }
            }
        }
    }
}
