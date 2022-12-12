use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::collections::VecDeque;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let graph = Graph::parse(&input).unwrap();
    let shortest_path_steps = graph
        .find_shortest_path(
            &graph.begin,
            false, // reverse
            |n| n == &graph.end // at destination?
        )
        .unwrap();

    format!("{shortest_path_steps}")
}

fn process_part_two(input: Vec<String>) -> String {
    let graph = Graph::parse(&input).unwrap();

    // Here we don't know the starting point, so we're path-finding
    // in reverse, starting at the known end.
    let shortest_path_steps = graph
        .find_shortest_path(
            &graph.end,
            true, // reverse
            |n| n.altitude == 1 // at destination?
        )
        .unwrap();

    format!("{shortest_path_steps}")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    y: usize,
    x: usize,
    altitude: usize
}

impl Position {
    fn can_access(&self, other: &Self, reverse: bool) -> bool {
        // Must be on either same X or same Y axis
        if self.x != other.x && self.y != other.y {
            return false;
        }

        // Must not be more than 1 square away
        if isize::abs(self.x as isize - other.x as isize) > 1 {
            return false
        }

        if isize::abs(self.y as isize - other.y as isize) > 1 {
            return false;
        }

        // Other must be any distance below, same level, or at most 1 above
        if reverse {
            other.altitude >= self.altitude - 1
        } else {
            self.altitude >= other.altitude - 1
        }
    }
}

#[derive(Debug, PartialEq)]
struct Graph {
    begin: Position,
    end: Position,
    field: Vec<Vec<usize>>,
}

impl Graph {

    fn parse(input: &Vec<String>) -> Option<Self> {

        let mut begin: Option<Position> = None;
        let mut end: Option<Position> = None;

        let field: Vec<Vec<usize>> = input
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line
                    .chars()
                    .enumerate()
                    .map(|(x, c)| {
                        // Map a-z + S/E to altitudes. Make note of begin
                        // and end when encountered.
                        match c {
                            'S' => {
                                let a = 1;
                                begin = Some(Position { y, x, altitude: a });
                                a
                            },

                            'E' => {
                                let a = 26;
                                end = Some(Position { y, x, altitude: a });
                                a
                            }

                            _ => {
                                // 'a' = ascii 97
                                // a = 1
                                c as usize- 96
                            }
                        }

                    })
                    .collect()
            })
            .collect();

        Some(Graph { begin: begin?, end: end?, field })
    }

    fn get_neighbours(&self, node: &Position, reverse: bool) -> Vec<Position> {
        vec![
            if node.y > 0 {
                self.try_get_node(node.y - 1, node.x)
            } else {
                None
            },

            if node.x > 0 {
                self.try_get_node(node.y, node.x - 1)
            } else {
                None
            },

            self.try_get_node(node.y, node.x + 1),
            self.try_get_node(node.y + 1, node.x),
        ]
        .iter()
        .filter_map(|n| *n)
        .filter(|n| node.can_access(n, reverse))
        .collect()
    }

    fn try_get_node(&self, y: usize, x: usize) -> Option<Position> {
        self.field.get(y)?.get(x).map(|a| {
            Position { y, x, altitude: *a }
        })
    }

    // Finds the shortest path from start to end using breadth-first-search
    // Returns the number of steps required to get there
    fn find_shortest_path<T>(
        &self,
        start: &Position,
        reverse: bool, // Reverses the check for which neighbours are accessible
                       // when traversing from opposite direction
        reached_goal: T // A lambda that, given a position, decides that we've reached where we
                        // need to be
    ) -> Option<usize> where T: Fn(&Position) -> bool {
        let mut queue = VecDeque::<(Position, usize)>::new();
        let mut visited = HashSet::<Position>::new();

        // Start the processing queue off with our start position, and number of steps in
        queue.push_back((*start, 0));

        loop {
            if queue.is_empty() { break; }

            // Grab next node to process
            let (pos, step) = queue.pop_front().unwrap();

            // If we've already visited this node, skip to the next
            if visited.contains(&pos) { continue; }

            // We have visited the end, return how many steps we took to get here
            if reached_goal(&pos) { return Some(step); }

            // Mark this node as visited so it does not get processed again
            visited.insert(pos);

            // Find all the accessible neighbouring nodes that we have not yet
            // visited, and queue them up to process (after we've finished
            // processing all the nodes at this level that remain in the queue,
            // which is what makes this a breadth-first search
            self.get_neighbours(&pos, reverse)
                .iter()
                .filter(|n| ! visited.contains(*n))
                .for_each(|n| {
                    queue.push_back((*n, step + 1));
                });
        }

        // If we get out of the loop that means we've traversed the entire
        // graph from begin without finding the end; it is unreachable.
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn testing_parsing_map() {
        let graph = Graph::parse(&vec![
             "Sabqponm".to_string(),
             "abcryxxl".to_string(),
             "accszExk".to_string(),
             "acctuvwj".to_string(),
             "abdefghi".to_string(),
        ]).unwrap();

        assert_eq!(graph.begin, Position { y: 0, x: 0, altitude: 1 });
        assert_eq!(graph.end, Position { y: 2, x: 5, altitude: 26 });
        assert_eq!(graph.field[0][0], 1);
        assert_eq!(graph.field[0][3], 17);
        assert_eq!(graph.field[2][5], 26);
        assert_eq!(graph.field[4][0], 1);
        assert_eq!(graph.field[4][7], 9);
    }

    #[test]
    fn finds_shortest_path() {
        let graph = Graph::parse(&vec![
             "Sabqponm".to_string(),
             "abcryxxl".to_string(),
             "accszExk".to_string(),
             "acctuvwj".to_string(),
             "abdefghi".to_string(),
        ]).unwrap();

        // Part 1
        assert_eq!(
            graph.find_shortest_path(
                &graph.begin,
                false, // reverse
                |p| p == &graph.end
            ).unwrap(),
            31,
        );

        // Part 2
        assert_eq!(
            graph.find_shortest_path(
                &graph.end,
                true, // reverse
                |p| p.altitude == 1,
            ).unwrap(),
            29,
        );
    }
}
