use std::io::{self, BufRead};

fn parse_lines() -> Vec<String> {
    let mut vec = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        vec.push(line);
    }

    return vec;
}

fn slope_down(inputs: &Vec<String>, right: usize, down: usize) -> i128 {
    let mut counter = 0;
    let tree: char = "#".chars().next().unwrap();

    for (i, line) in inputs.iter().enumerate() {
        if (i % down) == 0 {
            let j = i / down;
            let index = (j * right) % line.chars().count();
            let char_at_index = line.chars().nth(index).unwrap();
            if char_at_index == tree {
                counter += 1;
            }
        }
    }
    return counter;
}

fn main() {
    let inputs = parse_lines();
    println!("A: {}", slope_down(&inputs, 3, 1));
    let answer_b = slope_down(&inputs, 1, 1)
        * slope_down(&inputs, 3, 1)
        * slope_down(&inputs, 5, 1)
        * slope_down(&inputs, 7, 1)
        * slope_down(&inputs, 1, 2);
    println!("B: {}", answer_b);
}
