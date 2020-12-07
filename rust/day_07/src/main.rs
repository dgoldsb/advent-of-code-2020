use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn parse_lines() -> HashMap<String, HashSet<(String, usize)>> {
    let mut rules: HashMap<String, HashSet<(String, usize)>> = HashMap::new();

    // Define regular expressions.
    let first = Regex::new(r"(.+) bags contain (.+)\.").unwrap();
    let second = Regex::new(r"(\d+) ([\w\s]+) bags?").unwrap();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    // Iterate over all rules.
    for cap in first.captures_iter(&buffer) {
        let container: String = cap[1].parse().unwrap();
        let containeds: String = cap[2].parse().unwrap();

        for bag in second.captures_iter(&containeds) {
            let amount: usize = bag[1].parse().unwrap();
            let contained: String = bag[2].parse().unwrap();

            match rules.get_mut(&contained) {
                Some(s) => {
                    s.insert((container.clone(), amount));
                }
                None => {
                    let mut s = HashSet::new();
                    s.insert((container.clone(), amount));
                    rules.insert(contained, s);
                }
            };
        }
    }

    return rules;
}

fn part_a(inputs: &HashMap<String, HashSet<(String, usize)>>, target: &String) -> HashSet<String> {
    let mut possible = HashSet::new();

    match inputs.get(target) {
        Some(s) => {
            for container in s {
                possible.insert(container.0.clone());
                possible.extend(part_a(inputs, &container.0));
            }
        }
        None => {}
    };

    return possible;
}

fn main() {
    let inputs = parse_lines();
    println!("A: {}", part_a(&inputs, &"shiny gold".to_string()).len());
}
