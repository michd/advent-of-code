use std::cmp::{max, min};
use std::collections::{HashSet, VecDeque};
use std::{cmp, io};
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let cubes = input.iter().map(|l| parse_cube_loc(l)).collect::<Vec<Loc>>();
    let output = get_outer_surface_area(&cubes);
    format!("{}", output)
}

fn process_part_two(input: Vec<String>) -> String {
    let cubes = input.iter().map(|l| parse_cube_loc(l)).collect::<Vec<Loc>>();
    let output = get_accurate_outer_surface_area(&cubes);
    format!("{}", output)
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

type Loc = (usize, usize, usize);
type Surface = (Loc, Loc);
type Bounds = ((usize, usize),(usize, usize), (usize, usize));

fn parse_cube_loc(input: &str) -> Loc {
    let parsed = input
        .split(",")
        .map(|nr| usize::from_str_radix(nr, 10).unwrap() + 10)
        .collect::<Vec<usize>>();

    (parsed[0], parsed[1], parsed[2])
}

// Gets a list of all surfaces of a 1x1x1 cube at given location.
// given location is considered a corner coordinate.
fn get_surfaces(loc: Loc) -> Vec<Surface> {
    let (x, y, z) = loc;

    vec![
        ((x, y, z), (x + 1, y + 1, z)),
        ((x, y, z), (x + 1, y, z + 1)),
        ((x, y, z), (x, y + 1, z + 1)),
        ((x, y, z + 1), (x + 1, y + 1, z + 1)),
        ((x, y + 1, z), (x + 1, y + 1, z + 1)),
        ((x + 1, y, z), (x + 1, y + 1, z + 1)),
    ]
}

fn count_distinct_sides(cube_locations: &Vec<Loc>) -> usize {
    cube_locations
        .iter()
        .flat_map(|l| get_surfaces(*l))
        .collect::<HashSet<Surface>>()
        .len()
}

fn get_outer_surface_area(cube_locations: &Vec<Loc>) -> usize {
    let total_sides = cube_locations.len() * 6;
    let duped_sides = total_sides - count_distinct_sides(cube_locations);
    total_sides - (duped_sides * 2)
}

fn get_accurate_outer_surface_area(cube_locations: &Vec<Loc>) -> usize {
    let total_sides = cube_locations.len() * 6;
    let duped_sides = total_sides - count_distinct_sides(cube_locations);
    total_sides - (duped_sides * 2) - count_enclosed_sides(&cube_locations)
}

// Given a surface, gets the cubes on both sides of it
fn get_surface_adjacent_cubes(surface: Surface) -> Vec<Loc> {
    let ((x1, y1, z1), (_x2, y2, z2)) = surface;

    if z1 == z2 {
        vec![
            (x1, y1, z1),
            (x1, y1, z1 - 1)
        ]
    } else if y1 == y2 {
        vec![
            (x1, y1, z1),
            (x1, y1 - 1, z1)
        ]
    } else {
        vec![
            (x1, y1, z1),
            (x1 - 1, y1, z1)
        ]
    }
}

// Get all 6 cubes that share a surface with the given cube
fn get_cube_adjacent_cubes(cube: Loc) -> Vec<Loc> {
    let (x, y, z) = cube;

    vec![
        (x, y, z + 1),
        (x, y, z - 1),
        (x, y + 1, z),
        (x, y - 1, z),
        (x + 1, y, z),
        (x - 1, y, z),
    ]
}

// Gets the outer limits of our lava, on each axis
fn get_bounds(cube_locations: &Vec<Loc>) -> Bounds {
    let min_x = cube_locations.iter().fold(usize::MAX, |acc, (x,_,_)| min(acc, *x));
    let max_x = cube_locations.iter().fold(0usize, |acc, (x, _, _)| max(acc, *x));
    let min_y = cube_locations.iter().fold(usize::MAX, |acc, (_,y,_)| min(acc, *y));
    let max_y = cube_locations.iter().fold(0usize, |acc, (_, y, _)| max(acc, *y));
    let min_z = cube_locations.iter().fold(usize::MAX, |acc, (_,_,z)| min(acc, *z));
    let max_z = cube_locations.iter().fold(0usize, |acc, (_, _, z)| max(acc, *z));

    ((min_x, max_x), (min_y, max_y), (min_z, max_z))
}

fn is_outside_bounds(loc: &Loc, bounds: &Bounds) -> bool {
    let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = *bounds;
    let (x, y, z) = *loc;

    x < min_x || x > max_x || y < min_y || y > max_y || z < min_z || z > max_z
}

// Part 2.
// From part 1, we know how many sides are exposed to air.
// Now we want to know how many of those sides are fully enclosed in the lava,
// with no path to the outside.
//
// "Outside" is defined as "connects to a location that is outside of the boundaries of our scan"
// Summarising the procedure: We collect all exposed sides first.
// Then we derive the coordinates adjacent to those sides that we know not to contain lava.
// That is, they are not contained in cube_locations.
// We then look around this location for all the adjacent cubes and visit each one.
// Before doing so, we filter out any locations that we have already visited, or are already marked.
// Using something like breadth-first-search, we visit each such eligible neighbour and evaluate:
//
// is this location outside bounds? If yes, then everything we have visited on this run is connected
// to the outside air, and thus not enclosed. We mark all the locations visited, and remove all
// locations from those left to explore.
//
// If we keep following one of these paths and find we have nowhere left to go (we never reach
// either outside or a previously marked outside location), then we know everything in this path
// is enclosed, so we mark it as such and remove it from locations left to explore
//
// Once we run out of locations to explore, we have a list of enclosed air locations. We gather the
// sides of these cubes and correlate them with our naively exposed squares, keeping only those
// present in both (getting rid of the side of any air-surrounded air locations).
// Then we count those, and that's our number of enclosed sides.
fn count_enclosed_sides(cube_locations: &Vec<Loc>) -> usize {
    let bounds = get_bounds(&cube_locations);

    let mut naive_exposed_sides: HashSet<Surface> = HashSet::new();

    // Gather all surfaces, removing any that occur more than once.
    cube_locations
        .iter()
        .flat_map(|l| get_surfaces(*l))
        .for_each(|s| {
            if naive_exposed_sides.contains(&s) {
                naive_exposed_sides.remove(&s);
            } else {
                naive_exposed_sides.insert(s);
            }
        });

    // Gather all exposed-adjacent cubes that aren't lava
    let mut cubes_to_explore = naive_exposed_sides
        .iter()
        .flat_map(|s| get_surface_adjacent_cubes(*s))
        .filter(|c| !cube_locations.contains(c))
        .collect::<HashSet<Loc>>();

    // Here we'll mark known locations for being connected to outside air or not
    let mut outside_air: HashSet<Loc> = HashSet::new();
    let mut enclosed_cubes: HashSet<Loc> = HashSet::new();

    loop {
        // This loop is for every potential path. Could be there are multiple fully enclosed pockets.
        if cubes_to_explore.is_empty() {
            break;
        }

        let mut visited = HashSet::<Loc>::new();

        let mut queue = VecDeque::<Loc>::new();
        let first_loc = *cubes_to_explore.iter().nth(0).unwrap();
        cubes_to_explore.remove(&first_loc);
        queue.push_back(first_loc);

        loop {
            if queue.is_empty() { break; }
            let loc = queue.pop_front().unwrap();
            if visited.contains(&loc) { continue; }

            if outside_air.contains(&loc) || is_outside_bounds(&loc, &bounds) {
                // If we're visiting a location that's outside boundaries we know that everything
                // in our path is an exposed cube, part of outside air
                // Also if we've previously marked this location as being outside air, we
                // know that everything in our path is part of outside.
                visited.iter().for_each(|l| { outside_air.insert(*l); });
                visited.clear();
                outside_air.insert(loc);

                // Everything in queue is part of the same accessible space so should empty it into
                // outside_air, too.
                // I guess technically an optimization(?) might be to follow each search to its
                // conclusion hear instead of cutting off early. Not sure it'd be particular faster.
                // Here we stop the current search the moment the shortest path to outside air is
                // reached.
                queue.iter().for_each(|l| { outside_air.insert(*l); });
                queue.clear();
                break;
            }

            // Location was not outside air, carry on, looking for further adjacent locations
            // to try; add unexplored locations adjacent to this location to the queue.
            get_cube_adjacent_cubes(loc)
                .iter()
                .filter(|l| {
                    !visited.contains(l) &&
                        !cube_locations.contains(l) &&
                        !enclosed_cubes.contains(l)
                })
                .for_each(|l| { queue.push_back(*l); });

            // And finally, mark this location as visited so we don't visit it again.
            visited.insert(loc);
        }

        // If there's visited cubes left here, it means we didn't
        // find a path to outside. All visited cubes are part of an enclosed space.
        visited
            .iter()
            .for_each(|l| {
                cubes_to_explore.remove(l);
                enclosed_cubes.insert(*l);
            });
    }

    // We've run out of locations to explore, so we're ready to make our tally.

    // Surfaces of enclosed cubes that are exposed
    enclosed_cubes
        .iter()
        .flat_map(|l| get_surfaces(*l))
        .filter(|s| {
            // Only count those that we knew were exposed from our initial scan
            naive_exposed_sides.contains(s)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn testing_works() {
        assert_eq!(2,  1 + 1);
    }

    #[test]
    fn test_parse_cube_loc() {
        assert_eq!(parse_cube_loc("1,2,5"), (1,2,5));
    }

    #[test]
    fn test_parse_cubes() {
        let cubes = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5"#.lines().map(parse_cube_loc).collect::<Vec<Loc>>();

        assert_eq!(cubes, vec![
            (2,2,2),
            (1,2,2),
            (3,2,2),
            (2,1,2),
            (2,3,2),
            (2,2,1),
            (2,2,3),
            (2,2,4),
            (2,2,6),
            (1,2,5),
            (3,2,5),
            (2,1,5),
            (2,3,5),
        ])
    }

    #[test]
    fn test_get_outer_surface() {
        let cubes = vec![
            (2,2,2),
            (1,2,2),
            (3,2,2),
            (2,1,2),
            (2,3,2),
            (2,2,1),
            (2,2,3),
            (2,2,4),
            (2,2,6),
            (1,2,5),
            (3,2,5),
            (2,1,5),
            (2,3,5),
        ];

        assert_eq!(get_outer_surface_area(&cubes), 64);
    }

    #[test]
    fn test_get_accurate_outer_surface() {
        let cubes = vec![
            (2,2,2),
            (1,2,2),
            (3,2,2),
            (2,1,2),
            (2,3,2),
            (2,2,1),
            (2,2,3),
            (2,2,4),
            (2,2,6),
            (1,2,5),
            (3,2,5),
            (2,1,5),
            (2,3,5),
        ];

        assert_eq!(get_accurate_outer_surface_area(&cubes), 58);
    }
}