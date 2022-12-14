use std::io;
use std::io::BufRead;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    // Pretend there's no floor
    let mut cave = Cave::process_scan((500, 0), false, &input);

    loop {
        // drop_sand returns None if there's nowhere for the sand to settle
        if cave.drop_sand().is_none() {
            break;
        }
    }

    format!("{}", cave.count_sand())
}

fn process_part_two(input: Vec<String>) -> String {
    // Oh dang there _is_ a floor
    let mut cave = Cave::process_scan((500, 0), true, &input);

    loop {
        // Since here we have an infinite floor, there always is somewhere for
        // sand to settle. So we stop when the sand settles at its origin point.
        if cave.drop_sand().unwrap() == cave.sand_origin {
            break;
        }
    }

    format!("{}", cave.count_sand())
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

type Loc = (usize, usize);

#[derive(Debug, PartialEq, Eq, Hash)]
enum Material { Rock, Sand }

#[derive(Debug, PartialEq)]
struct Cave {
    // Sand falls from here
    sand_origin: Loc,

    // Lowest piece of rock found in the scan
    rock_bottom: usize,

    // If we're dealing with a model of a cave that has a floor,
    // this is the level it's at (rock_bottom+2)
    floor: Option<usize>,

    // Keep track of things.
    // This contains each location that has something we care about;
    // rock, and sand.
    stuff: HashMap<Loc, Material>,
}

impl Cave {
    fn process_scan(sand_origin: Loc, has_floor: bool, input: &Vec<String>) -> Self {
        let stuff: HashMap<Loc, Material> = input
            .iter()
            // Each line produces a set of rock locations,
            // which we aggregate here into a larger set
            .flat_map(|l| Self::get_rocks_from_scan_line(l))
            // Pair all those rock locations with the material
            .map(|loc| (loc, Material::Rock))
            // And produce the final map of loc => material
            .collect();

        // Locate the furthest down location with rock
        let rock_bottom = stuff
            .iter()
            .filter(|(_, m)| **m == Material::Rock)
            .fold(0, |acc, ((_, y), _)| {
                if acc > *y { acc } else { *y }
            });

        // If this cave model has a floor, derive it from rock_bottom
        let floor = if has_floor { Some(rock_bottom + 2) } else { None };

        Self {
            sand_origin,
            rock_bottom,
            floor,
            stuff,
        }
    }

    fn get_rocks_from_scan_line(input: &str) -> HashSet<Loc> {
        let locs: Vec<Loc> = input
            // Split the line into locations
            .split(" -> ")
            .map(|raw_loc| {
                // Convert a raw str location into a Loc
                raw_loc
                    .split_once(",")
                    .map(|(x_raw, y_raw)| {
                        (
                            usize::from_str_radix(x_raw, 10).unwrap(),
                            usize::from_str_radix(y_raw, 10).unwrap()
                        )
                    })
                    .unwrap()
            })
            // And collect that into a list of locations
            .collect();

        // This bit is kinda complex. Given the description, all segments
        // are straight lines, so either their X or their Y coords will match.
        // So from this, we technically iterate over every set of 2 locs as
        // we move through the list (note the range stops 1 off the end)
        (0..(locs.len() - 1))
            // Map index to a pair of locations
            .map(|i| (locs[i], locs[i+1]))
            .flat_map(|((l1_x, l1_y),(l2_x, l2_y))| {
                // For this pair of locations, produce a set of distinct locations
                // from the first up to and including the last
                // Since it's flat_map, the contents of set of locations are
                // added to that from previous pairs in the list, and any dupes
                // removed as we go
                if l1_x != l2_x {
                    // Implied that y components are equal
                    let x_range = if l1_x < l2_x { l1_x..=l2_x } else { l2_x..=l1_x };
                    x_range.map(|x| (x, l1_y)).collect::<HashSet<Loc>>()
                } else {
                    // x components are equal, implied y components are not equal
                    let y_range = if l1_y < l2_y { l1_y..=l2_y } else { l2_y..=l1_y };
                    y_range.map(|y| (l1_x, y)).collect::<HashSet<Loc>>()
                }
            })
            // Collect it into a single set of locations and return that,
            // we now have a comprehensive list of all the locations with rock
            // described by this scanner line.
            .collect()
    }


    // Check whether a given location has anything in it at the moment.
    // If this cave has a floor, this will return true for any location where
    // the y component >= self.floor
    fn is_vacant(&self, loc: &Loc) -> bool {
        let (_, y) = *loc;

        if let Some(floor_level) = self.floor {
            if y >= floor_level {
                return false;
            }
        }

        self.stuff.get(loc).is_none()
    }

    // Tries the 3 possible locations underneath the given location
    // and returns the first one that's vacant, or None if none are.
    // Order: directly down, down left, down right
    fn get_available_loc_under(&self, loc: Loc) -> Option<Loc> {
        let (origin_x, origin_y) = loc;

        let locs_to_try = [
            (origin_x, origin_y + 1),
            (origin_x - 1, origin_y + 1),
            (origin_x + 1, origin_y +1)
        ];

        locs_to_try
            .iter()
            .find(|l| self.is_vacant(l))
            .cloned()
    }

    // Drops one unit of sand from self.sand_origin
    // Possibly mutates self.stuff by adding the resting location mapped to
    // sand.
    // Returns the resting location or None if it dropped into the void.
    fn drop_sand(&mut self) -> Option<Loc> {
        let mut current_loc = self.sand_origin;

        loop {
            if let Some((x,y)) = self.get_available_loc_under(current_loc) {
                if y > self.rock_bottom && self.floor.is_none() {
                    // Fell beyond the lowest rock, not settling anywhere
                    return None;
                }

                current_loc = (x,y);
            } else {
                break;
            }
        }

        // current loc then is sand to be added to stuff.
        self.stuff.insert(current_loc.clone(), Material::Sand);
        Some(current_loc)
    }

    // Counts the number sand items that have settled in the cave
    fn count_sand(&self) -> usize {
        self.stuff.iter().filter(|(_, m)| **m == Material::Sand).count()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_cave_get_rocks_from_scan_line() {

        assert_eq!(
            Cave::get_rocks_from_scan_line("498,4 -> 498,6 -> 496,6"),
            HashSet::from([
                (498, 4), (498, 5), (498, 6),
                (497, 6), (496,6)
            ]),
        );

        assert_eq!(
            Cave::get_rocks_from_scan_line("503,4 -> 502,4 -> 502,9 -> 494,9"),
            HashSet::from([
                (503, 4), (502, 4),
                (502, 5), (502, 6), (502, 7), (502, 8), (502, 9),
                (501, 9), (500, 9), (499, 9), (498, 9), (497, 9), (496, 9), (495, 9), (494, 9),
            ]),
        );
    }

    #[test]
    fn test_cave_process_scan() {
        let input = vec![
            "498,4 -> 498,6 -> 496,6".to_string(),
            "503,4 -> 502,4 -> 502,9 -> 494,9".to_string(),
        ];

        assert_eq!(
            Cave::process_scan((500, 0), false, &input),
            Cave {
                sand_origin: (500, 0),
                rock_bottom: 9,
                floor: None,
                stuff: HashMap::from([
                    ((498, 4), Material::Rock),
                    ((498, 5), Material::Rock),
                    ((498, 6), Material::Rock),
                    ((497, 6), Material::Rock),
                    ((496, 6), Material::Rock),
                    ((503, 4), Material::Rock),
                    ((502, 4), Material::Rock),
                    ((502, 5), Material::Rock),
                    ((502, 6), Material::Rock),
                    ((502, 7), Material::Rock),
                    ((502, 8), Material::Rock),
                    ((502, 9), Material::Rock),
                    ((501, 9), Material::Rock),
                    ((500, 9), Material::Rock),
                    ((499, 9), Material::Rock),
                    ((498, 9), Material::Rock),
                    ((497, 9), Material::Rock),
                    ((496, 9), Material::Rock),
                    ((495, 9), Material::Rock),
                    ((494, 9), Material::Rock),
                ]),
            }
        );

        assert_eq!(
            Cave::process_scan((500, 0), true, &input),
            Cave {
                sand_origin: (500, 0),
                rock_bottom: 9,
                floor: Some(11),
                stuff: HashMap::from([
                    ((498, 4), Material::Rock),
                    ((498, 5), Material::Rock),
                    ((498, 6), Material::Rock),
                    ((497, 6), Material::Rock),
                    ((496, 6), Material::Rock),
                    ((503, 4), Material::Rock),
                    ((502, 4), Material::Rock),
                    ((502, 5), Material::Rock),
                    ((502, 6), Material::Rock),
                    ((502, 7), Material::Rock),
                    ((502, 8), Material::Rock),
                    ((502, 9), Material::Rock),
                    ((501, 9), Material::Rock),
                    ((500, 9), Material::Rock),
                    ((499, 9), Material::Rock),
                    ((498, 9), Material::Rock),
                    ((497, 9), Material::Rock),
                    ((496, 9), Material::Rock),
                    ((495, 9), Material::Rock),
                    ((494, 9), Material::Rock),
                ]),
            }
        );
    }

    #[test]
    fn test_drop_sand_without_floor() {
        let input = vec![
            "498,4 -> 498,6 -> 496,6".to_string(),
            "503,4 -> 502,4 -> 502,9 -> 494,9".to_string(),
        ];

        let mut cave = Cave::process_scan((500, 0), false, &input);

        assert_eq!(cave.drop_sand().unwrap(), (500, 8));
        assert_eq!(cave.drop_sand().unwrap(), (499, 8));
        assert_eq!(cave.drop_sand().unwrap(), (501, 8));
        assert_eq!(cave.drop_sand().unwrap(), (500, 7));
        assert_eq!(cave.drop_sand().unwrap(), (498, 8));

        assert_eq!(cave.count_sand(), 5);

        // Drop sand until it no longer settles anywhere
        loop {
            if cave.drop_sand().is_none() { break; }
        }

        assert_eq!(cave.count_sand(), 24);
    }

    #[test]
    fn test_drop_sand_with_floor() {
        let input = vec![
            "498,4 -> 498,6 -> 496,6".to_string(),
            "503,4 -> 502,4 -> 502,9 -> 494,9".to_string(),
        ];

        let mut cave = Cave::process_scan((500, 0), true, &input);

        // Drop sand until we clog the origin
        loop {
            if cave.drop_sand().unwrap() == cave.sand_origin {
                break;
            }
        }

        assert_eq!(cave.count_sand(), 93);
    }
}
