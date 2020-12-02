use std::io::{self, BufRead};


fn parse_integers() -> Vec<i32> {
    let mut vec = Vec::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Could not read line from standard in");
        vec.push(line.parse::<i32>().unwrap());
    }

    return vec;
}


fn part_a(inputs: &Vec<i32>) -> i32 {
    for x in inputs {
        for y in inputs {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    return -1;
}


fn part_b(inputs: &Vec<i32>) -> i32 {
    for x in inputs {
        for y in inputs {
            for z in inputs {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    return -1;
}


fn main() {
    let inputs = parse_integers();
    println!("A: {:?}", part_a(&inputs));
    println!("B: {:?}", part_b(&inputs));
}
