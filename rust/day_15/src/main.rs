use aoc::parse_ints;
use std::collections::HashMap;

fn play_game(inputs: &Vec<isize>, duration: usize) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut last_number: usize = *inputs
        .get(0)
        .expect("Input should have length greater than zero")
        as usize;

    for i in 1..duration {
        let next_number = match inputs.get(i) {
            Some(m) => *m as usize,
            None => match memory.get(&last_number) {
                Some(n) => i - *n,
                None => 0,
            },
        };
        memory.insert(last_number, i);
        last_number = next_number;
    }

    return last_number;
}

fn main() {
    let inputs = parse_ints();

    println!("A: {}", play_game(&inputs, 2020));
    println!("B: {}", play_game(&inputs, 30000000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(play_game(&vec![0, 3, 6], 2020), 436);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(play_game(&vec![0, 3, 6], 30000000), 175594);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(play_game(&vec![1, 3, 2], 30000000), 2578);
    }
}
