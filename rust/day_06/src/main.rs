use std::collections::HashSet;
use std::io::{self, Read};

fn parse_inputs() -> Vec<Vec<HashSet<char>>> {
    let mut vec = Vec::new();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    // Iterate over all groups.
    for raw_group in buffer.split("\n\n") {
        let mut group = Vec::new();

        for raw_answers in raw_group.split("\n") {
            let set: HashSet<char> = raw_answers.chars().collect();
            if set.len() > 0 {
                group.push(set);
            }
        }
        vec.push(group);
    }

    return vec;
}

fn distinct_in_group(group: &Vec<HashSet<char>>) -> usize {
    return group
        .iter()
        .fold(HashSet::new(), |acc, x| acc.union(x).map(|x| *x).collect())
        .len();
}

fn part_a(inputs: &Vec<Vec<HashSet<char>>>) -> usize {
    return inputs.iter().map(|g| distinct_in_group(g)).sum::<usize>();
}

fn overlap_in_group(group: &Vec<HashSet<char>>) -> usize {
    let mut group_iter = group.iter();
    let overlap: HashSet<char> = group_iter.next().unwrap().clone();
    return group_iter
        .fold(overlap, |acc, x| acc.intersection(&x).copied().collect())
        .len();
}

fn part_b(inputs: &Vec<Vec<HashSet<char>>>) -> usize {
    return inputs.iter().map(|g| overlap_in_group(g)).sum::<usize>();
}

fn main() {
    let inputs = parse_inputs();
    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(&inputs));
}
