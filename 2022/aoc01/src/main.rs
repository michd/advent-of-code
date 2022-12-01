use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

// Get the total calories carried by the elf with the most calories
fn process_part_one(input: Vec<String>) -> String {
    let elves = parse_elves(input);

    let mut highest_calories: i64 = 0;

    for elf in elves.iter() {
        let elf_calories = elf.total_calories();

        if elf_calories > highest_calories {
            highest_calories = elf_calories
        }
    }

    format!("{highest_calories}")
}

// Get the total calories carried by the 3 elves with the most calories
fn process_part_two(input: Vec<String>) -> String {
    let mut elves = parse_elves(input);

    //b.cmp(a) instead of a.cmp(b) sorts in reverse; biggest first.
    elves.sort_by(|a, b| b.total_calories().cmp(&a.total_calories()));

    let cumulative_calories = &elves[0..3].iter().fold(0, |acc, elf| {
        acc + elf.total_calories()
    });

    format!("{cumulative_calories}")
}

// Input comes as calories per food item, in blocks of food items carried per
// elf. A blank line indicates the end of the list for one elf, thus separating
// the elves food lists.
fn parse_elves(input: Vec<String>) -> Vec<Elf> {
    let mut elves = Vec::new();
    let mut elf_food_items = Vec::new();

    for line in input.iter() {
        if line.trim().is_empty() {
            // Blank line = elf finished; create elf instances with what we've
            // accumulated in elf_food_items and clear it for next elf
            if !elf_food_items.is_empty() {
                elves.push(Elf::new(elf_food_items.clone()));
                elf_food_items.clear()
            }
        } else {
            // Otherwise, collect a food items by parsing it as an int,
            // and add it to the current list of food items we're collecting
            if let Ok(calories) = i64::from_str_radix(&line, 10) {
                elf_food_items.push(calories)
            }
        }
    }

    elves
}

struct Elf {
    food_items: Vec<i64>
}

impl Elf {
    pub fn new(items: Vec<i64>) -> Elf {
        Elf {
            food_items: items
        }
    }

    pub fn total_calories(&self) -> i64 {
        self.food_items.iter().fold(0, |acc, x| acc + x)
    }
}
