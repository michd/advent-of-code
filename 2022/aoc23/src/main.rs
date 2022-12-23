use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let mut troop = ElfTroop::import(&input);

    (0..10).for_each(|_| { troop.process_round(); });

    format!("{}", troop.count_empty_squares())
}

fn process_part_two(input: Vec<String>) -> String {
    let mut troop = ElfTroop::import(&input);

    let rounds = troop.process_until_done();

    format!("{}", rounds)
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

type Loc = (isize, isize);

fn get_surrounding_locs(loc: Loc) -> Vec<Loc> {
    let (x, y) = loc;

    vec![
        (x - 1, y - 1), (x, y - 1), (x + 1, y - 1),
        (x - 1, y), (x + 1, y),
        (x - 1, y + 1), (x, y + 1), (x + 1, y + 1),
    ]
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn value(&self) -> usize {
        match self {
            Self::North => 0,
            Self::South => 1,
            Self::West => 2,
            Self::East => 3,
        }
    }

    fn from_value(val: usize) -> Self {
        match val % 4 {
            0 => Self::North,
            1 => Self::South,
            2 => Self::West,
            3 => Self::East,
            _ => Self::North,
        }
    }

    fn increment(&self) -> Self {
        Self::from_value(self.value() + 1)
    }

    // Gets all the adjacent locations to check if we want to travel in this direction from a
    // current location
    fn locs_to_check(&self, current: Loc) -> HashSet<Loc> {
        let (x, y) = current;

        match self {
            Self::North => HashSet::from([(x - 1, y - 1), (x, y - 1), (x + 1, y - 1)]),
            Self::South => HashSet::from([(x - 1, y + 1), (x, y + 1), (x + 1, y + 1)]),
            Self::East => HashSet::from([(x + 1, y - 1), (x + 1, y), (x + 1, y + 1)]),
            Self::West => HashSet::from([(x - 1, y - 1), (x - 1, y), (x - 1, y + 1)]),
        }
    }

    // Get the location directly adjacent to a given current location, in this direction
    fn get_adjacent_loc(&self, current: Loc) -> Loc {
        let (x, y) = current;
        match self {
            Self::North => (x, y - 1),
            Self::South => (x, y + 1),
            Self::East => (x + 1, y),
            Self::West => (x - 1, y)
        }
    }
}

#[derive(Debug, PartialEq)]
struct ElfTroop {
    // Locations an Elf is standing at the moment
    elves: HashSet<Loc>,

    // Which direction elves will look to move to next as the first candidate
    dir_index: usize,
}

impl ElfTroop {
    fn import(input: &Vec<String>) -> Self {
        let mut elves: HashSet<Loc> = HashSet::new();

        // Extract all coordinates where we find a '#'
        input
            .iter()
            .enumerate()
            .for_each(|(y, l)| {
                l.chars().enumerate().for_each(|(x, c)| {
                   if c == '#' {
                       elves.insert((x as isize, y as isize));
                   }
                });
            });

        ElfTroop { elves, dir_index: 0 }
    }

    // Based on the logic described in the challenge, proposes a location for an elf to go to next.
    // - If `current` is fully surrounded by open squares, we're happy where we are and need to stay
    //   so `None` is returned
    // - Otherwise we look first in start_dir to see if we can go there. If not, we check the next
    //   direction in the list. If a viable direction to go is found, returns Some(loc) with the
    //   adjacent location in that direction
    //   If no viable direction is found, returns `None`; we'll stay put.
    fn propose_location(&self, current: Loc, start_dir: Direction) -> Option<Loc> {
        if !get_surrounding_locs(current).iter().any(|l| self.elves.contains(l)) {
            return None;
        }

        for i in start_dir.value()..(start_dir.value() + 4) {
            let dir = Direction::from_value(i);
            let locs_to_check = dir.locs_to_check(current);

            if !locs_to_check.iter().any(|l| self.elves.contains(l)) {
                return Some(dir.get_adjacent_loc(current));
            }
        }

        None
    }

    // Figure out where each elf should go this round, and resolve which ones actually go through
    // with it. Returns how many elves moved from their spot before this round.
    fn process_round(&mut self) -> usize {
        let direction = Direction::from_value(self.dir_index);

        // Iterates over each elf and matches it up with a target location. If the elf is not to
        // move (see `propose_location`), it is left out of the set of moves collected here
        let target_current = self.elves
            .iter()
            .filter_map(|current| self.propose_location(*current, direction).map(|t| (t, *current)))
            .collect::<Vec<(Loc, Loc)>>();

        // Collect targets seen so far
        let mut targets: HashSet<Loc> = HashSet::new();

        // Add any targets we've seen once more so we can remove them later.
        let mut targets_to_remove: HashSet<Loc> = HashSet::new();

        target_current
            .iter()
            .for_each(|(t, _)| {
                // target already encountered? add it to those to remove
                if targets.contains(t) {
                    targets_to_remove.insert(*t);
                } else {
                    targets.insert(*t);
                }
            });

        // Flip to current->target while removing pairs that are in targets_to_remove
        let proposed_moves = target_current
            .iter()
            .filter_map(|(t, c)| {
                if targets_to_remove.contains(t) {
                    None
                } else {
                    Some((*c, *t))
                }
            })
            .collect::<HashMap<Loc, Loc>>();

        // Find elves that don't have a move queued up
        let elves_that_stay = self.elves
            .iter()
            .filter(|e| {
                !proposed_moves.contains_key(*e)
            })
            .map(|i| *i)
            .collect::<HashSet<Loc>>();

        // Funny enough, this is kind of all I needed for part 2, everything else was already as-is.
        let move_count = proposed_moves.len();

        // Combine locations of staying elves with all the move targets
        self.elves = elves_that_stay
            .union(&proposed_moves.values().map(|i| *i).collect::<HashSet<Loc>>())
            .map(|i| *i)
            .collect();

        // For next round, move to next starting directions
        self.dir_index = direction.increment().value();

        move_count
    }

    fn process_until_done(&mut self) -> usize {
        let mut count = 0;

        loop {
            count += 1;
            if self.process_round() == 0  { return count }
        }
    }

    // Surface area is calculated by taking the max and min X/Y coordinates, getting their
    // differences + 1 (to include boundary), and multiplying them together.
    fn surface_area(&self) -> isize {
        // I'm pretty pleased with this four-fold ... fold; only have to run through the elves
        // once to get 4 extremes.
        let (min_x, max_x, min_y, max_y) = self.elves
            .iter()
            .fold((isize::MAX, isize::MIN, isize::MAX, isize::MIN), |acc, (x, y)| {
                let (a_min_x, a_max_x, a_min_y, a_max_y) = acc;
                (min(a_min_x, *x), max(a_max_x, *x), min(a_min_y, *y), max(a_max_y, *y))
            });

        let w = max_x - min_x + 1;
        let h = max_y - min_y + 1;

        return w * h;
    }

    fn count_empty_squares(&self) -> isize {
        self.surface_area() - self.elves.len() as isize
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    fn get_input_small() -> Vec<String> {
        vec![
            ".....".to_string(),
            "..##.".to_string(),
            "..#..".to_string(),
            ".....".to_string(),
            "..##.".to_string(),
            ".....".to_string(),
        ]
    }

    fn get_input_larger() -> Vec<String> {
        vec![
            "....#..".to_string(),
            "..###.#".to_string(),
            "#...#.#".to_string(),
            ".#...##".to_string(),
            "#.###..".to_string(),
            "##.#.##".to_string(),
            ".#..#..".to_string(),
        ]
    }

    #[test]
    fn test_elftroop_import() {
        assert_eq!(
            ElfTroop::import(&get_input_small()),
            ElfTroop {
                elves: HashSet::from([
                    (2, 1),
                    (3, 1),
                    (2, 2),
                    (2, 4),
                    (3, 4),
                ]),

                dir_index: 0,
            },
        );
    }

    #[test]
    fn test_elftroop_round() {
        let mut troop = ElfTroop::import(&get_input_small());

        troop.process_round();
        assert_eq!(
            troop,
            ElfTroop {
                elves: HashSet::from([
                    (2, 0),
                    (3, 0),
                    (2, 2),
                    (3, 3),
                    (2, 4)
                ]),

                dir_index: 1,
            }
        );

        troop.process_round();
        assert_eq!(
            troop,
            ElfTroop {
                elves: HashSet::from([
                    (2, 1),
                    (3, 1),
                    (1, 2),
                    (4, 3),
                    (2, 5),
                ]),

                dir_index: 2,
            }
        );

        troop.process_round();
        assert_eq!(
            troop,
            ElfTroop {
                elves: HashSet::from([
                    (2, 0),
                    (4, 1),
                    (0, 2),
                    (4, 3),
                    (2, 5),
                ]),

                dir_index: 3,
            }
        );
    }

    #[test]
    fn test_empty_tiles() {
        let mut troop = ElfTroop::import(&get_input_larger());
        (0..10).for_each(|_| { troop.process_round(); });
        assert_eq!(troop.surface_area(), 12 * 11);
        assert_eq!(troop.count_empty_squares(), 110);
    }

    #[test]
    fn test_rounds_needed() {
        let mut troop = ElfTroop::import(&get_input_larger());
        let rounds = troop.process_until_done();
        assert_eq!(rounds, 20);
    }
}
