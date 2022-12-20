use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{}", output);
}

fn process_part_one(input: Vec<String>) -> String {
    let mut enc_file = EncFile::init(&input, 1);
    enc_file.mix();

    format!("{}", enc_file.grove_coord_sum())
}

fn process_part_two(input: Vec<String>) -> String {
    let mut enc_file = EncFile::init(&input, 811589153);
    (0..10).for_each(|_| {
        enc_file.mix();
    });

    format!("{}", enc_file.grove_coord_sum())
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

// Absolute index, new pos
type PosMove = (usize, usize);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Entry {
    pos: usize,
    val: isize,
}

impl Entry {
    fn calc_pos(&self, list_len: usize) -> usize {
        let mut new_pos = (self.pos as isize + (self.val % (list_len as isize -1)))
            % (list_len as isize - 1);

        new_pos = if new_pos < 0 {
            new_pos - 1 + list_len as isize
        } else {
            new_pos
        };

        new_pos as usize
    }
}

#[derive(Debug, PartialEq)]
struct EncFile {
    entries: Vec<Entry>,
    len: usize,
}

impl EncFile {
    fn init(input: &Vec<String>, key: isize) -> Self {
        EncFile {
            entries: input
                .iter()
                .map(|l| isize::from_str_radix(l, 10).unwrap() * key)
                .enumerate()
                .map(|(pos, val)| Entry { pos, val })
                .collect(),
            len: input.len()
        }
    }

    fn mix(&mut self) {
        (0..self.len).for_each(|ai| {
            let map: HashMap<usize, usize> = self.entries
                .iter()
                .enumerate()
                .map(|(i, e)| (e.pos, i))
                .collect();

            let old_pos = self.entries[ai].pos;
            let new_pos = self.entries[ai].calc_pos(self.len);

            let moves = if old_pos < new_pos {
                ((old_pos + 1)..=new_pos).map(|pos_to_update| {
                    (map[&pos_to_update], pos_to_update - 1)
                }).collect::<Vec<PosMove>>()
            } else {
                (new_pos..old_pos).map(|pos_to_update| {
                    (map[&pos_to_update], pos_to_update + 1)
                }).collect::<Vec<PosMove>>()
            };

            moves.iter().for_each(|(i, new_pos)| {
                self.entries[*i].pos = *new_pos;
            });

            self.entries[ai].pos = new_pos;
        });
    }

    fn current_order(&self) -> Vec<isize> {
        let mut cloned = self.entries.clone();
        cloned.sort_by(|a, b| a.pos.cmp(&b.pos));
        cloned.iter().map(|e| e.val).collect()
    }

    fn grove_coord_sum(&self) -> isize {
        let current = self.current_order();
        let index_0 = current
            .iter()
            .enumerate()
            .find(|(i, v)| **v == 0)
            .map(|(i, _)| i)
            .unwrap();

        let indexes = vec![
            (index_0 + 1000) % self.len,
            (index_0 + 2000) % self.len,
            (index_0 + 3000) % self.len,
        ];

        indexes
            .iter()
            .map(|i| current[*i])
            .fold(0, |acc, a| {
                acc + a
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_new_pos() {
        let entry = Entry { pos: 0, val: 1 };
        assert_eq!(entry.calc_pos(7), 1);

        let entry = Entry { pos: 0, val: 2 };
        assert_eq!(entry.calc_pos(7), 2);

        let entry = Entry { pos: 1, val: -3 };
        assert_eq!(entry.calc_pos(7), 4);

        let entry = Entry { pos: 2, val: 3 };
        assert_eq!(entry.calc_pos(7), 5);

        let entry = Entry { pos: 2, val: -2 };
        assert_eq!(entry.calc_pos(7), 6);

        let entry = Entry { pos: 3, val : 0 };
        assert_eq!(entry.calc_pos(7), 3);

        let entry = Entry { pos: 5, val: 4 };
        assert_eq!(entry.calc_pos(7), 3);

    }

    #[test]
    fn test_mix() {
        let mut file = EncFile {
            entries: vec![1, 2, -3, 3, -2, 0, 4]
                .iter()
                .enumerate()
                .map(|(i, v)| Entry { pos: i, val: *v })
                .collect(),
            len: 7
        };

        assert_eq!(file.current_order(), vec![1, 2, -3, 3, -2, 0, 4]);

        file.mix();

        assert_eq!(file.current_order(), vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_grove_coord() {

        let mut file = EncFile {
            entries: vec![1, 2, -3, 3, -2, 0, 4]
                .iter()
                .enumerate()
                .map(|(i, v)| Entry { pos: i, val: *v })
                .collect(),
            len: 7
        };

        file.mix();

        assert_eq!(file.grove_coord_sum(), 3);
    }
}