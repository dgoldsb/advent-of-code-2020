use aoc::parse_lines;
use regex::Regex;
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Clone, Debug, Eq, PartialEq)]
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

    let rule_regex = Regex::new(r"([\s\w]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
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

fn find_valid_rule(
    rules: &Vec<Rule>,
    tickets: &Vec<Vec<usize>>,
    index: usize,
) -> Result<Rule, usize> {
    let mut valid_rules = Vec::new();
    for rule in rules {
        if tickets
            .iter()
            .filter(|t| !rule.is_valid(t.get(index).unwrap()))
            .count()
            == 0
        {
            valid_rules.push(rule.clone());
        }
    }

    match valid_rules.len() {
        1 => Ok(valid_rules.get(0).unwrap().clone()),
        _ => Err(valid_rules.len()),
    }
}

fn part_2(rules: &Vec<Rule>, tickets: &Vec<Vec<usize>>) -> usize {
    let mut result: usize = 1;
    let my_ticket = tickets.iter().next().expect("Must supply tickets!");
    let valid_tickets = tickets
        .iter()
        .filter(|t| find_ticket_error_rate(rules, t) == 0)
        .map(|t| t.clone())
        .collect();
    let mut column_indices: HashSet<usize> = (0..my_ticket.len()).collect();
    let mut unmapped_rules = (*rules).clone();

    loop {
        for index in column_indices.clone() {
            match find_valid_rule(&unmapped_rules, &valid_tickets, index) {
                Ok(r) => {
                    if r.field.contains("departure") {
                        result *= *my_ticket.get(index).unwrap();
                    }

                    column_indices.remove(&index);
                    unmapped_rules.remove(
                        unmapped_rules
                            .iter()
                            .position(|p| *p == r)
                            .expect("Rule should be in unmapped rules"),
                    );
                    break;
                }
                Err(_) => continue,
            }
        }

        if column_indices.len() == 0 {
            break;
        }
    }

    return result;
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", find_error_rate(&inputs.0, &inputs.1));
    println!("B: {}", part_2(&inputs.0, &inputs.1));
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
