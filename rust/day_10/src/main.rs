use aoc::parse_ints;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref MEMORY: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
}

fn part_a(inputs: &Vec<isize>) -> usize {
    let mut cloned_inputs = inputs.clone();

    cloned_inputs.push(0);
    cloned_inputs.push(inputs.iter().max().unwrap() + 3);
    cloned_inputs.sort();

    let mut differences = Vec::new();
    for i in 1..cloned_inputs.len() {
        differences.push(cloned_inputs[i] - cloned_inputs[i - 1]);
    }

    return differences.iter().filter(|x| **x == 1).count()
        * differences.iter().filter(|x| **x == 3).count();
}

fn part_b(inputs: &Vec<isize>) -> usize {
    let mut cloned_inputs = inputs.clone();

    cloned_inputs.push(0);
    cloned_inputs.push(inputs.iter().max().unwrap() + 3);
    cloned_inputs.sort();

    // Create differences vectors.
    let mut differences = Vec::new();
    for i in 1..cloned_inputs.len() {
        differences.push(cloned_inputs[i] - cloned_inputs[i - 1]);
    }

    // Iterate over vector.
    let mut result = 1;
    for i in 0..differences.len() {
        // Skip if the pointer is not at a one.
        if differences[i] != 1 {
            continue;
        }

        // Skip if previous was also a one.
        if (i != 0) && (differences[i - 1] == 1) {
            continue;
        }

        // Find block size with iteration forward.
        let mut block_length: usize = 1;
        for j in i..differences.len() {
            if differences[j] != 1 {
                block_length = j - i;
                break;
            }
        }

        if block_length == 2 {
            result *= 2
        } else if block_length == 3 {
            result *= 4
        } else if block_length == 4 {
            result *= 7
        }
    }

    return result;
}

fn find_ways(inputs: &Vec<isize>, pointer: usize) -> usize {
    if MEMORY.lock().unwrap().contains_key(&pointer) {
        return MEMORY.lock().unwrap()[&pointer];
    }

    if pointer == 0 {
        return 1;
    }

    let mut ways = 0;

    for i in 1..(pointer + 1) {
        let difference: isize = inputs[pointer] - inputs[pointer - i];
        if difference < 4 {
            ways += find_ways(inputs, pointer - i);
        } else {
            break;
        }
    }

    MEMORY.lock().unwrap().insert(pointer, ways);
    return ways;
}

fn part_b_dynamic(inputs: &Vec<isize>) -> usize {
    let mut cloned_inputs = inputs.clone();

    cloned_inputs.push(0);
    cloned_inputs.push(inputs.iter().max().unwrap() + 3);
    cloned_inputs.sort();

    return find_ways(&cloned_inputs, cloned_inputs.len() - 1);
}

fn main() {
    let inputs = parse_ints();
    println!("A: {}", part_a(&inputs));
    println!("B: {}", part_b(&inputs));
    println!("B (dynamic programming): {}", part_b_dynamic(&inputs));
}
