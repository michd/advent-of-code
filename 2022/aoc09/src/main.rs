use std::io;
use std::io::BufRead;
use std::collections::HashSet;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let mut bridge = RopeBridge::new(2);

    input.iter().for_each(|l| {
        let split: Vec<&str> = l.split(" ").collect();

        let ch = split[0].chars().nth(0).unwrap();
        let count = usize::from_str_radix(split[1], 10).unwrap();
        let instr = Move::from_char(ch).unwrap();

        (0..count).for_each(|_| {
            bridge.move_head(&instr);
        });
    });

    let distinct_tail_positions = bridge.tail_positions.len();
    format!("{distinct_tail_positions}")
}

fn process_part_two(input: Vec<String>) -> String {
    let mut bridge = RopeBridge::new(10);

    input.iter().for_each(|l| {
        let split: Vec<&str> = l.split(" ").collect();

        let ch = split[0].chars().nth(0).unwrap();
        let count = usize::from_str_radix(split[1], 10).unwrap();
        let instr = Move::from_char(ch).unwrap();

        (0..count).for_each(|_| {
            bridge.move_head(&instr);
        });
    });

    let distinct_tail_positions = bridge.tail_positions.len();
    format!("{distinct_tail_positions}")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn is_adjacent_to(&self, other: &Position) -> bool {
        ((self.x - 1)..=(self.x + 1)).contains(&other.x) 
            && ((self.y - 1)..=(self.y + 1)).contains(&other.y)
    }

    fn plus(&self, instr: &Move) -> Position {
        match *instr {
            Move::Up => Position { x: self.x, y: self.y + 1 },
            Move::Right => Position { x: self.x + 1, y: self.y },
            Move::Down => Position { x: self.x, y: self.y - 1 },
            Move::Left => Position { x: self.x - 1, y: self.y }
        }
    }
}

#[derive(Debug)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

impl Move {
    fn from_char(c: char) -> Option<Move> {
        match c {
            'U' => Some(Move::Up),
            'R' => Some(Move::Right),
            'D' => Some(Move::Down),
            'L' => Some(Move::Left),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct RopeBridge {
    knots: Vec<Position>,
    length: usize,
    tail_positions: HashSet<Position>,
}

impl RopeBridge {
    fn new(length: usize) -> Self {
        let mut bridge = Self {
            knots: (0..length).map(|_| Position { x: 0, y: 0 }).collect(),
            length: length,
            tail_positions: HashSet::new(),
        };

        bridge.tail_positions.insert(bridge.knots[length - 1].clone());
        bridge
    }

    fn set_knot(&mut self, index: usize, pos: &Position) {
        self.knots[index] = *pos;

        if index == self.length - 1 {
            self.tail_positions.insert(*pos);
        }
    }

    fn head(&self) -> Position {
        self.knots[0]
    }

    fn tail(&self) -> Position {
        self.knots[self.length - 1]
    }

    fn move_head(&mut self, instr: &Move) {
        self.set_knot(0, &self.knots[0].plus(instr));

        for i in 1..(self.length) {
            let puller = self.knots[i - 1];
            let pulled = self.knots[i];

            if pulled.is_adjacent_to(&puller) {
                continue;
            }

            let x_diff = pulled.x - puller.x;
            let y_diff = pulled.y - puller.y;

            let new_x = if x_diff > 1 || x_diff < -1 {
                puller.x + if pulled.x > puller.x { 1 } else { -1 }
            } else {
                if y_diff > 1 || y_diff < -1 { puller.x } else { pulled.x }
            };

            let new_y = if y_diff > 1 || y_diff < -1 {
                puller.y + if pulled.y > puller.y { 1 } else { -1 }
            } else {
                if x_diff > 1 || x_diff < -1 { puller.y } else { pulled.y }
            };

            self.set_knot(i, &Position { x: new_x, y: new_y });
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    fn pos(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    #[test]
    fn test_position_is_adjacent() {
        assert!(Position { x: 0, y: 0 }.is_adjacent_to(&pos(0, 0)));
        assert!(Position { x: 0, y: 0 }.is_adjacent_to(&pos(0, 1)));
        assert!(Position { x: 0, y: 0 }.is_adjacent_to(&pos(0, -1)));
        assert!(Position { x: 0, y: 0 }.is_adjacent_to(&pos(1, 0)));
        assert!(Position { x: 0, y: 0 }.is_adjacent_to(&pos(1, 1)));
        assert!(Position { x: 0, y: 0 }.is_adjacent_to(&pos(1, -1)));
        assert!(Position { x: 0, y: 0 }.is_adjacent_to(&pos(-1, 0)));
        assert!(Position { x: 0, y: 0 }.is_adjacent_to(&pos(-1, 1)));
        assert!(Position { x: 0, y: 0 }.is_adjacent_to(&pos(-1, -1)));
        assert!( ! Position { x: 0, y: 0 }.is_adjacent_to(&pos(0, 2)));
        assert!( ! Position { x: 0, y: 0 }.is_adjacent_to(&pos(0, -2)));
        assert!( ! Position { x: 0, y: 0 }.is_adjacent_to(&pos(2, 0)));
        assert!( ! Position { x: 0, y: 0 }.is_adjacent_to(&pos(2, 2)));
        assert!( ! Position { x: 0, y: 0 }.is_adjacent_to(&pos(2, -2)));
        assert!( ! Position { x: 0, y: 0 }.is_adjacent_to(&pos(-2, 0)));
        assert!( ! Position { x: 0, y: 0 }.is_adjacent_to(&pos(-2, 2)));
        assert!( ! Position { x: 0, y: 0 }.is_adjacent_to(&pos(-2, -2)));
    }

    #[test]
    // No unit tests for part two, but the principle is the same so ehhh.
    fn test_move_head() {
        let mut bridge = RopeBridge::new(2);

        // Init
        assert_eq!(bridge.head(), pos(0, 0));
        assert_eq!(bridge.tail(), pos(0, 0));

        // R 4
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(1, 0));
        assert_eq!(bridge.tail(), pos(0, 0));
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(2, 0));
        assert_eq!(bridge.tail(), pos(1, 0));
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(3, 0));
        assert_eq!(bridge.tail(), pos(2, 0));
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(4, 0));
        assert_eq!(bridge.tail(), pos(3, 0));

        // U4
        bridge.move_head(&Move::Up);
        assert_eq!(bridge.head(), pos(4, 1));
        assert_eq!(bridge.tail(), pos(3, 0));
        bridge.move_head(&Move::Up);
        assert_eq!(bridge.head(), pos(4, 2));
        assert_eq!(bridge.tail(), pos(4, 1));
        bridge.move_head(&Move::Up);
        assert_eq!(bridge.head(), pos(4, 3));
        assert_eq!(bridge.tail(), pos(4, 2));
        bridge.move_head(&Move::Up);
        assert_eq!(bridge.head(), pos(4, 4));
        assert_eq!(bridge.tail(), pos(4, 3));

        // L3
        bridge.move_head(&Move::Left);
        assert_eq!(bridge.head(), pos(3, 4));
        assert_eq!(bridge.tail(), pos(4, 3));
        bridge.move_head(&Move::Left);
        assert_eq!(bridge.head(), pos(2, 4));
        assert_eq!(bridge.tail(), pos(3, 4));
        bridge.move_head(&Move::Left);
        assert_eq!(bridge.head(), pos(1, 4));
        assert_eq!(bridge.tail(), pos(2, 4));

        // D1
        bridge.move_head(&Move::Down);
        assert_eq!(bridge.head(), pos(1, 3));
        assert_eq!(bridge.tail(), pos(2, 4));

        // R4
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(2, 3));
        assert_eq!(bridge.tail(), pos(2, 4));
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(3, 3));
        assert_eq!(bridge.tail(), pos(2, 4));
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(4, 3));
        assert_eq!(bridge.tail(), pos(3, 3));
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(5, 3));
        assert_eq!(bridge.tail(), pos(4, 3));

        // D1
        bridge.move_head(&Move::Down);
        assert_eq!(bridge.head(), pos(5, 2));
        assert_eq!(bridge.tail(), pos(4, 3));

        // L5
        bridge.move_head(&Move::Left);
        assert_eq!(bridge.head(), pos(4, 2));
        assert_eq!(bridge.tail(), pos(4, 3));
        bridge.move_head(&Move::Left);
        assert_eq!(bridge.head(), pos(3, 2));
        assert_eq!(bridge.tail(), pos(4, 3));
        bridge.move_head(&Move::Left);
        assert_eq!(bridge.head(), pos(2, 2));
        assert_eq!(bridge.tail(), pos(3, 2));
        bridge.move_head(&Move::Left);
        assert_eq!(bridge.head(), pos(1, 2));
        assert_eq!(bridge.tail(), pos(2, 2));
        bridge.move_head(&Move::Left);
        assert_eq!(bridge.head(), pos(0, 2));
        assert_eq!(bridge.tail(), pos(1, 2));

        // R2
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(1, 2));
        assert_eq!(bridge.tail(), pos(1, 2));
        bridge.move_head(&Move::Right);
        assert_eq!(bridge.head(), pos(2, 2));
        assert_eq!(bridge.tail(), pos(1, 2));

        assert_eq!(bridge.tail_positions.len(), 13);
    }
}
