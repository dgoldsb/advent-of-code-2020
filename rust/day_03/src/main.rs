use aoc::parse_lines;

const TREE: char = '#';

fn slope_down(inputs: &Vec<String>, right: usize, down: usize) -> i128 {
    let mut counter = 0;

    for (i, line) in inputs.iter().enumerate() {
        if (i % down) == 0 {
            let j = i / down;
            let index = (j * right) % line.chars().count();
            let char_at_index = line.chars().nth(index).unwrap();
            if char_at_index == TREE {
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
