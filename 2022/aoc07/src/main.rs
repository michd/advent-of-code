use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    // Goal: find all dirs with total size of _at most_ 100 000, sum their
    // sizes
    // important: nesting does not matter for this so a directory and its
    // ancestor can individually be counted as this (which seems counterintuitive)
    let mut parser = Parser::new();

    let nodes = parser.parse(input);

    let dirs: Vec<&Node> = nodes.iter().filter(|n| n.size.is_none()).collect();

    let filtered_dir_sizes = dirs.iter().filter_map(|d| {
        let size = nodes
            .iter()
            // files in dir
            .filter(|n| n.path.starts_with(&d.path) && n.size.is_some())
            .fold(0, |acc, f| acc + f.size.unwrap());

        if size <= 100000 {
            Some(size)
        } else {
            None
        }
    });

    let total_size = filtered_dir_sizes.fold(0, |acc, size| acc + size);

    format!("{total_size}")
}

fn process_part_two(input: Vec<String>) -> String {
    let mut parser = Parser::new();

    let nodes = parser.parse(input);

    let total_size = nodes
        .iter()
        .filter(|n| n.size.is_some())
        .map(|n| n.size.unwrap())
        .fold(0, |acc, s| acc + s);

    let disk_size = 70000000;

    let required_space = 30000000;

    let need_to_free = required_space - (disk_size - total_size);

    let mut big_enough_sizes: Vec<u64> = nodes
        .iter()
        .filter(|n| n.size.is_none())
        .filter_map(|d| {
            let size = nodes
                .iter()
                .filter(|n| n.path.starts_with(&d.path) && n.size.is_some())
                .fold(0, |acc, f| acc + f.size.unwrap());
            if size >= need_to_free {
                Some(size)
            } else {
                None
            }
        }).collect();

    big_enough_sizes.sort();

    format!("{0}", big_enough_sizes[0])
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    path: String,
    size: Option<u64>,
}

impl Node {
    fn parse(input: &str, cur_dir: &str) -> Option<Node> {
        let split: Vec<&str> = input.split(' ').collect();

        let first_token = *split.get(0).unwrap();
        let second_token = *split.get(1)?;

        match first_token {
            "dir" => Some(
                Node {
                    path: format!("{cur_dir}{second_token}/"),
                    size: None,
                }
            ),

            "$" => None,

            _ => match u64::from_str_radix(first_token, 10) {
                Ok(size) => Some(
                    Node {
                        path: format!("{cur_dir}{second_token}"),
                        size: Some(size),
                    }
                ),

                _ => None,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum Command {
    ChangeDirRoot,
    ChangeDirUp,
    ChangeDirNamed(String),
    List,
}

impl Command {
    /// Parses a command from a line starting with `$`.
    /// Returns None if the line does not start with '$' or some other unexpected
    /// input is given.
    fn parse(input: &str) -> Option<Command> {
        let split: Vec<&str> = input.split(' ').collect();

        if *split.get(0).unwrap() != "$" {
            return None;
        }

        let cmd = *split.get(1).unwrap();

        if cmd == "ls" {
            return Some(Self::List);
        }


        if cmd == "cd" {
            let target = *split.get(2).unwrap();

            return match target {
                "/" => Some(Self::ChangeDirRoot),
                ".." => Some(Self::ChangeDirUp),
                _ => Some(Self::ChangeDirNamed(target.to_string()))
            };
        }

        None
    }
}

#[derive(Debug)]
struct Parser {
    expect_node: bool,
    cur_dir: String,
    nodes: Vec<Node>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            expect_node: false,
            cur_dir: "/".to_string(),
            nodes: vec![],
        }
    }

    fn get_parent(current: &str) -> String {
        if current == "/" {
            return current.to_string();
        }

        let current_split: Vec<&str> = current.split("/").collect();
        if current_split.len() <= 3 {
            return "/".to_string();
        }

        format!("{}/", current_split[..current_split.len()-2].join("/"))
    }

    fn execute_command<'a>(&'a mut self, command: Command) {
        match command {
            Command::ChangeDirRoot => {
                self.cur_dir = "/".to_string();
                self.expect_node = false;
            }

            Command::ChangeDirUp => {
                self.cur_dir = Self::get_parent(&self.cur_dir);
                self.expect_node = false;
            }

            Command::ChangeDirNamed(name) => {
                self.cur_dir = format!("{}{}/", self.cur_dir, name);
                self.expect_node = false;
            }

            Command::List => {
                self.expect_node = true;
            }
        }
    }

    fn parse_line(&mut self, line: &str) {
        if self.expect_node {
            let node = Node::parse(line, &self.cur_dir);

            if let Some(node) = node {
                self.nodes.push(node);
                return;
            }
        }

        let command = Command::parse(line);

        match command {
            Some(command) => {
                self.execute_command(command);
            }

            None => {
                panic!("Expected a command but failed to parse one.");
            }
        }
    }

    fn parse(&mut self, input: Vec<String>) -> &Vec<Node> {
        // Lines formats to expect:
        // 1. Lines starting with '$':
        //    a. `$ ls`: directory listing follows
        //    b. `cd (target)`: change to other directory
        //        allowed targets:
        //        - `/`: Go to root directory
        //        - `..`: Go one directory up
        //        - <some name>: go one directory deeper, to named directory
        // 2. Directory listing lines:
        //    a. `dir <name>`: directory that may contain more stuff
        //    b. `<decimal digits> <filename>`: File preceded by its size
        //

        for line in input.iter() {
            self.parse_line(line);
        }

        self.nodes.sort();
        self.nodes.dedup();

        &self.nodes
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_command_works() {
        assert_eq!(Command::parse("invalid"), None);
        assert_eq!(Command::parse("$ ls"), Some(Command::List));
        assert_eq!(Command::parse("$ cd /"), Some(Command::ChangeDirRoot));
        assert_eq!(Command::parse("$ cd .."), Some(Command::ChangeDirUp));
        assert_eq!(Command::parse("$ cd foo"), Some(Command::ChangeDirNamed("foo".to_string())));
    }

    #[test]
    fn parse_node_works() {
        assert_eq!(Node::parse("lkdjfgkljdf fsjkh", "/"), None);
        assert_eq!(Node::parse("$ ls", "/"), None);
        assert_eq!(Node::parse("dir foobar.roo", "/"), Some(Node { path: "/foobar.roo/".to_string(), size: None } ));
        assert_eq!(
            Node::parse("123 roo.foo", "/"),
            Some(
                Node { path: "/roo.foo".to_string(), size: Some(123) }
            ),
        );
    }

    #[test]
    fn test_get_parent() {
        assert_eq!(Parser::get_parent("/"), "/".to_string());
        assert_eq!(Parser::get_parent("/foo/"), "/".to_string());
        assert_eq!(Parser::get_parent("/foo/bar/"), "/foo/".to_string());
    }
}
