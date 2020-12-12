use regex::Regex;
use std::io::{self, Read};

const DIRS: [&'static char; 4] = [&'E', &'S', &'W', &'N'];

fn parse_inputs() -> Vec<(char, isize)> {
    let mut vec = Vec::new();
    let re = Regex::new(r"([NESWLRF])(\d+)").unwrap();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    for cap in re.captures_iter(&buffer) {
        vec.push((cap[1].parse().unwrap(), cap[2].parse().unwrap()));
    }

    return vec;
}

fn translate_inputs(inputs: &Vec<(char, isize)>) -> Vec<(isize, isize)> {
    let mut new_vec = Vec::new();

    let mut dir_pointer: usize = 0;

    for (ch, digits) in inputs {
        let mut direction = ch.clone();

        // Update the pointer and replace the direction.
        if direction == 'L' {
            let ticks: usize = (*digits / 90) as usize;
            dir_pointer = (dir_pointer + 4 - ticks) % 4;
            continue;
        } else if direction == 'R' {
            let ticks: usize = (*digits / 90) as usize;
            dir_pointer = (dir_pointer + ticks) % 4;
            continue;
        }

        // If we go forward, just replace the direction.
        if direction == 'F' {
            direction = DIRS[dir_pointer].clone();
        }

        // Push the newly improved (dx, dy)!
        if direction == 'N' {
            new_vec.push((*digits, 0));
        } else if direction == 'S' {
            new_vec.push((-*digits, 0));
        } else if direction == 'E' {
            new_vec.push((0, *digits));
        } else if direction == 'W' {
            new_vec.push((0, -*digits));
        }
    }

    return new_vec;
}

fn translate_waypoint_inputs(inputs: &Vec<(char, isize)>) -> Vec<(isize, isize)> {
    let mut new_vec = Vec::new();

    let mut waypoint: (isize, isize) = (1, 10);

    for (direction, digits) in inputs {
        // Rotate the waypoint.
        if *direction == 'L' {
            let ticks: usize = (*digits / 90) as usize;
            for _ in 0..ticks {
                waypoint = (waypoint.1, -waypoint.0)
            }
            continue;
        } else if *direction == 'R' {
            let ticks: usize = (*digits / 90) as usize;
            for _ in 0..ticks {
                waypoint = (-waypoint.1, waypoint.0)
            }
            continue;
        }

        // Move the waypoint.
        if *direction == 'N' {
            waypoint = (waypoint.0 + *digits, waypoint.1);
            continue;
        } else if *direction == 'S' {
            waypoint = (waypoint.0 - *digits, waypoint.1);
            continue;
        } else if *direction == 'E' {
            waypoint = (waypoint.0, waypoint.1 + *digits);
            continue;
        } else if *direction == 'W' {
            waypoint = (waypoint.0, waypoint.1 - *digits);
            continue;
        }

        // Move the ship.
        if *direction == 'F' {
            for _ in 0..*digits {
                new_vec.push(waypoint.clone());
            }
        }
    }

    return new_vec;
}

fn get_distance(inputs: &Vec<(char, isize)>, part_a: bool) -> isize {
    let translated: Vec<(isize, isize)>;
    if part_a {
        translated = translate_inputs(inputs);
    } else {
        translated = translate_waypoint_inputs(inputs);
    }
    return translated.iter().map(|t| t.0).sum::<isize>().abs()
        + translated.iter().map(|t| t.1).sum::<isize>().abs();
}

fn main() {
    let inputs = parse_inputs();
    println!("Part A: {}", get_distance(&inputs, true));
    println!("Part B: {}", get_distance(&inputs, false));
}
