use std::io::stdin;
use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let cases = parse_input(&input);

    for (i, case) in cases.iter().enumerate() {
        let result = case
            .solve()
            .map(|t| t.to_string())
            .unwrap_or_else(|| "IMPOSSIBLE".into());

        println!("Case #{}: {}", i + 1, result);
    }
}

fn parse_input(input: &str) -> Vec<Case> {
    let mut lines = input.trim().lines();

    let num_cases: usize = lines.next().unwrap().parse().unwrap();

    let mut cases = Vec::with_capacity(num_cases);

    for _ in 0..num_cases {
        cases.push(lines.next().unwrap().parse().unwrap());
    }

    cases
}

#[derive(Debug, PartialEq)]
struct Case {
    x: i64,
    y: i64,
    route: Vec<Dir>,
}

impl Case {
    fn solve(&self) -> Option<usize> {
        for (time, &(x, y)) in self.tour_route().iter().enumerate() {
            let dist = (x.abs() + y.abs()) as usize;

            if dist <= time {
                return Some(time);
            }
        }

        None
    }

    fn tour_route(&self) -> Vec<(i64, i64)> {
        let mut x = self.x;
        let mut y = self.y;

        let mut steps = Vec::with_capacity(self.route.len() + 1);

        steps.push((x, y));

        for &dir in &self.route {
            match dir {
                Dir::N => y += 1,
                Dir::S => y -= 1,
                Dir::E => x += 1,
                Dir::W => x -= 1,
            }

            steps.push((x, y));
        }

        steps
    }
}

impl FromStr for Case {
    type Err = ParseIntError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();

        let x = parts.next().unwrap().parse()?;
        let y = parts.next().unwrap().parse()?;

        let route = parts.next().unwrap().chars().map(Dir::from_char).collect();

        Ok(Case { x, y, route })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn from_char(c: char) -> Self {
        match c {
            'N' => Dir::N,
            'S' => Dir::S,
            'E' => Dir::E,
            'W' => Dir::W,
            _ => panic!("Unexpected direction: {}", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_correct_answer_to_sample_input() {
        use Dir::*;

        let input = "
7
4 4 SSSS
3 0 SNSS
2 10 NSNNSN
0 1 S
2 7 SSSSSSSS
3 2 SSSW
4 0 NESW";

        let test_cases = parse_input(&input);

        assert_eq!(
            test_cases[1],
            Case {
                x: 3,
                y: 0,
                route: vec!(S, N, S, S)
            }
        );

        let results: Vec<_> = test_cases.iter().map(|case| case.solve()).collect();

        assert_eq!(
            results,
            vec![Some(4), None, None, Some(1), Some(5), Some(4), Some(4)]
        );
    }
}
