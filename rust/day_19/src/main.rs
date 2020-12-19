use aoc::parse_lines;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Rule {
    Base(char),
    Concat(Vec<isize>),
    Split((Vec<isize>, Vec<isize>)),
}

fn cartesian_product(a: &Vec<String>, b: &Vec<String>, messages: &Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    for x in a {
        for y in b {
            let new_vec = x.clone() + y;
            if messages.iter().filter(|&m| m.contains(&new_vec)).count() > 0 {
                result.push(new_vec);
            }
        }
    }
    return result;
}

impl Rule {
    fn expand(
        &self,
        others: &HashMap<isize, Rule>,
        max_recursion_depth: &usize,
        recursion_depth: &usize,
        messages: &Vec<String>,
    ) -> Vec<String> {
        return match self {
            Rule::Base(c) => vec![c.clone().to_string()],
            Rule::Concat(v) => {
                let mut result = vec!["".to_string()];
                for rule_id in v {
                    result = cartesian_product(
                        &result,
                        &others[rule_id].expand(
                            others,
                            max_recursion_depth,
                            &recursion_depth,
                            messages,
                        ),
                        messages,
                    );
                }
                result
            }
            Rule::Split(t) => {
                let depth: usize;

                if t.1.contains(&8) || t.1.contains(&11) {
                    depth = recursion_depth + 1;
                } else {
                    depth = 0;
                }

                let mut first =
                    Rule::Concat(t.0.clone()).expand(others, max_recursion_depth, &depth, messages);
                if depth < *max_recursion_depth {
                    first.extend(Rule::Concat(t.1.clone()).expand(
                        others,
                        max_recursion_depth,
                        &depth,
                        messages,
                    ));
                }
                first
            }
        };
    }
}

fn solve_one(rules: &HashMap<isize, Rule>, message: String) -> bool {
    let whitelist: HashSet<String> = rules
        .get(&0)
        .unwrap()
        .expand(rules, &8, &0, &vec![message.clone()])
        .iter()
        .map(|m| m.clone())
        .collect();
    return whitelist.contains(&message);
}

fn solve(rules: &HashMap<isize, Rule>, messages: &Vec<String>) -> usize {
    return messages
        .iter()
        .filter(|&m| solve_one(rules, m.clone()))
        .count();
}

fn parse_inputs() -> (HashMap<isize, Rule>, Vec<String>) {
    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    let rule_regex_base = Regex::new(r"^(\d+): .([ab]).$").unwrap();
    let rule_regex_concat = Regex::new(r"^(\d+): ([\d\s]+)$").unwrap();
    let rule_regex_split = Regex::new(r"^(\d+): ([\d\s]+) \| ([\d\s]+)$").unwrap();
    let message_regex = Regex::new(r"^([ab]+)$").unwrap();

    for line in parse_lines() {
        if rule_regex_base.is_match(&line) {
            let c = rule_regex_base.captures_iter(&line).next().unwrap();
            rules.insert(
                c[1].parse().unwrap(),
                Rule::Base(c[2].chars().next().unwrap()),
            );
        }

        if rule_regex_concat.is_match(&line) {
            let c = rule_regex_concat.captures_iter(&line).next().unwrap();
            rules.insert(
                c[1].parse().unwrap(),
                Rule::Concat(
                    c[2].to_string()
                        .split(" ")
                        .map(|v| v.parse().unwrap())
                        .collect(),
                ),
            );
        }

        if rule_regex_split.is_match(&line) {
            let c = rule_regex_split.captures_iter(&line).next().unwrap();
            rules.insert(
                c[1].parse().unwrap(),
                Rule::Split((
                    c[2].to_string()
                        .split(" ")
                        .map(|v| v.parse().unwrap())
                        .collect(),
                    c[3].to_string()
                        .split(" ")
                        .map(|v| v.parse().unwrap())
                        .collect(),
                )),
            );
        }

        if message_regex.is_match(&line) {
            let c = message_regex.captures_iter(&line).next().unwrap();
            messages.push(c[1].to_string());
        }
    }

    return (rules, messages);
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", solve(&inputs.0, &inputs.1));

    let mut new_rules = inputs.0;
    new_rules.insert(8, Rule::Split((vec![42], vec![42, 8])));
    new_rules.insert(11, Rule::Split((vec![42, 31], vec![42, 11, 31])));

    println!("B: {}", solve(&new_rules, &inputs.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_a() {
        let mut rules = HashMap::new();
        rules.insert(0, Rule::Concat(vec![4, 1, 5]));
        rules.insert(1, Rule::Split((vec![2, 3], vec![3, 2])));
        rules.insert(2, Rule::Split((vec![4, 4], vec![5, 5])));
        rules.insert(3, Rule::Split((vec![4, 5], vec![5, 4])));
        rules.insert(4, Rule::Base('a'));
        rules.insert(5, Rule::Base('b'));

        let messages = vec![
            "ababbb".to_string(),
            "bababa".to_string(),
            "abbbab".to_string(),
            "aaabbb".to_string(),
            "aaaabbb".to_string(),
        ];

        assert_eq!(solve(&rules, &messages), 2);
    }
}
