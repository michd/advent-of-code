use std::io;
use std::io::BufRead;


fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let mut tree_field = TreeField::from_text(input);
    tree_field.scan_cover();

    let visible_count = tree_field.trees.iter().fold(0, |acc, row| {
        acc + row.iter().fold(0, |r_acc, tree| {
            r_acc + match tree.is_visible() {
                true => 1,
                false => 0
            }
        })
    });
    format!("{visible_count}")
}

#[allow(dead_code)]
fn process_part_two(input: Vec<String>) -> String {
    let mut tree_field = TreeField::from_text(input);
    tree_field.scan_visibility();

    let highest_vis = tree_field.trees.iter().fold(0, |row_best, row| {
        row.iter().fold(row_best, |best, tree| {
            let score = tree.scenic_score();

            if score > best {
                score
            } else {
                best
            }
        })
    });

    format!("{highest_vis}")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Tree {
    height: i8,
    cover_north: Option<i8>,
    cover_east: Option<i8>,
    cover_south: Option<i8>,
    cover_west: Option<i8>,
    view_north: i64,
    view_east: i64,
    view_south: i64,
    view_west: i64,
}

impl Tree {
    fn new(height: i8) -> Tree {
        Tree {
            height,
            cover_north: None,
            cover_east: None,
            cover_south: None,
            cover_west: None,
            view_north: 0,
            view_east: 0,
            view_south: 0,
            view_west: 0,
        }
    }

    fn is_visible(&self) -> bool {
        let (n, e, s, w) = (
            self.cover_north.unwrap_or(0),
            self.cover_east.unwrap_or(0),
            self.cover_south.unwrap_or(0),
            self.cover_west.unwrap_or(0),
        );

        let h = self.height;

        n < h || e < h || s < h || w < h
    }

    fn scenic_score(&self) -> i64 {
        let (n, e, s, w) = (
            i64::from(self.view_north),
            i64::from(self.view_east),
            i64::from(self.view_south),
            i64::from(self.view_west),
        );

        n * e * s * w
    }
}

#[derive(Debug, PartialEq)]
struct TreeField {
    trees: Vec<Vec<Tree>>
}

impl TreeField {
    fn from_text(input: Vec<String>) -> TreeField {
        let trees: Vec<Vec<Tree>> = input.iter().map(|r| {
            r
                .chars()
                .filter_map(|t| {
                    t.to_string().parse::<i8>().ok()
                })
                .map(Tree::new)
                .collect()
        }).collect();

        TreeField { trees }
    }

    fn scan_cover(&mut self) {
        self.trees = Self::scan_cover_west(&self.trees);
        self.trees = Self::scan_cover_east(&self.trees);
        self.trees = Self::scan_cover_north(&self.trees);
        self.trees = Self::scan_cover_south(&self.trees);
    }

    /// Cover scanning:
    ///
    /// The idea here is that we keep track of the highest tree we've
    /// seen so far, in the direction we're scanning. As we come across each
    /// tree, we store the height of the highest tree we've seen before
    /// encountering the current tree.
    ///
    /// If the current tree is taller than the tallest one we've seen so far,
    /// we make note of the new tallest height.
    ///
    /// Then, we can evaluate whether the tree is visible from that direction
    /// by checking if it's taller than the tallest tree found on that side.
    ///
    /// The scanning is split into four methods because the direction of
    /// scanning calls for some different iteration each time.
    ///
    /// Arguably some of the code can be abstracted out here, but I think
    /// it's good enough.
    ///
    /// This approach means we only need to iterate over any one row/column
    /// once for a given directional scan. I don't know how you'd express that
    /// in big O notation because I never took computer science, but it's got
    /// to count for something.
    fn scan_cover_west(input_trees: &Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
        input_trees 
            .iter()
            .map(|row| {
                let mut new_row: Vec<Tree> = vec![];

                row.iter().fold(-1, |cover_west, tree| {
                    let mut scanned_tree = tree.clone();
                    scanned_tree.cover_west = Some(cover_west);

                    // Kind of abusing fold here to make it alter something...
                    new_row.push(scanned_tree);

                    if tree.height > cover_west {
                        tree.height
                    } else {
                        cover_west
                    }
                });

                new_row
            })
            .collect()
    }

    fn scan_cover_east(input_trees: &Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
        input_trees
            .iter()
            .map(|row| {
                let mut new_row: Vec<Tree> = vec![];

                row.iter().rev().fold(-1, |cover_east, tree| {
                    let mut scanned_tree = tree.clone();
                    scanned_tree.cover_east = Some(cover_east);

                    // Instead of pushing to end, this negates the .rev()
                    // above and preserves original order
                    new_row.insert(0, scanned_tree);

                    if tree.height > cover_east {
                        tree.height
                    } else {
                        cover_east
                    }
                });

                new_row
            })
            .collect()
    }

    fn scan_cover_north(input_trees: &Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
        let grid_w = input_trees[0].len();

        let mut cover_north: Vec<i8> = (0..grid_w).map(|_| -1).collect();

        input_trees.iter().map(|row| {
            row.iter().enumerate().map(|(x, tree)| {
                let mut scanned_tree = tree.clone();
                scanned_tree.cover_north = Some(cover_north[x]);

                if tree.height > cover_north[x] {
                    cover_north[x] = tree.height
                }

                scanned_tree
            }).collect()
        }).collect()
    }

    fn scan_cover_south(input_trees: &Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
        let grid_w = input_trees[0].len();

        let mut cover_south: Vec<i8> = (0..grid_w).map(|_| -1).collect();

        let mut new_trees: Vec<Vec<Tree>> = input_trees.iter().rev().map(|row| {
            row.iter().enumerate().map(|(x, tree)| {
                let mut scanned_tree = tree.clone();
                scanned_tree.cover_south = Some(cover_south[x]);

                if tree.height > cover_south[x] {
                    cover_south[x] = tree.height
                }

                scanned_tree
            }).collect()
        }).collect();

        new_trees.reverse();
        new_trees
    }

    fn scan_visibility(&mut self) {
        self.trees = Self::scan_visibility_west(&self.trees);
        self.trees = Self::scan_visibility_east(&self.trees);
        self.trees = Self::scan_visibility_south(&self.trees);
        self.trees = Self::scan_visibility_north(&self.trees);
    }

    /// Visibility scanning:
    ///
    /// Somewhat similar to cover scanning, this is done separately for each
    /// direction.
    ///
    /// Since we have only 10 total possible tree heights, we can store how
    /// far you can see from each height.
    ///
    /// As we move in from one side, we assign the visibility + 1 from the
    /// height of the current tree, to that tree's view in this direction.
    ///
    /// Then we update visibility at each height:
    /// - At heights up to the current tree, the visibility is reset to 0,
    ///   since the current tree blocks the view.
    /// - At heights higher than the current tree, we increment number of
    ///   visible trees to include the current tree.
    ///
    /// This approach means we only need to iterate over any one row/column
    /// once for a given directional scan. I don't know how you'd express that
    /// in big O notation because I never took computer science, but it's got
    /// to count for something.
    fn scan_visibility_west(input_trees: &Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
        input_trees
            .iter()
            .map(|row| {
                // Current distance visible from each height, height set by
                // index in vis_h. Distance is # of trees
                let mut vis_h = vec![ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ];

                row.iter().map(|tree| {
                    let mut scanned_tree = tree.clone();

                    scanned_tree.view_west = vis_h[tree.height as usize] + 1;

                    // Mark new visibility at height
                    (0..=(tree.height)).for_each(|h| vis_h[h as usize] = 0);
                    ((tree.height+1)..=9).for_each(|h| vis_h[h as usize] += 1);


                    scanned_tree
                })
                .collect()
            })
            .collect()
    }

    fn scan_visibility_east(input_trees: &Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
        input_trees
            .iter()
            .map(|row| {
                // Current distance visible from each height, height set by
                // index in vis_h. Distance is # of trees
                let mut vis_h = vec![ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ];

                let mut new_row = vec![];

                row.iter().rev().for_each(|tree| {
                    let mut scanned_tree = tree.clone();

                    scanned_tree.view_east = vis_h[tree.height as usize] + 1;

                    // Mark new visibility at height
                    (0..=(tree.height)).for_each(|h| vis_h[h as usize] = 0);
                    ((tree.height+1)..=9).for_each(|h| vis_h[h as usize] += 1);

                    new_row.insert(0, scanned_tree);
                });

                new_row
            })
            .collect()
    }

    fn scan_visibility_north(input_trees: &Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
        let grid_h = input_trees.len();
        let grid_w = input_trees[0].len();

        let mut new_field: Vec<Vec<Tree>> = (0..grid_h).map(|_| vec![]).collect();

        (0..grid_w).for_each(|x| {

            let mut vis_h = vec![ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ];

            (0..grid_h).for_each(|y| {
                let tree = input_trees[y][x];
                let mut scanned_tree = tree.clone();

                scanned_tree.view_north = vis_h[scanned_tree.height as usize] + 1;
                // Mark new visibility at height
                (0..=(tree.height)).for_each(|h| vis_h[h as usize] = 0);
                ((tree.height+1)..=9).for_each(|h| vis_h[h as usize] += 1);

                new_field[y].push(scanned_tree);
            });
        });

        new_field
    }

    fn scan_visibility_south(input_trees: &Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
        let grid_h = input_trees.len();
        let grid_w = input_trees[0].len();

        let mut new_field: Vec<Vec<Tree>> = (0..grid_h).map(|_| vec![]).collect();

        (0..grid_w).for_each(|x| {

            let mut vis_h = vec![ -1, -1, -1, -1, -1, -1, -1, -1, -1, -1 ];

            (0..grid_h).rev().for_each(|y| {
                let tree = input_trees[y][x];
                let mut scanned_tree = tree.clone();

                scanned_tree.view_south = vis_h[tree.height as usize] + 1;
                // Mark new visibility at height
                (0..=(tree.height)).for_each(|h| vis_h[h as usize] = 0);
                ((tree.height+1)..=9).for_each(|h| vis_h[h as usize] += 1);

                new_field[y].push(scanned_tree);
            });
        });

        new_field
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new_tree() {
        assert_eq!(
            Tree::new(3),
            Tree {
                height: 3,
                cover_north: None,
                cover_east: None,
                cover_south: None,
                cover_west: None,
                view_north: 0,
                view_east: 0,
                view_south: 0,
                view_west: 0,

            },
        );
    }

    #[test]
    fn test_parsing_tree_field() {
        let input = vec![
            "123".to_string(),
            "456".to_string(),
            "789".to_string(),
        ];

        assert_eq!(
            TreeField::from_text(input),
            TreeField {
                trees: vec![
                    vec![ Tree::new(1), Tree::new(2), Tree::new(3) ],
                    vec![ Tree::new(4), Tree::new(5), Tree::new(6) ],
                    vec![ Tree::new(7), Tree::new(8), Tree::new(9) ],
                ],
            }
        );
    }

    #[test]
    fn test_scanning_cover() {
        let input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let mut field = TreeField::from_text(input);
        field.scan_cover();

        // The top-left 5 is visible from the left and top. (It isn't visible from the right or
        // bottom since other trees of height 5 are in the way.)
        assert!(field.trees[1][1].is_visible());

        // The top-middle 5 is visible from the top and right.
        assert!(field.trees[1][2].is_visible());

        // The top-right 1 is not visible from any direction; for it to be visible, there would
        // need to only be trees of height 0 between it and an edge.
        assert!( ! field.trees[1][3].is_visible());

        // The left-middle 5 is visible, but only from the right.
        assert!(field.trees[2][1].is_visible());

        // The center 3 is not visible from any direction; for it to be visible, there would need
        // to be only trees of at most height 2 between it and an edge.
        assert!( ! field.trees[2][2].is_visible());

        // The right-middle 3 is visible from the right.
        assert!(field.trees[2][3].is_visible());

        // In the bottom row, the middle 5 is visible, but the 3 and 4 are not.
        assert!(field.trees[3][2].is_visible());

        // Check all the outside ones are visible - north edge
        for tree in field.trees[0].iter() {
            assert!(tree.is_visible());
        }

        // south edge
        for tree in field.trees[4].iter() {
            assert!(tree.is_visible());
        }

        for row in field.trees.iter() {
            // west edge
            assert!(row[0].is_visible());

            // east edge
            assert!(row[4].is_visible());
        }
    }

    #[test]
    fn test_scanning_visibility() {
        let input = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let mut field = TreeField::from_text(input);
        field.scan_visibility();

        assert_eq!(field.trees[1][2].scenic_score(), 4);
        assert_eq!(field.trees[3][2].scenic_score(), 8);

        for tree in field.trees[0].iter() {
            assert_eq!(tree.scenic_score(), 0);
        }

        for tree in field.trees[4].iter() {
            assert_eq!(tree.scenic_score(), 0);
        }

        for row in field.trees.iter() {
            assert_eq!(row[0].scenic_score(), 0);
            assert_eq!(row[4].scenic_score(), 0);
        }
    }
}

