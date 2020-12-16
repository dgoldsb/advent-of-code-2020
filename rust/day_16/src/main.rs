use aoc::parse_lines;
use regex::Regex;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Rule {
    field: String,
    lower: RangeInclusive<usize>,
    upper: RangeInclusive<usize>,
}

impl Rule {
    fn is_valid(&self, value: &usize) -> bool {
        return self.lower.contains(value) || self.upper.contains(value);
    }
}

fn parse_inputs() -> (Vec<Rule>, Vec<Vec<usize>>) {
    let mut rules = Vec::new();
    let mut tickets = Vec::new();

    let rule_regex = Regex::new(r"(\w+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let int_regex = Regex::new(r"(\d+)\D?").unwrap();

    for line in parse_lines() {
        if rule_regex.is_match(&line) {
            rules.push(
                rule_regex
                    .captures_iter(&line)
                    .map(|c| Rule {
                        field: c[1].to_string(),
                        lower: (c[2].parse().unwrap())..=(c[3].parse().unwrap()),
                        upper: (c[4].parse().unwrap())..=(c[5].parse().unwrap()),
                    })
                    .next()
                    .unwrap(),
            );
        } else if int_regex.is_match(&line) {
            tickets.push(
                int_regex
                    .captures_iter(&line)
                    .map(|c| c[1].parse().unwrap())
                    .collect(),
            );
        }
    }

    return (rules, tickets);
}

fn find_field_error_rate(rules: &Vec<Rule>, field: &usize) -> usize {
    return match rules
        .iter()
        .map(|r| r.is_valid(field))
        .filter(|b| *b)
        .count()
    {
        0 => field.clone(),
        _ => 0,
    };
}

fn find_ticket_error_rate(rules: &Vec<Rule>, ticket: &Vec<usize>) -> usize {
    return ticket.iter().map(|f| find_field_error_rate(rules, f)).sum();
}

fn find_error_rate(rules: &Vec<Rule>, tickets: &Vec<Vec<usize>>) -> usize {
    return tickets
        .iter()
        .map(|t| find_ticket_error_rate(rules, t))
        .sum();
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", find_error_rate(&inputs.0, &inputs.1));
    println!("B: {}", 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            find_error_rate(
                &vec![
                    Rule {
                        field: "class".to_string(),
                        lower: 1..=3,
                        upper: 5..=7
                    },
                    Rule {
                        field: "row".to_string(),
                        lower: 6..=11,
                        upper: 33..=44
                    },
                    Rule {
                        field: "seat".to_string(),
                        lower: 13..=40,
                        upper: 45..=50
                    }
                ],
                &vec![
                    vec![7, 1, 14],
                    vec![7, 3, 47],
                    vec![40, 4, 50],
                    vec![55, 2, 20],
                    vec![38, 6, 12]
                ]
            ),
            71
        );
    }
}
