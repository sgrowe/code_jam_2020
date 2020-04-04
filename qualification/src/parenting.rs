use std::io::stdin;
use std::io::Read;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let mut input = String::new();

    stdin().read_to_string(&mut input).unwrap();

    for (i, case) in parse_input(&input).iter().enumerate() {
        println!("Case #{}: {}", i + 1, case.assignments());
    }
}

fn parse_input(input: &str) -> Vec<Case> {
    let mut lines = input.trim().lines();

    let num_cases = lines.next().unwrap().parse().unwrap();

    let mut cases = Vec::with_capacity(num_cases);

    for _ in 0..num_cases {
        let num_activities = lines.next().unwrap().parse().unwrap();

        let mut case = Case {
            activities: Vec::with_capacity(num_activities),
        };

        for _ in 0..num_activities {
            let activity = lines.next().unwrap().parse().unwrap();

            case.activities.push(activity);
        }

        cases.push(case);
    }

    cases
}

struct Case {
    activities: Vec<Activity>,
}

impl Case {
    pub fn assignments(&self) -> String {
        let mut j_busy_until = 0;
        let mut c_busy_until = 0;

        let mut assignments = Vec::with_capacity(self.activities.len());

        let mut sorted_activities: Vec<_> = self.activities.iter().enumerate().collect();
        sorted_activities.sort_by_key(|&(_, a)| a.start);

        for (i, activity) in sorted_activities {
            if activity.start >= c_busy_until {
                assignments.push((i, 'C'));
                c_busy_until = activity.end;
            } else if activity.start >= j_busy_until {
                assignments.push((i, 'J'));
                j_busy_until = activity.end;
            } else {
                return "IMPOSSIBLE".into();
            }
        }

        assignments.sort_by_key(|&(i, _)| i);

        assignments.iter().map(|&(_, c)| c).collect()
    }
}

#[derive(Debug, Copy, Clone)]
struct Activity {
    start: usize,
    end: usize,
}

impl FromStr for Activity {
    type Err = ParseIntError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        let mut ints = input.split_whitespace().map(|n| n.parse());

        let start = ints.next().unwrap()?;
        let end = ints.next().unwrap()?;

        Ok(Activity { start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assigns_tasks_correctly() {
        let input = std::fs::read_to_string("src/parenting.txt").unwrap();

        let output: Vec<String> = parse_input(&input)
            .iter()
            .map(|case| case.assignments())
            .collect();

        assert_eq!(output, vec!["CJC", "IMPOSSIBLE", "JCCJJ", "CC"]);
    }
}
