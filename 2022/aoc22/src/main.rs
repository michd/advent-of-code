use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_one(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let map = Map::import(&input, 50);
    let instructions_raw = input
        .iter()
        .skip_while(|l| !l.is_empty())
        .skip_while(|l| l.is_empty())
        .nth(0)
        .unwrap();

    let instructions = Instruction::parse_all(instructions_raw);

    let cursor = map.execute(&instructions);

    format!("{}", cursor.as_password())
}

fn process_part_two(input: Vec<String>) -> String {
    format!("Output")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

type Loc = (isize, isize);
type LocDiff = (isize, isize);

#[derive(Debug, PartialEq, Copy, Clone)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

impl Facing {
    fn value(&self) -> usize {
        match self {
            Self::Right=> 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }

    fn from_value(val: isize) -> Self {
        let mut normalized = val % 4;

        normalized = if normalized < 0 {
            4 + normalized
        } else {
            normalized
        };

        match normalized {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => Self::Right,
        }
    }

    fn plus(&self, turn: isize) -> Self {
        let normalized_turn = if turn < 0 { -1 } else { 1 };
        Self::from_value(self.value() as isize + normalized_turn)
    }

    // x, y diff if a move forward happens
    fn coord_diff(&self) -> LocDiff {
        match self {
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Up => (0, -1)
        }
    }
}

#[derive(Debug, PartialEq)]
struct Cursor {
    loc: Loc,
    facing: Facing,
}

impl Cursor {
    fn as_password(&self) -> usize {
        let (x, y) = self.loc;

        let (row, column) = ((y + 1) as usize, (x + 1) as usize);
        let facing = self.facing.value();

        1000 * row + 4 * column + facing
    }

    fn loc_ahead(&self) -> Loc {
        let (x, y) = self.loc;
        let (xd, yd) = self.facing.coord_diff();
        (x + xd, y + yd)
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Go(usize),
    Turn(isize)
}

impl Instruction {
    fn parse_all(input: &str) -> Vec<Instruction> {
        let mut chars = input.chars();

        let mut instructions: Vec<Instruction> = vec![];
        let mut cur_chars: String = "".to_string();

        loop {
            let c = chars.next();

            match c {
                None | Some('L') | Some('R') => {
                    if let Ok(num) = usize::from_str_radix(&cur_chars, 10) {
                        instructions.push(Instruction::Go(num));
                    }

                    cur_chars = "".to_string();
                }

                _ => {}
            }

            match c {
                None => { break; }
                Some('L') => { instructions.push(Instruction::Turn(-1)); }
                Some('R') => { instructions.push(Instruction::Turn(1));  }
                Some('0'..='9') => { cur_chars.push(c.unwrap()); }
                _ => { panic!("Invalid char {}", c.unwrap()); }
            }
        }

        instructions
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    tiles: Vec<String>,
    face_size: usize,
}

impl Map {
    fn import(input: &Vec<String>, face_size: usize) -> Self {
        Map {
            tiles: input
                .iter()
                .take_while(|l| !l.is_empty())
                .map(|l| l.to_string())
                .collect(),

            face_size
        }
    }

    fn init_cursor(&self) -> Cursor {
        let first_open_x = self.tiles[0]
            .chars()
            .enumerate()
            .find(|(_, c)| *c == '.')
            .map(|(i, _)| i)
            .unwrap();

        Cursor {
            loc: (first_open_x as isize, 0),
            facing: Facing::Right
        }
    }

    fn resolve_loc(&self, loc: Loc, facing: &Facing) -> Loc {
        // Wraps where needed to not be on unavailable space
        // Returns a loc of either open space or wall.

        match *facing {
            Facing::Right => {
                let (mut x, y) = loc;
                // If we're past the end
                if x >= self.tiles[y as usize].chars().count() as isize {
                    // Find the first tile on this row from the beginning
                    // Does not care if the tile is a wall, just needs to be avail or wall.
                    x = self.tiles[y as usize]
                        .chars()
                        .enumerate()
                        .find(|(_, t)| *t != ' ')
                        .map(|(i, _)| i as isize)
                        .unwrap_or(x);
                }

                (x, y)
            }

            Facing::Left => {
                let (mut x, y) = loc;

                // If we're past the end
                if x < 0 || self.tiles[y as usize].chars().nth(x as usize).unwrap() == ' ' {
                    x = self.tiles[y as usize]
                        .chars()
                        .collect::<Vec<char>>()
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, t)| **t != ' ')
                        .map(|(i, _)| i as isize)
                        .unwrap_or(x);
                }

                (x, y)
            }

            Facing::Down => {
                let (x, mut y) = loc;

                // If we're past the end
                if y as usize >= self.tiles.len()
                    || self.tiles[y as usize].chars().nth(x as usize).unwrap_or(' ') == ' ' {

                    y = self.tiles
                        .iter()
                        .enumerate()
                        .find(|(_, r)| {
                            r.chars().nth(x as usize).unwrap_or(' ') != ' '
                        })
                        .map(|(i, _)| i as isize)
                        .unwrap_or(y)
                }

                (x, y)
            }

            Facing::Up=> {
                let (x, mut y) = loc;

                // If we're past the end
                if y < 0 || self.tiles[y as usize].chars().nth(x as usize).unwrap_or(' ') == ' ' {
                    y = self.tiles
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, r)| {
                            r.chars().nth(x as usize).unwrap_or(' ') != ' '
                        })
                        .map(|(i, _)| i as isize)
                        .unwrap_or(y)
                }

                (x, y)
            }
        }
    }

    fn resolve_loc_and_facing_cube(&self, loc: Loc, facing: &Facing) -> (Loc, Facing) {
        let (x, y) = loc;
        if y >= 0
            && (y as usize) < self.tiles.len()
            && x >= 0
            && self.tiles[y as usize].chars().nth(x as usize).unwrap_or(' ') != ' ' {
            // Coordinate directly accessible, nothing to do
            return (loc, *facing);
        }

        // Okay, figure out warping. We want to traverse 4 joints between faces to end up on the
        // face we need to be on. Then, based on the directions we've had to move to get there,
        // we can determine where we're facing and how to mangle coordinates.
        // TODO next

        // Q: how to detect which face we're on and where to go?



        ((0, 0), Facing::Right)
    }

    fn move_cursor(&self, cursor: &Cursor) -> Loc {
        let target_loc = self.resolve_loc(cursor.loc_ahead(), &cursor.facing);

        let (xt, yt) = target_loc;

        if self.tiles[yt as usize].chars().nth(xt as usize).unwrap_or('#') == '#' {
            cursor.loc
        } else {
            target_loc
        }
    }

    fn execute(&self, program: &Vec<Instruction>) -> Cursor {
        let mut cursor = self.init_cursor();

        program
            .iter()
            .for_each(|inst| {
                match inst {
                    Instruction::Go(n) => {
                        for _ in 0..*n {
                            let new_loc = self.move_cursor(&cursor);
                            if new_loc == cursor.loc { break; }
                            cursor.loc = new_loc;
                        }
                    }

                    Instruction::Turn(dir) => {
                        cursor.facing = cursor.facing.plus(*dir);
                    }
                }
            });

        cursor
    }
}

// TODO edges might not actually be needed
#[derive(Debug, PartialEq)]
enum Edge {
    A, B, C, D, E, F, G, H, I, J, K, L,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ModelCubeFace {
    Front,
    Left,
    Rear,
    Right,
    Top,
    Bottom,
}
c
impl ModelCubeFace {
    // TODO this is probably not needed
    fn get_edge(&self, dir: Facing) -> Edge {
        match self {
            Self::Front => match dir {
                Facing::Up => Edge::A,
                Facing::Right => Edge::B,
                Facing::Down => Edge::C,
                Facing::Left => Edge::D,
            }

            Self::Left => match dir {
                Facing::Up => Edge::E,
                Facing::Right => Edge::D,
                Facing::Down => Edge::G,
                Facing::Left =>  Edge::I,
            }

            Self::Rear => match dir {
                Facing::Right => Edge::I,
                Facing::Down => Edge::L,
                Facing::Left => Edge::K,
                Facing::Up => Edge::J,
            }

            Self::Right => match dir {
                Facing::Right => Edge::K,
                Facing::Down => Edge::H,
                Facing::Left => Edge::B,
                Facing::Up => Edge::F,
            }

            Self::Top => match dir {
                Facing::Right => Edge::F,
                Facing::Down => Edge::A,
                Facing::Left => Edge::E,
                Facing::Up => Edge::J,
            }

            Self::Bottom => match dir {
                Facing::Right => Edge::H,
                Facing::Down => Edge::L,
                Facing::Left => Edge::G,
                Facing::Up => Edge::C,
            }
        }
    }

    fn get_face(&self, dir: Facing) -> Self {
        match self {
            Self::Front => match dir {
                Facing::Right => Self::Right,
                Facing::Down => Self::Bottom,
                Facing::Left => Self::Left,
                Facing::Up => Self::Top,
            }

            Self::Left => match dir {
                Facing::Right => Self::Front,
                Facing::Down => Self::Bottom,
                Facing::Left => Self::Rear,
                Facing::Up => Self::Top,
            }

            Self::Rear => match dir {
                Facing::Right => Self::Left,
                Facing::Down => Self::Bottom,
                Facing::Left => Self::Right,
                Facing::Up => Self::Top,
            }

            Self::Right => match dir {
                Facing::Right => Self::Rear,
                Facing::Down => Self::Bottom,
                Facing::Left => Self::Front,
                Facing::Up => Self::Top,
            }

            Self::Top => match dir {
                Facing::Right => Self::Right,
                Facing::Down => Self::Front,
                Facing::Left => Self::Left,
                Facing::Up => Self::Rear,
            }

            Self::Bottom => match dir {
                Facing::Right => Self::Right,
                Facing::Down => Self::Rear,
                Facing::Left => Self::Left,
                Facing::Up => Self::Front,
            }
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
struct CubeFace {
    // Top left index in unfolded map
    loc: Loc,
    model_face: Option<ModelCubeFace>,
}

impl CubeFace {
    fn new(loc: Loc) -> Self {
        CubeFace {
            loc,
            model_face: None,
        }
    }
}


#[derive(Debug, PartialEq)]
struct Cube {
    faces: HashMap<Loc, CubeFace>,
}

impl Cube {
    fn have_face_at(map: &Map, x: usize, y: usize) -> bool {
        if y >= map.tiles.len() { return false; }
        map.tiles[y].chars().nth(x).unwrap_or(' ') != ' '
    }

    fn read_map(map: &Map) -> Self {

        // We can have a max grid size of 4x4, so let's scan that.

        let mut faces = HashMap::new();

        // First collect existing faces
        for y in (0..(4 * map.face_size)).step_by(map.face_size) {
            for x in (0..(4 * map.face_size)).step_by(map.face_size) {
                if Self::have_face_at(map, x, y) {
                    let loc = (x as isize, y as isize);
                    faces.insert(loc, CubeFace::new(loc));
                }
            }
        }

        let face = faces.iter().nth(0).unwrap();
        let (_, front_face) = face;
        let mut front_face = front_face.clone();

        front_face.model_face = Some(ModelCubeFace::Front);

        // Okay I've run out of my self-imposed time limit.
        //
        // TODO, at some point:
        // travers from the front face, visiting all adjacent faces, and assigning the
        // appropriate ModelCubeFace to each of them.

        // Then from that point, we know each side's adjacent faces, and can thus find which
        // place to continue from to wrap around.

        Cube {
            faces
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::*;

    fn get_test_input() -> Vec<String> {
        vec![
            "        ...#".to_string(),
            "        .#..".to_string(),
            "        #...".to_string(),
            "        ....".to_string(),
            "...#.......#".to_string(),
            "........#...".to_string(),
            "..#....#....".to_string(),
            "..........#.".to_string(),
            "        ...#....".to_string(),
            "        .....#..".to_string(),
            "        .#......".to_string(),
            "        ......#.".to_string(),
            "".to_string(),
            "10R5L5R10L4R5L5".to_string(),
        ]
    }

    #[test]
    fn test_init_cursor() {
        let map = Map::import(&get_test_input(), 4);
        let cursor = map.init_cursor();

        assert_eq!(
            cursor,
            Cursor {
                loc: (8, 0),
                facing: Facing::Right,
            }
        )
    }

    #[test]
    fn test_parse_instructions() {

        assert_eq!(
            Instruction::parse_all("10R5L5R10L4R5L5"),
            vec![
                Instruction::Go(10),
                Instruction::Turn(1),
                Instruction::Go(5),
                Instruction::Turn(-1),
                Instruction::Go(5),
                Instruction::Turn(1),
                Instruction::Go(10),
                Instruction::Turn(-1),
                Instruction::Go(4),
                Instruction::Turn(1),
                Instruction::Go(5),
                Instruction::Turn(-1),
                Instruction::Go(5),
            ]
        )
    }

    #[test]
    fn test_resolve_loc() {
        let map = Map::import(&get_test_input(), 4);

        // An available location (either open or wall, but not nothing), should resolve to same
        assert_eq!(map.resolve_loc((9, 2), &Facing::Right), (9, 2));
        assert_eq!(map.resolve_loc((9, 2), &Facing::Left), (9, 2));
        assert_eq!(map.resolve_loc((9, 2), &Facing::Down), (9, 2));
        assert_eq!(map.resolve_loc((9, 2), &Facing::Up), (9, 2));

        // Locations off the end
        // Left/right
        assert_eq!(map.resolve_loc((12, 1), &Facing::Right), (8, 1));
        assert_eq!(map.resolve_loc((7, 1), &Facing::Left), (11, 1));
        assert_eq!(map.resolve_loc((-1, 4), &Facing::Left), (11, 4));

        // Up/down
        assert_eq!(map.resolve_loc((8, -1), &Facing::Up), (8, 11));
        assert_eq!(map.resolve_loc((0, 3), &Facing::Up), (0, 7));
        assert_eq!(map.resolve_loc((13, 7), &Facing::Up), (13, 11));
        assert_eq!(map.resolve_loc((8, 12), &Facing::Down), (8, 0));
        assert_eq!(map.resolve_loc((2, 8), &Facing::Down), (2,4));
    }

    #[test]
    fn test_facing_plus() {
        assert_eq!(Facing::Right.plus(1), Facing::Down);
        assert_eq!(Facing::Down.plus(1), Facing::Left);
        assert_eq!(Facing::Left.plus(1), Facing::Up);
        assert_eq!(Facing::Up.plus(1), Facing::Right);
        assert_eq!(Facing::Right.plus(-1), Facing::Up);
        assert_eq!(Facing::Down.plus(-1), Facing::Right);
        assert_eq!(Facing::Left.plus(-1), Facing::Down);
        assert_eq!(Facing::Up.plus(-1), Facing::Left);
    }

    #[test]
    fn test_execute_program() {
        let map = Map::import(&get_test_input(), 4);
        let instructions =Instruction::parse_all("10R5L5R10L4R5L5");

        let cursor = map.execute(&instructions);

        assert_eq!(cursor.as_password(), 6032);
    }

    #[test]
    fn test_cube_read_map() {
        // TODO check if all the ModelCubeFaces are assigned correctly
    }
}
