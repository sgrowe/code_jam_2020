use std::collections::{BTreeMap, HashSet};
use std::io::stdin;
use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    let cases = parse_input(&input);

    for (i, case) in cases.iter().enumerate() {
        println!("Case #{}: {}", i + 1, case.solve());
    }
}

fn parse_input(input: &str) -> Vec<Case> {
    let mut lines = input.trim().lines();

    let num_cases: usize = lines.next().unwrap().parse().unwrap();

    let mut cases = Vec::with_capacity(num_cases);

    for _ in 0..num_cases {
        let upper_limit_exp = lines.next().unwrap().parse().unwrap();
        let base: u64 = 10;
        let upper_limit = base.pow(upper_limit_exp) - 1;

        let num_queries = 10000;
        let mut queries = Vec::with_capacity(num_queries);

        for _ in 0..num_queries {
            queries.push(lines.next().unwrap().parse().unwrap());
        }

        cases.push(Case {
            upper_limit,
            queries,
        });
    }

    cases
}

#[derive(Debug, PartialEq)]
struct Case {
    upper_limit: u64,
    queries: Vec<Query>,
}

impl Case {
    fn solve(&self) -> String {
        let mut mappings: BTreeMap<u64, HashSet<&Vec<char>>> = BTreeMap::new();

        self.queries
            .iter()
            .filter_map(|query| query.i.map(|i| (i, &query.response)))
            .for_each(|(i, chars)| match mappings.get(&i) {
                Some(_) => {
                    mappings.get_mut(&i).unwrap().insert(chars);
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(chars);
                    mappings.insert(i, set);
                }
            });

        let mut chars: BTreeMap<u64, char> = BTreeMap::new();

        for (i, pos_chars) in mappings {
            if i > 10 {
                break;
            }

            for c in pos_chars.iter().map(|chars| {
                if i == 10 && chars.len() > 1 {
                    chars[1]
                } else {
                    chars[0]
                }
            }) {
                let has_already = chars.iter().any(|(_, &ch)| ch == c);

                if !has_already {
                    let x = if i == 10 { 0 } else { i };

                    chars.insert(x, c);
                    break;
                }
            }
        }

        chars.values().collect()
    }
}

#[derive(Debug, PartialEq)]
struct Query {
    i: Option<u64>,
    response: Vec<char>,
}

impl FromStr for Query {
    type Err = ParseIntError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = input.split_whitespace();

        let q: i64 = parts.next().unwrap().parse()?;

        let i = match q {
            -1 => None,
            _ => Some(q as u64),
        };

        let response = parts.next().unwrap().chars().collect();

        Ok(Query { i, response })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn runs_example_correctly() {
        let input = read_to_string("src/input.txt").unwrap();

        let cases = parse_input(&input);

        let results: Vec<_> = cases.iter().map(|c| c.solve()).collect();

        assert_eq!(results, vec!["TPFOXLUSHB"]);
    }
}
