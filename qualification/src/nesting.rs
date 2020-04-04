use std::cmp;
use std::io::stdin;
use std::io::BufRead;

fn main() {
    let stdin = stdin();
    let stdin_lock = stdin.lock();

    let mut lines = stdin_lock.lines();

    lines.next().unwrap().unwrap();

    for (i, line) in lines.enumerate() {
        println!("Case #{}: {}", i + 1, add_brackets(&line.unwrap()))
    }
}

fn add_brackets(input: &str) -> String {
    let mut digits = input.chars().peekable();

    let mut out = String::new();

    let mut open_brackets: i32 = 0;

    while let Some(c) = digits.next() {
        let x = c.to_digit(10).unwrap();

        let next_digit = digits
            .peek()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .unwrap_or_default();

        let to_open = x as i32 - open_brackets;
        let to_close = cmp::max(x as i32 - next_digit, 0);

        open_brackets += to_open - to_close;

        for _ in 0..to_open {
            out.push('(');
        }

        out.push(c);

        for _ in 0..to_close {
            out.push(')');
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0000", "0000")]
    #[test_case("101", "(1)0(1)")]
    #[test_case("111000", "(111)000")]
    #[test_case("1", "(1)")]
    #[test_case("021", "0((2)1)")]
    #[test_case("312", "(((3))1(2))")]
    #[test_case("4", "((((4))))")]
    #[test_case("221", "((22)1)")]
    fn adds_brackets_correctly(input: &str, expected: &str) {
        let actual = add_brackets(input);

        assert_eq!(actual, expected);
    }
}
