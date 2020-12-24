use aoc::parse_lines;
use std::collections::{HashMap, VecDeque};

struct Cups {
    cups: HashMap<usize, usize>,
}

impl Cups {
    fn from_deque(deque: &VecDeque<usize>) -> Cups {
        // A ghetto linked list with quick access to each item.
        // TODO: This could be an array as our indices are usize without gaps, would be faster as we skip hashing.
        let mut cups: HashMap<usize, usize> = HashMap::new();
        let len = deque.len();
        for i in 0..deque.len() {
            let cup = deque.get(i).unwrap().clone();
            let next = deque.get((i + 1) % len).unwrap().clone();
            cups.insert(cup, next);
        }
        return Cups { cups };
    }

    fn get_after(&self, value: usize) -> usize {
        return self.cups.get(&value).unwrap().clone();
    }

    fn remove_after(&mut self, value: usize) -> usize {
        let a = self.cups.get(&value).unwrap().clone();
        let b = self.cups.get(&a).unwrap().clone();

        // Update the record before.
        self.cups.insert(value, b);

        // Remove the record we are returning.
        self.cups.remove(&a);

        return a;
    }

    fn remove_three_after(&mut self, value: usize) -> (usize, usize, usize) {
        let a = self.cups.get(&value).unwrap().clone();
        let b = self.cups.get(&a).unwrap().clone();
        let c = self.cups.get(&b).unwrap().clone();
        let d = self.cups.get(&c).unwrap().clone();

        // Update the record before.
        self.cups.insert(value, d);

        // Remove the record we are returning.
        self.cups.remove(&a);

        return (a, b, c);
    }

    fn insert_after(&mut self, after: usize, value: usize) {
        let a = self.cups.get(&after).unwrap().clone();

        // Update the record before.
        self.cups.insert(after, value);
        self.cups.insert(value, a);
    }
}

fn parse_inputs() -> VecDeque<usize> {
    let mut queue = VecDeque::new();
    let input = parse_lines();
    let input_string = input.iter().next().unwrap();
    for c in input_string.chars() {
        queue.push_back(c.to_string().parse().unwrap());
    }
    return queue;
}

fn play_game(start: &VecDeque<usize>, moves: usize) -> Cups {
    let mut cups = Cups::from_deque(start);
    let max_cup = start.iter().max().unwrap().clone();

    // Now we play cups!
    let mut current_cup = start.get(0).unwrap().clone();
    for _ in 0..moves {
        // The crab picks up the three cups that are immediately clockwise of the current cup. They
        // are removed from the circle; cup spacing is adjusted as necessary to maintain the circle.
        let picked_up = cups.remove_three_after(current_cup);

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
        cups.insert_after(destination_cup, picked_up.2);
        cups.insert_after(destination_cup, picked_up.1);
        cups.insert_after(destination_cup, picked_up.0);

        // The crab selects a new current cup: the cup which is immediately clockwise of the current
        // cup.
        current_cup = cups.get_after(current_cup);
    }

    return cups;
}

fn pad_queue(input: &VecDeque<usize>, size: usize) -> VecDeque<usize> {
    let mut result = input.clone();
    for i in input.len()..size {
        result.push_back(i + 1);
    }
    return result;
}

fn get_result_a(mut cups: Cups) -> String {
    let mut result = "".to_string();
    while cups.cups.len() > 1 {
        result += &cups.remove_after(1).to_string();
    }
    return result;
}

fn get_result_b(cups: &Cups) -> usize {
    let first = cups.get_after(1);
    let second = cups.get_after(first);

    return first * second;
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", get_result_a(play_game(&inputs, 100)));
    let padded_queue = pad_queue(&inputs, 1000000);
    println!("B: {}", get_result_b(&play_game(&padded_queue, 10000000)));
}
