use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_two(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let total_score = input.iter().fold(0, |score, line| {
        let (opponent, you) = get_shapes_part_1(&line);
        score + calculate_round_score(&opponent, &you)
    });
    format!("{total_score}")
}

fn process_part_two(input: Vec<String>) -> String {
    let total_score = input.iter().fold(0, |score, line| {
        let (opponent, you) = get_shapes_part_2(&line);
        score + calculate_round_score(&opponent, &you)
    });
    format!("{total_score}")
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

fn get_shapes_part_1(input_line: &str) -> (Shape, Shape) {
    (
        Shape::from_char(input_line.chars().nth(0).unwrap()).unwrap(),
        Shape::from_char(input_line.chars().nth(2).unwrap()).unwrap(),
    )
}

fn get_shapes_part_2(input_line: &str) -> (Shape, Shape) {
    let opponent = Shape::from_char(input_line.chars().nth(0).unwrap()).unwrap();
    let outcome = Outcome::from_char(input_line.chars().nth(2).unwrap()).unwrap();

    let you = match (opponent, outcome) {
        (Shape::Rock, Outcome::Loss) => Shape::Scissors,
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Paper, Outcome::Loss) => Shape::Rock,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Scissors, Outcome::Loss) => Shape::Paper,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        _ => opponent.clone(),
    };

    (opponent, you)
}

fn calculate_round_score(opponent: &Shape, you: &Shape) -> i64 {
    let shape_score = you.value();
    let outcome_score = match (opponent, you) {
        (Shape::Scissors, Shape::Rock) => 6,
        (Shape::Rock, Shape::Paper) => 6,
        (Shape::Paper, Shape::Scissors) => 6,
        (Shape::Rock, Shape::Rock) => 3,
        (Shape::Paper, Shape::Paper) => 3,
        (Shape::Scissors, Shape::Scissors) => 3,
        _ => 0,
    };

    shape_score + outcome_score
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn value(&self) -> i64 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn from_char(c: char) -> Option<Shape> {
        // X,Y,Z only relevant for part one of the challenge.
        match c {
            'A' | 'X' => Some(Self::Rock),
            'B' | 'Y' => Some(Self::Paper),
            'C' | 'Z' => Some(Self::Scissors),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

impl Outcome {
    fn from_char(c: char) -> Option<Outcome> {
        match c {
            'X' => Some(Self::Loss),
            'Y' => Some(Self::Draw),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Shape;
    use crate::Outcome;
    use crate::calculate_round_score;
    use crate::get_shapes_part_1;
    use crate::get_shapes_part_2;


    #[test]
    fn correctly_converts_char_to_shape() {
        assert_eq!(
            Shape::from_char('A').unwrap(),
            Shape::Rock,
        );

        assert_eq!(
            Shape::from_char('B').unwrap(),
            Shape::Paper,
        );

        assert_eq!(
            Shape::from_char('C').unwrap(),
            Shape::Scissors,
        );

        assert_eq!(
            Shape::from_char('X').unwrap(),
            Shape::Rock,
        );

        assert_eq!(
            Shape::from_char('Y').unwrap(),
            Shape::Paper,
        );

        assert_eq!(
            Shape::from_char('Z').unwrap(),
            Shape::Scissors,
        );

        assert_eq!(
            Shape::from_char('Q'),
            None,
        );
    }

    #[test]
    fn calculates_correct_round_score() {
        assert_eq!(
            calculate_round_score(
                &Shape::from_char('A').unwrap(),
                &Shape::from_char('Y').unwrap(),
            ),
            8,
        );

        assert_eq!(
            calculate_round_score(
                &Shape::from_char('B').unwrap(),
                &Shape::from_char('X').unwrap(),
            ),
            1,
        );

        assert_eq!(
            calculate_round_score(
                &Shape::from_char('C').unwrap(),
                &Shape::from_char('Z').unwrap(),
            ),
            6,
        );
    }

    #[test]
    fn get_shapes_part_1_works_correctly() {
        assert_eq!(
            get_shapes_part_1("A Y"),
            (Shape::Rock, Shape::Paper),
        );

        assert_eq!(
            get_shapes_part_1("B X"),
            (Shape::Paper, Shape::Rock),
        );

        assert_eq!(
            get_shapes_part_1("C Z"),
            (Shape::Scissors, Shape::Scissors),
        );
    }

    #[test]
    fn correct_converts_char_to_outcome() {
        assert_eq!(
            Outcome::from_char('X').unwrap(),
            Outcome::Loss,
        );

        assert_eq!(
            Outcome::from_char('Y').unwrap(),
            Outcome::Draw,
        );

        assert_eq!(
            Outcome::from_char('Z').unwrap(),
            Outcome::Win,
        );
    }

    #[test]
    fn get_shapes_part_2_works_correctly() {
        assert_eq!(
            get_shapes_part_2("A Y"),
            (Shape::Rock, Shape::Rock),
        );

        assert_eq!(
            get_shapes_part_2("B X"),
            (Shape::Paper, Shape::Rock),
        );

        assert_eq!(
            get_shapes_part_2("C Z"),
            (Shape::Scissors, Shape::Rock),
        );
    }
}
