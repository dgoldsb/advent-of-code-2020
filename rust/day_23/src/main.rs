use aoc::parse_lines;
use std::collections::VecDeque;

fn parse_inputs() -> VecDeque<usize> {
    let mut queue = VecDeque::new();
    let input = parse_lines();
    let input_string = input.iter().next().unwrap();
    for c in input_string.chars() {
        queue.push_back(c.to_string().parse().unwrap());
    }
    return queue;
}

fn find_cup_index(cups: &VecDeque<usize>, value: &usize) -> usize {
    for (i, c) in cups.iter().enumerate() {
        if c == value {
            return i;
        }
    }
    panic!("Cup not found, could not get index...")
}

fn play_game(start: &VecDeque<usize>, moves: usize) -> VecDeque<usize> {
    let mut cups = start.clone();
    let max_cup = start.iter().max().unwrap().clone();

    for _ in 0..moves {

        // The crab picks up the three cups that are immediately clockwise of the current cup. They
        // are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let current_cup = cups.pop_front().unwrap();
        let picked_up = (
            cups.pop_front().unwrap(),
            cups.pop_front().unwrap(),
            cups.pop_front().unwrap(),
        );

        // The crab selects a destination cup: the cup with a label equal to the current cup's label
        // minus one. If this would select one of the cups that was just picked up, the crab will
        // keep subtracting one until it finds a cup that wasn't just picked up. If at any point in
        // this process the value goes below the lowest value on any cup's label, it wraps around to
        // the highest value on any cup's label instead.
        let mut destination_cup = current_cup.clone();
        while (destination_cup == current_cup)
            || (picked_up.0 == destination_cup)
            || (picked_up.1 == destination_cup)
            || (picked_up.2 == destination_cup)
        {
            destination_cup = match destination_cup {
                1 => max_cup,
                _ => destination_cup - 1,
            };
        }

        // The crab places the cups it just picked up so that they are immediately clockwise of the
        // destination cup. They keep the same order as when they were picked up.
        let index = find_cup_index(&cups, &destination_cup) + 1;
        cups.insert(index, picked_up.2);
        cups.insert(index, picked_up.1);
        cups.insert(index, picked_up.0);

        // The crab selects a new current cup: the cup which is immediately clockwise of the current
        // cup.
        cups.push_back(current_cup);
    }
    return cups;
}

fn get_result_a(mut cups: VecDeque<usize>) -> String {
    while *cups.get(0).unwrap() != 1 {
        let cup = cups.pop_front().unwrap();
        cups.push_back(cup);
    }

    let mut result = "".to_string();
    cups.pop_front();
    while !cups.is_empty() {
        let cup = cups.pop_front().unwrap();
        result += &cup.to_string();
    }
    return result;
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", get_result_a(play_game(&inputs, 100)));
}
