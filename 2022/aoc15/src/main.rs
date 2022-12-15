use std::io;
use std::io::BufRead;
use std::ops::RangeInclusive;
use std::collections::HashSet;
use std::cmp::{min,max};

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let sensors = input
        .iter()
        .map(|l| parse_sensor_line(l))
        .collect::<Vec<Sensor>>();

    let y = 2000000;

    let ranges = sensors
        .iter()
        .filter(|s| s.intersects_y(y))
        .map(|s| s.coverage_at_y(y))
        .collect::<Vec<RangeInclusive<isize>>>();

    let simplified_ranges = simplify_ranges(ranges);

    let beacons_in_ranges = sensors
        .iter()
        .filter(|(_, b)| {
            let (bx, by) = b;
            if *by != y {
                false
            } else {
                simplified_ranges.iter().any(|r| r.contains(bx))
            }
        })
        .map(|(_, (bx, by))| *bx)
        .collect::<HashSet<isize>>();

    let coverage = simplified_ranges
        .iter()
        .fold(0, |acc, r| acc + r.end() - r.start() + 1) - beacons_in_ranges.len() as isize;

    format!("{coverage}")
}

fn process_part_two(input: Vec<String>) -> String {
    let sensors = input
        .iter()
        .map(|l| parse_sensor_line(l))
        .collect::<Vec<Sensor>>();

    let limit = 4000000;

    // So this feels like a naive approach, but I'm betting on my implementation
    // of part one being efficient enough that I can get away with running it up to
    // 4 million times.
    // Turns out it was a decent bet, because it found the solution for my input
    // after about 26 seconds,
    // somewhere between 3.3M and 3.4M (~82.5% into the search space), running on a 9 year old
    // machine on a single core.
    //
    // Heh, a cool exercise could be to split up this search across more CPU cores.
    let target_y = (0..=limit).find(|y| {
        // Progress indicator
        if y % 100000 == 0 {
            println!("{y}/{limit}...")
        }

        let ranges = sensors
            .iter()
            .filter(|s| s.intersects_y(*y))
            .map(|s| s.coverage_at_y(*y))
            .collect::<Vec<RangeInclusive<isize>>>();

        let simplified_ranges = simplify_ranges(ranges);

        ! simplified_ranges.iter().any(|r| r.contains(&0) && r.contains(&limit))
    }).unwrap();

    // What's one more time?
    let mut simplified_ranges = simplify_ranges(
        sensors
            .iter()
            .filter(|s| s.intersects_y(target_y))
            .map(|s| s.coverage_at_y(target_y))
            .collect::<Vec<RangeInclusive<isize>>>()
    ).iter().map(|r| *r.start()..=*r.end()).collect::<Vec<RangeInclusive<isize>>>();

    simplified_ranges.sort_by(|ar, br| ar.start().cmp(br.start()));

    let target_x = simplified_ranges[0].end() + 1;

    let tuning_freq = target_x * limit +  target_y;

    format!("Tuning freq: {tuning_freq}")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

type Loc = (isize, isize);
type Sensor = (Loc, Loc);

trait SensorCalculations {
    fn beacon_distance(&self) -> isize;
    fn coverage_at_y(&self, y: isize) -> RangeInclusive<isize>;
    fn intersects_y(&self, y: isize) -> bool;
}

impl SensorCalculations for Sensor {
    fn beacon_distance(&self) -> isize {
        let ((sx, sy), (bx, by)) = self;
        (sx - bx).abs() + (sy - by).abs()
    }

    fn coverage_at_y(&self, y: isize) -> RangeInclusive<isize> {
        let ((sx, sy), _) = self;
        let half_width = self.beacon_distance() - (y - sy).abs();

        (sx - half_width)..=(sx + half_width)
    }

    fn intersects_y(&self, y: isize) -> bool {
        let ((_, sy), _) = self;
        (y - sy).abs() <= self.beacon_distance()
    }
}

/// Simplifies a set of ranges, combining overlapping ranges into one larger
/// range, producing a set of the minimum number of ranges required to represent
/// the input ranges.
///
/// There probably is some very common algorithm for this, but I don't know it,
/// and I'm pretty proud of what I came up with here.
fn simplify_ranges(in_ranges: Vec<RangeInclusive<isize>>) -> HashSet<RangeInclusive<isize>> {
    // Compare ranges to tuples, easier to work with in this function.
    let mut ranges: Vec<(isize, isize)> = in_ranges
        .iter()
        .map(|r| (*r.start(), *r.end()))
        .collect();

    let mut simplified_ranges: HashSet<RangeInclusive<isize>> = HashSet::new();

    loop {
        // Step : sort our ranges by start. This ensures overlapping ranges
        // will follow one another
        ranges.sort_by(|(a_s, _), (bs, _)| { a_s.cmp(bs) });

        let combined = ranges
            .iter()
            // Starting with the first range, grow the range until we run out of
            // overlapping ranges
            // Hm, this could be improved by not checking any further ranges
            // after first one that doesn't overlap.
            .fold(ranges[0], |acc, r| {
                let (a_s, ae) = acc;
                let (rs, re) = *r;

                if (rs <= a_s && a_s <= re) || (rs <= ae && ae <= re) {
                    (min(a_s, rs), max(ae, re))
                } else {
                    // TODO in this case we should essentially exit from the
                    // fold.
                    // Perhaps it can be achieved with `scan`?
                    (a_s, ae)
                }
            });

        // Remove any ranges that are overlapped by the combined one just found
        ranges = ranges
            .iter()
            .filter(|(rs, _) | {
                // Only keep ranges not overlapping with combined range
                let (cs, ce) = combined;
                ce < *rs || cs > *rs
            })
            .map(|r| *r)
            .collect();

        let (cs, ce) = combined;
        simplified_ranges.insert(cs..=ce);

        // Once we run out of ranges, we're done.
        if ranges.len() == 0 {
            break;
        }
    }

    simplified_ranges
}

fn parse_sensor_line(input: &str) -> Sensor {
    // We assume all input matches what we're after, hence the unwraps all over
    let split: Vec<&str> = input.split(" ").collect();

    // Disgusting parsing routine time
    // Split by spaces, we're interested in the following number items
    let values = vec![2, 3, 8, 9]
        .iter()
        .map(|i| {
            isize::from_str_radix(
                split[*i]
                    // Then for each of these items, we abuse split() further
                    // to remove cruft we don't want and keep only the bit that can be parsed
                    // as an isize.
                    .split("=").nth(1).unwrap() // Applies to all
                    .split(",").nth(0).unwrap()  // Applies to sensor x, beacon x
                    .split(":").nth(0).unwrap(), // Only applies to sensor y
                10
            ).unwrap()
        })
        .collect::<Vec<isize>>();

    ((values[0], values[1]), (values[2], values[3]))
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_sensor_calculates_beacon_distance() {
        assert_eq!(((2, 18), (-2, 15)).beacon_distance(), 7);
        assert_eq!(((9, 16), (10, 16)).beacon_distance(), 1);
    }

    #[test]
    fn test_sensor_coverage_at_y() {
        let s = ((8, 7), (2, 10));

        assert_eq!(s.coverage_at_y(-3), 9..=7);
        assert_eq!(s.coverage_at_y(-2), 8..=8);
        assert_eq!(s.coverage_at_y(-1), 7..=9);
        assert_eq!(s.coverage_at_y(0), 6..=10);
        assert_eq!(s.coverage_at_y(1), 5..=11);
        assert_eq!(s.coverage_at_y(2), 4..=12);
        assert_eq!(s.coverage_at_y(3), 3..=13);
        assert_eq!(s.coverage_at_y(4), 2..=14);
        assert_eq!(s.coverage_at_y(5), 1..=15);
        assert_eq!(s.coverage_at_y(6), 0..=16);
        assert_eq!(s.coverage_at_y(7), -1..=17);
        assert_eq!(s.coverage_at_y(8), 0..=16);
        assert_eq!(s.coverage_at_y(9), 1..=15);
        assert_eq!(s.coverage_at_y(10), 2..=14);
        assert_eq!(s.coverage_at_y(11), 3..=13);
        assert_eq!(s.coverage_at_y(12), 4..=12);
        assert_eq!(s.coverage_at_y(13), 5..=11);
        assert_eq!(s.coverage_at_y(14), 6..=10);
        assert_eq!(s.coverage_at_y(15), 7..=9);
        assert_eq!(s.coverage_at_y(16), 8..=8);
        assert_eq!(s.coverage_at_y(17), 9..=7);
    }

    #[test]
    fn test_sensor_intersects_y() {
        let s = ((8, 7), (2, 10));

        assert!( ! s.intersects_y(-3));

        for i in -2..=16 {
            println!("intersects {i}: {}", s.intersects_y(i));
            assert!(s.intersects_y(i));
        }

        assert!( ! s.intersects_y(17));
    }

    #[test]
    fn test_parse_sensor_line() {
        assert_eq!(
            parse_sensor_line(
                "Sensor at x=2300471, y=2016823: closest beacon is at x=2687171, y=2822745",
            ),
            ((2300471, 2016823), (2687171, 2822745)),
        );

        assert_eq!(
            parse_sensor_line(
                "Sensor at x=-471, y=2016823: closest beacon is at x=2687171, y=2822745",
            ),
            ((-471, 2016823), (2687171, 2822745)),
        );
    }

    #[test]
    fn test_simplify_ranges() {
        assert_eq!(
            simplify_ranges(vec![
                0..=4,
                2..=6,
            ]),
            HashSet::from([
                0..=6
            ])
        );

        assert_eq!(
            simplify_ranges(vec![
                2..=6,
                0..=4,
            ]),
            HashSet::from([
                0..=6
            ]),
        );

        assert_eq!(
            simplify_ranges(vec![
                8..=10,
                10..=13,

                2..=6,
                0..=4,
            ]),
            HashSet::from([
                0..=6,
                8..=13,
            ]),
        );

        assert_eq!(
            simplify_ranges(vec![
               4..=6, // contained in 3..=25, removed
               8..=10, // contained in 3..=25, removed

               -1..=2, // Overlaps nothing
               3..=25, // Contains: 4..=6, 8..=10
               22..=30, // overlaps 3..=25, so extends to 3..=30
               42..=69, // overlaps nothing
            ]),
            HashSet::from([
                -1..=2,
                3..=30,
                42..=69,
            ]),
        );
    }
}
