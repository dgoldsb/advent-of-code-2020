use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn parse_lines() -> (
    HashMap<String, HashSet<String>>,
    HashMap<String, HashSet<(String, usize)>>,
) {
    // Map the contained versus the container.
    let mut input_a: HashMap<String, HashSet<String>> = HashMap::new();
    // Map the container versus its contents.
    let mut input_b: HashMap<String, HashSet<(String, usize)>> = HashMap::new();

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

            match input_a.get_mut(&contained) {
                Some(s) => {
                    s.insert(container.clone());
                }
                None => {
                    let mut s = HashSet::new();
                    s.insert(container.clone());
                    input_a.insert(contained.clone(), s);
                }
            };

            match input_b.get_mut(&container) {
                Some(s) => {
                    s.insert((contained.clone(), amount));
                }
                None => {
                    let mut s = HashSet::new();
                    s.insert((contained.clone(), amount));
                    input_b.insert(container.clone(), s);
                }
            };
        }
    }

    return (input_a, input_b);
}

fn part_a(inputs: &HashMap<String, HashSet<String>>, target: &String) -> HashSet<String> {
    let mut possible = HashSet::new();

    match inputs.get(target) {
        Some(s) => {
            for container in s {
                possible.insert(container.clone());
                possible.extend(part_a(inputs, &container));
            }
        }
        None => {}
    };

    return possible;
}

fn part_b(inputs: &HashMap<String, HashSet<(String, usize)>>, container: &String) -> usize {
    let mut count = 0;

    match inputs.get(container) {
        Some(s) => {
            for contained in s {
                count += contained.1;
                count += contained.1 * part_b(inputs, &contained.0);
            }
        }
        None => {}
    };

    return count;
}

fn main() {
    let inputs = parse_lines();
    println!("A: {}", part_a(&inputs.0, &"shiny gold".to_string()).len());
    println!("B: {}", part_b(&inputs.1, &"shiny gold".to_string()));
}
