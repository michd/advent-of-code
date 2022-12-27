use std::collections::{HashMap, HashSet, VecDeque};
use std::io;
use std::io::BufRead;
use std::ops::Deref;

// TODO since the result is wrong (250 too high, 232 too low), try keeping track of walls
// and rework based on their positions
fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let valley = Valley::import(&input);
    format!("{}", valley.shortest_path(vec![valley.end]))
}

fn process_part_two(input: Vec<String>) -> String {
    let valley = Valley::import(&input);
    format!("{}", valley.shortest_path(vec![valley.end, valley.start, valley.end]))
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn next_wind_loc(&self, valley: &Valley, current: Loc) -> Loc {
        let (x, y) = current;

        match self {
            Direction::North => {
                let new_y = if y == 1 { valley.height as isize - 2 } else { y - 1 };
                (x, new_y)
            }

            Direction::East => {
                let new_x = if x == valley.width as isize - 2 { 1 } else { x + 1 };
                (new_x, y)
            }

            Direction::South => {
                let new_y = if y == valley.height as isize - 2 { 1 } else { y + 1 };
                (x, new_y)
            }

            Direction::West => {
                let new_x = if x == 1 { valley.width as isize - 2 } else { x - 1 };
                (new_x, y)
            }
        }
    }
}

type Loc = (isize, isize);

#[derive(Debug, PartialEq, Clone)]
struct Valley {
    width: usize,
    height: usize,
    start: Loc,
    end: Loc,
    north_winds: Vec<Loc>,
    east_winds: Vec<Loc>,
    south_winds: Vec<Loc>,
    west_winds: Vec<Loc>,
    walls: Vec<Loc>,
}

impl Valley {
    fn import(input: &Vec<String>) -> Self {
        let width = input[0].chars().count();
        let height = input.len();

        let mut north_winds: Vec<Loc> = vec![];
        let mut east_winds: Vec<Loc> = vec![];
        let mut south_winds: Vec<Loc> = vec![];
        let mut west_winds: Vec<Loc> = vec![];

        let mut walls: Vec<Loc> = vec![];

        input
            .iter()
            .enumerate()
            .for_each(|(y, l)| {
                l
                    .chars()
                    .enumerate()
                    .for_each(|(x, c)| {
                        match c {
                            '#' => { walls.push((x as isize, y as isize)); }
                            '<' => { west_winds.push((x as isize, y as isize)); }
                            '>' => { east_winds.push((x as isize, y as isize)); }
                            '^' => { north_winds.push((x as isize, y as isize)); }
                            'v' => { south_winds.push((x as isize, y as isize)); }
                            _ => { return; }
                        }
                    });
            });

        Self {
            width,
            height,
            start: (1, 0),
            end: (width as isize - 2, height as isize - 1),
            north_winds,
            east_winds,
            south_winds,
            west_winds,
            walls,
        }
    }

    // Calculates next positions of each bit of wind and returns new instance
    fn next_frame(&self) -> Self {
        let north_winds = self.north_winds
            .iter()
            .map(|l| Direction::North.next_wind_loc(self, *l))
            .collect();

        let east_winds = self.east_winds
            .iter()
            .map(|l| Direction::East.next_wind_loc(self, *l))
            .collect();

        let south_winds = self.south_winds
            .iter()
            .map(|l| Direction::South.next_wind_loc(self, *l))
            .collect();

        let west_winds = self.west_winds
            .iter()
            .map(|l| Direction::West.next_wind_loc(self, *l))
            .collect();

        Valley {
            north_winds,
            east_winds,
            south_winds,
            west_winds,
            width: self.width,
            height: self.height,
            start: self.start,
            end: self.end,
            walls: self.walls.clone(),
        }
    }

    fn available_positions(&self, current: Loc) -> Vec<Loc> {
        let (x, y) = current;
        let (sx, sy) = self.start;
        let (ex, ey) = self.end;
        vec![
            (x, y - 1),
            (x - 1, y),
            (x, y),
            (x + 1, y),
            (x, y + 1)
        ]
            .iter()
            .filter(|(px, py)| {
                if *px < 0 || *px > self.width as isize - 1 || *py < 0 || *py > self.height as isize - 1 {
                    return false;
                }

                let pos = (*px, *py);
                !self.walls.contains(&pos)
                    && !self.north_winds.contains(&pos)
                    && !self.east_winds.contains(&pos)
                    && !self.south_winds.contains(&pos)
                    && !self.west_winds.contains(&pos)
            })
            .map(|i| *i)
            .collect()
    }

    fn shortest_path(&self, mut goals: Vec<Loc>) -> usize {

        let mut visited: HashSet<(usize, Loc)> = HashSet::new();
        let mut queue: VecDeque<(usize, Loc)> = VecDeque::new();
        queue.push_back((0, self.start));
        let mut valley_states: Vec<Self> = vec![];

        valley_states.push(self.clone());

        let mut goal = goals.pop().unwrap();

        loop {
            if queue.len() == 0 {
                break;
            }

            let (depth, pos) = queue.pop_front().unwrap();

            if visited.contains(&(depth, pos)) { continue; }

            let valley = valley_states
                .get(depth)
                .map(|v| v.clone())
                .unwrap_or_else(|| {
                    valley_states[depth - 1].next_frame()
                });

            if valley_states.len() <= depth {
                valley_states.push(valley.clone());
            }

            visited.insert((depth, pos));

            if pos == goal {
                if let Some(new_goal) = goals.pop() {
                    goal = new_goal;
                    queue.clear();
                    // TODO clear queue where relevant, mayne outright
                } else {
                    return depth;
                }
            }

            let next_valley = valley_states
                .get(depth + 1)
                .map(|v| v.clone())
                .unwrap_or_else(|| {
                    valley.next_frame()
                });

            if valley_states.len() <= depth + 1 {
                valley_states.push(next_valley.clone());
            }

            //if depth == 0 {
            //    println!("\nInitial state:");
            //} else {
            //    println!("\nMinute {}:", depth);
            //}
            //valley.print(pos);

            next_valley.available_positions(pos)
                .iter()
                .for_each(|loc| {
                    queue.push_back((depth + 1, *loc));
                });
        }

        0
    }

    fn print(&self, current: Loc) {
        let (sx, sy) = self.start;
        let (ex, ey) = self.end;
        let (cx, cy) = current;

        (0..self.height as isize).for_each(|y| {
            (0..self.width as isize).for_each(|x| {

                let pos = (x, y);
                let w = self.walls.contains(&pos);
                let n_c = self.north_winds.contains(&pos);
                let e_c = self.east_winds.contains(&pos);
                let s_c = self.south_winds.contains(&pos);
                let w_c = self.west_winds.contains(&pos);

                let wind_count = if n_c { 1 } else { 0 } + if e_c { 1 } else { 0 } + if s_c { 1 } else { 0 } + if w_c { 1 } else { 0 };

                if (w || n_c || e_c || s_c || w_c) && cx ==x && cy == y {
                    print!("?");
                } else {
                    if w {
                        print!("#");
                    }
                    else if cx == x && cy == y {
                        print!("E");
                    } else if wind_count == 0 {
                        print!(".");
                    }
                    else if wind_count > 1 {
                        print!("{}", wind_count);
                    }
                    else {
                        print!(
                            "{}",
                            (if n_c { "^" } else if e_c { ">" } else if s_c { "v" } else { "<" }).to_string()
                        );
                    }
                }

                if x == self.width as isize - 1 {
                    print!("\n");
                }
            });
        });
    }
}
#[cfg(test)]
mod tests {
    use crate::*;

    fn get_input_simple() -> Vec<String> {
        vec![
            "#.#####".to_string(),
            "#.....#".to_string(),
            "#>....#".to_string(),
            "#.....#".to_string(),
            "#...v.#".to_string(),
            "#.....#".to_string(),
            "#####.#".to_string(),
        ]
    }

    fn get_input_complex() -> Vec<String> {
        vec![
            "#.######".to_string(),
            "#>>.<^<#".to_string(),
            "#.<..<<#".to_string(),
            "#>v.><>#".to_string(),
            "#<^v^^>#".to_string(),
            "######.#".to_string(),
        ]
    }

    #[test]
    fn test_valley_import() {
        let valley = Valley::import(&get_input_simple());

        assert_eq!(
            valley,
            Valley {
                width: 7,
                height: 7,
                start: (1, 0),
                end: (5, 6),
                north_winds: vec![],
                east_winds: vec![(1, 2)],
                south_winds: vec![(4, 4)],
                west_winds: vec![],
                walls: vec![
                    (0, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0),
                    (0, 1), (6, 1),
                    (0, 2), (6, 2),
                    (0, 3), (6, 3),
                    (0, 4), (6, 4),
                    (0, 5), (6, 5),
                    (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (6, 6),
                ]
            }
        )
    }

    #[test]
    fn test_shortest_path() {
        let valley = Valley::import(&get_input_complex());

        assert_eq!(valley.shortest_path(vec![valley.end]), 18);
    }

    #[test]
    fn test_shortes_path_multiple() {
        let valley = Valley::import(&get_input_complex());

        assert_eq!(valley.shortest_path(vec![valley.end, valley.start, valley.end]), 54);
    }
}
