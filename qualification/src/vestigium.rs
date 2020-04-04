use std::collections::HashSet;
use std::io::stdin;
use std::io::Read;

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let cases = parse_input(&input);

    for line in output(&cases) {
        println!("{}", line);
    }
}

fn parse_input(input: &str) -> Vec<Case> {
    let mut lines = input.trim().lines();
    let num_cases: usize = lines.next().unwrap().parse().unwrap();

    let mut cases = Vec::with_capacity(num_cases);

    while let Some(line) = lines.next() {
        let size = line.parse().unwrap();
        let mut rows = Vec::with_capacity(size);

        for _ in 0..size {
            let row = lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            rows.push(row);
        }

        cases.push(Case { size, rows });
    }

    cases
}

fn output(cases: &[Case]) -> Vec<String> {
    cases
        .iter()
        .enumerate()
        .map(|(index, case)| {
            format!(
                "Case #{}: {} {} {}",
                index + 1,
                case.trace(),
                case.rows_with_repeated_elements(),
                case.cols_with_repeated_elements()
            )
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct Case {
    size: usize,
    rows: Vec<Vec<usize>>,
}

impl Case {
    pub fn trace(&self) -> usize {
        let mut sum = 0;

        for i in 0..self.size {
            sum += self.rows[i][i];
        }

        sum
    }

    pub fn rows_with_repeated_elements(&self) -> usize {
        self.rows.iter().filter(|row| has_repeats(&row)).count()
    }

    fn cols(&self) -> Vec<Vec<usize>> {
        (0..self.size)
            .map(move |col| (0..self.size).map(|row| self.rows[row][col]).collect())
            .collect()
    }

    pub fn cols_with_repeated_elements(&self) -> usize {
        self.cols().iter().filter(|row| has_repeats(&row)).count()
    }
}

fn has_repeats(items: &[usize]) -> bool {
    let mut seen = HashSet::with_capacity(items.len());

    for x in items {
        let already_seen = !seen.insert(x);

        if already_seen {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn parses_example_input() {
        let input = read_to_string("src/example.txt").unwrap();

        let cases = parse_input(&input);

        assert_eq!(cases.len(), 3);

        assert_eq!(
            cases[0],
            Case {
                size: 4,
                rows: vec!(
                    vec!(1, 2, 3, 4),
                    vec!(2, 1, 4, 3),
                    vec!(3, 4, 1, 2),
                    vec!(4, 3, 2, 1),
                )
            }
        );
    }

    #[test]
    fn gives_right_output_for_example() {
        let input = read_to_string("src/example.txt").unwrap();

        let cases = parse_input(&input);

        let output: Vec<String> = output(&cases);

        assert_eq!(
            output,
            vec!("Case #1: 4 0 0", "Case #2: 9 4 4", "Case #3: 8 0 2")
        );
    }
}
