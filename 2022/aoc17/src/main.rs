use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_one(input);
    println!("{output}");
}

// That's right we're gonna do bitwise stuff for this one.
const SHAPES: &'static [[u16; 4]; 5] = &[
    [
        0b000000000u16,
        0b000000000u16,
        0b000000000u16,
        0b000111100u16,
    ],
    [
        0b000000000u16,
        0b000010000u16,
        0b000111000u16,
        0b000010000u16,
    ],
    [
        0b000000000u16,
        0b000001000u16,
        0b000001000u16,
        0b000111000u16,
    ],
    [
        0b000100000u16,
        0b000100000u16,
        0b000100000u16,
        0b000100000u16,
    ],
    [
        0b000000000u16,
        0b000000000u16,
        0b000110000u16,
        0b000110000u16,
    ],
];

const WALL: u16 = 0b100000001u16;
const FLOOR: u16 = 0b111111111u16;

fn process_part_one(input: Vec<String>) -> String {
    let mut cave = Cave::new(input[0].to_string());

    for _ in 0..2022 {
        cave.drop_rock();
    }

    format!("{}", cave.stack_height)
}

fn process_part_two(input: Vec<String>) -> String {
    format!("Output")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

/// Using bitwise logic this checks whether a proposed position of a rock
/// overlaps at all with existing material in this row.
fn collides(existing: u16, rock: u16) -> bool {
    existing & rock != 0
}

fn push_rock(rock: &Vec<u16>, jet: char) -> Vec<u16> {

    let would_collide = rock
        .iter()
        .any(|layer| {
            collides(
                WALL,
                match jet {
                    '<' => *layer << 1,
                    '>' => *layer >> 1,
                    _ => *layer,
                }
            )
        });

    if !would_collide {
        rock
            .iter()
            .map(|layer| {
                match jet {
                    '<' => *layer << 1,
                    '>' => *layer >> 1,
                    _ => *layer
                }
            })
            .collect()
    } else {
        rock.clone()
    }
}

struct Cave {
    jet_pattern: String,
    jet_index: usize,
    jet_count: usize,
    rock_index: usize,
    stack: Vec<u16>,
    stack_height: usize,
}

impl Cave {
    fn new(jet_pattern: String) -> Self {
        Self {
            jet_pattern,
            jet_index: 0,
            jet_count: 0,
            rock_index: 0,
            stack: vec![FLOOR],
            stack_height: 0,
        }
    }

    fn next_rock(&mut self) -> Vec<u16> {
        let rock = self.peek_next_rock();
        self.rock_index = (self.rock_index + 1) % 5;
        rock
    }

    fn peek_next_rock(&self) -> Vec<u16> {
        let mut rock = vec![];
        for layer in SHAPES[self.rock_index] {
            rock.push(layer)
        }

        rock
    }

    fn next_jet(&mut self) -> char {
        let jet_char = self.jet_pattern.chars().nth(self.jet_index).unwrap();
        self.jet_index = (self.jet_index + 1) % self.jet_pattern.chars().count();
        self.jet_count += 1;
        jet_char
    }

    fn jet_push_rock(&self, y: usize, jet: char, rock: &Vec<u16>) -> Vec<u16> {
        // Iterate over the layers of the rock + layers of the stack matching its current
        // position (or use walls if stack not that high yet),
        // and see if it collides

        let would_collide = rock.iter().rev().enumerate().any(|(i, rock_layer)| {
            let existing_layer = *self.stack.get(y + i).unwrap_or(&WALL);
            let shifted = match jet {
                '<' => *rock_layer << 1,
                '>' => *rock_layer >> 1,
                _ => *rock_layer,
            };
            collides(existing_layer, shifted)
        });

        if would_collide {
            rock.clone()
        } else {
            rock.iter().map(|l| {
                match jet {
                    '<' => *l << 1,
                    '>' => *l >> 1,
                    _ => *l
                }
            }).collect()
        }
    }

    fn can_lower_rock(&self, y: usize, rock: &Vec<u16>) -> bool {
        !rock.iter().rev().enumerate().any(|(i, rock_layer)| {
          let existing_layer = *self.stack.get(y + i - 1).unwrap_or(&WALL);
           collides(existing_layer, *rock_layer)
        })
    }

    // Returns whether it cleared the stack
    fn deposit_rock(&mut self, y: usize, rock: &Vec<u16>) {
        let mut stack_y = y;

        rock.iter().rev().for_each(|rock_layer| {
            if *rock_layer == 0 {
                return;
            }

            while self.stack.len() <= stack_y {
                self.stack.push(WALL);
                self.stack_height += 1;
            }

            self.stack[stack_y] = self.stack[stack_y] | *rock_layer;
            stack_y += 1;
        });
    }

    fn drop_rock(&mut self) {
        let stack_top = self.stack.len() - 1;
        let mut rock = self.next_rock();
        let mut y = stack_top + 4;

        loop {
            let jet = self.next_jet();

            rock = if y >= self.stack.len() {
                push_rock(&rock, jet)
            } else {
                self.jet_push_rock(y, jet, &rock)
            };

            if self.can_lower_rock(y, &rock) {
                // Lower the rock as it won't collide yet
                y -= 1
            } else {
                self.deposit_rock(y, &rock);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn testing_works() {
        assert_eq!(2,  1 + 1);
    }

    #[test]
    fn test_next_rock() {
        let jet_pattern_raw = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string();
        let mut cave = Cave::new(jet_pattern_raw);

        let rock = cave.next_rock();
        assert_eq!(cave.rock_index, 1);
        assert_eq!(rock, vec![
            0b000000000u16,
            0b000000000u16,
            0b000000000u16,
            0b000111100u16,
        ]);

        let rock = cave.next_rock();
        assert_eq!(cave.rock_index, 2);
        assert_eq!(rock, vec![
            0b000000000u16,
            0b000010000u16,
            0b000111000u16,
            0b000010000u16,
        ]);

        let rock = cave.next_rock();
        assert_eq!(cave.rock_index, 3);
        assert_eq!(rock, vec![
            0b000000000u16,
            0b000001000u16,
            0b000001000u16,
            0b000111000u16,
        ]);

        let rock = cave.next_rock();
        assert_eq!(cave.rock_index, 4);
        assert_eq!(rock, vec![
            0b000100000u16,
            0b000100000u16,
            0b000100000u16,
            0b000100000u16,
        ]);

        let rock = cave.next_rock();
        assert_eq!(cave.rock_index, 0);
        assert_eq!(rock, vec![
            0b000000000u16,
            0b000000000u16,
            0b000110000u16,
            0b000110000u16,
        ]);

        let rock = cave.next_rock();
        assert_eq!(cave.rock_index, 1);
        assert_eq!(rock, vec![
            0b000000000u16,
            0b000000000u16,
            0b000000000u16,
            0b000111100u16,
        ]);
    }

    #[test]
    fn test_drop_rock() {
        let jet_pattern_raw = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string();
        let mut cave = Cave::new(jet_pattern_raw);

        assert_eq!(
            cave.stack, vec![FLOOR]
        );

        cave.drop_rock();
        assert_eq!(
            cave.stack, vec![
                FLOOR,
                WALL | 0b000111100,
            ]
        );

        cave.drop_rock();
        assert_eq!(
            cave.stack, vec![
                FLOOR,
                WALL | 0b000111100,
                WALL | 0b000010000,
                WALL | 0b000111000,
                WALL | 0b000010000,
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let jet_pattern_raw = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>".to_string();
        let mut cave = Cave::new(jet_pattern_raw);

        for _ in 0..2022 {
            cave.drop_rock();
        }

        assert_eq!(cave.stack.len() - 1, 3068);
    }
}
