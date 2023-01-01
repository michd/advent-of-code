use std::io;
use std::io::BufRead;

fn main() {
    let input = read_stdin();
    let output = process_part_one(input);
    println!("{output}");
}

fn process_part_one(input: Vec<String>) -> String {
    let dec_sum = input
        .iter()
        .fold(0, |acc, snafu| {
            acc + snafu_to_dec(snafu)
        });

    format!("{}", dec_to_snafu(dec_sum, None))
}

fn read_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin.lock().lines().map(|l| l.unwrap()).collect();
}

fn snafu_to_dec(input: &str) -> isize {
    input
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .iter()
        .enumerate()
        .fold(0, |acc, (i, c)| {
            let mult = 5isize.pow(i as u32);
            acc + mult * match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '-' => -1,
                '=' => -2,
                _ => 0,
            }
        })
}

fn snafu_max_with_digits(digits: u32) -> isize {
    (0..digits).fold(0, |acc, n| acc + 2 * 5isize.pow(n ))
}

fn snafu_min_with_digits(digits: u32) -> isize {
    (0..digits).fold(0, |acc, n| acc + -2 * 5isize.pow(n))
}

fn dec_to_snafu(input: isize, prev_index: Option<usize>) -> String {
    if input == 0 {
        return format!("0");
    }

    return if input > 0 {
        let d = (0..100u32)
            .map(|n| {
                (n, snafu_max_with_digits(n))
            })
            .find(|(_, max)| *max >= input)
            .map(|(d, _)| d)
            .unwrap();

        let digit_val = 5isize.pow(d - 1);

        let digit_index = prev_index.unwrap_or(d as usize);

        if digit_val == input {
            return format!("{}1{}", "0".repeat(digit_index - d as usize), "0".repeat(d as usize - 1))
        } else if digit_val * 2 == input {
            return format!("{}2{}", "0".repeat(digit_index - d as usize), "0".repeat(d as usize - 1))
        }

        let (snafu_digit, remainder) = if (input - digit_val).abs() * 10 > digit_val * 10 / 2 {
            (format!("2"), input - 5isize.pow(d - 1) * 2)
        } else {
            (format!("1"), input - 5isize.pow(d - 1))
        };

        format!(
            "{}{}{}",
            "0".repeat(digit_index - d as usize),
            snafu_digit,
            dec_to_snafu(remainder, Some(d as usize - 1)),
        )

    } else {
        let d = (0..100u32)
            .map(|n| {
                (n, snafu_min_with_digits(n))
            })
            .find(|(_, min)| *min <= input)
            .map(|(d, _)| d)
            .unwrap();

        let digit_val = 5isize.pow(d - 1) * -1;

        let digit_index = prev_index.unwrap_or(d as usize);

        let digit = if digit_val == input { "-" } else { "=" };

        if digit_val == input {
            return format!(
                "{}{}{}",
                "0".repeat(digit_index - d as usize), // prefix 0-padding
                digit,
                "0".repeat(d as usize - 1), // suffix 0-padding
            )
        } else if digit_val * 2 == input {
            return format!(
                "{}={}",
                "0".repeat(digit_index - d as usize),
                "0".repeat(d as usize - 1),
            )
        }

        let (snafu_digit, remainder) = if (digit_val - input).abs() * 10 < digit_val.abs() * 10 / 2 {
            (format!("-"), input - digit_val)
        } else {
            (format!("="), input - digit_val * 2)
        };

        format!(
            "{}{}{}",
            "0".repeat(digit_index - d as usize),
            snafu_digit,
            dec_to_snafu(remainder, Some(d as usize - 1)),
        )
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_snafu_to_dec() {
        assert_eq!(snafu_to_dec("1"), 1);
        assert_eq!(snafu_to_dec("1=-0-2"), 1747);
        assert_eq!(snafu_to_dec("12111"), 906);
        assert_eq!(snafu_to_dec("2=0="), 198);
        assert_eq!(snafu_to_dec("21"), 11);
        assert_eq!(snafu_to_dec("2=01"), 201);
        assert_eq!(snafu_to_dec("111"), 31);
        assert_eq!(snafu_to_dec("20012"), 1257);
        assert_eq!(snafu_to_dec("112"), 32);
        assert_eq!(snafu_to_dec("1=-1="), 353);
        assert_eq!(snafu_to_dec("1-12"), 107);
        assert_eq!(snafu_to_dec("12"), 7);
        assert_eq!(snafu_to_dec("1="), 3);
        assert_eq!(snafu_to_dec("122"), 37);
    }

    #[test]
    fn test_snafu_max_with_digits() {
        assert_eq!(snafu_max_with_digits(0), 0);
        assert_eq!(snafu_max_with_digits(1), 2);
        assert_eq!(snafu_max_with_digits(2), 12);
        assert_eq!(snafu_max_with_digits(3), 62);
        assert_eq!(snafu_max_with_digits(4), 312);
    }

    #[test]
    fn test_snafu_min_with_digits() {
        assert_eq!(snafu_min_with_digits(0), 0);
        assert_eq!(snafu_min_with_digits(1), -2);
        assert_eq!(snafu_min_with_digits(2), -12);
        assert_eq!(snafu_min_with_digits(3), -62);
        assert_eq!(snafu_min_with_digits(4), -312);
    }

    #[test]
    fn test_dec_to_snafu() {
        assert_eq!(dec_to_snafu(1, None), "1".to_string());
        assert_eq!(dec_to_snafu(2, None), "2".to_string());
        assert_eq!(dec_to_snafu(3, None), "1=".to_string());
        assert_eq!(dec_to_snafu(4, None), "1-".to_string());
        assert_eq!(dec_to_snafu(5, None), "10".to_string());
        assert_eq!(dec_to_snafu(6, None), "11".to_string());
        assert_eq!(dec_to_snafu(7, None), "12".to_string());
        assert_eq!(dec_to_snafu(8, None), "2=".to_string());
        assert_eq!(dec_to_snafu(9, None), "2-".to_string());
        assert_eq!(dec_to_snafu(10, None), "20".to_string());
        assert_eq!(dec_to_snafu(11, None), "21".to_string());
        assert_eq!(dec_to_snafu(12, None), "22".to_string());
        assert_eq!(dec_to_snafu(13, None), "1==".to_string());
        assert_eq!(dec_to_snafu(14, None), "1=-".to_string());
        assert_eq!(dec_to_snafu(15, None), "1=0".to_string());
        assert_eq!(dec_to_snafu(16, None), "1=1".to_string());
        assert_eq!(dec_to_snafu(17, None), "1=2".to_string());
        assert_eq!(dec_to_snafu(18, None), "1-=".to_string());
        assert_eq!(dec_to_snafu(19, None), "1--".to_string());
        assert_eq!(dec_to_snafu(20, None), "1-0".to_string());
        assert_eq!(dec_to_snafu(21, None), "1-1".to_string());
        assert_eq!(dec_to_snafu(22, None), "1-2".to_string());
        assert_eq!(dec_to_snafu(23, None), "10=".to_string());
        assert_eq!(dec_to_snafu(24, None), "10-".to_string());
        assert_eq!(dec_to_snafu(25, None), "100".to_string());
        assert_eq!(dec_to_snafu(1747, None), "1=-0-2".to_string());
        assert_eq!(dec_to_snafu(906, None), "12111".to_string());
        assert_eq!(dec_to_snafu(198, None), "2=0=".to_string());
        assert_eq!(dec_to_snafu(11, None), "21".to_string());
        assert_eq!(dec_to_snafu(201, None), "2=01".to_string());
        assert_eq!(dec_to_snafu(31, None), "111".to_string());
        assert_eq!(dec_to_snafu(1257, None), "20012".to_string());
        assert_eq!(dec_to_snafu(32, None), "112".to_string());
        assert_eq!(dec_to_snafu(353, None), "1=-1=".to_string());
        assert_eq!(dec_to_snafu(107, None), "1-12".to_string());
        assert_eq!(dec_to_snafu(7, None), "12".to_string());
        assert_eq!(dec_to_snafu(3, None), "1=".to_string());
        assert_eq!(dec_to_snafu(37, None), "122".to_string());
    }
}
