use regex::Regex;
use std::io::{self, Read};

fn parse_inputs() -> Vec<isize> {
    let mut vec = Vec::new();
    let re = Regex::new(r"([-+]?\d+|x)\D?").unwrap();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    for cap in re.captures_iter(&buffer) {
        vec.push(cap[1].replace("x", "0").parse().unwrap());
    }

    return vec;
}

fn part_a(earliest_leave: &isize, bus_ids: &Vec<isize>) -> isize {
    let soonest = bus_ids
        .iter()
        .filter(|b| **b != 0)
        .map(|b| {
            (
                ((((*earliest_leave - (*earliest_leave % b)) / b) + 1) * b),
                b.clone(),
            )
        })
        .min()
        .unwrap();

    return soonest.1 * (soonest.0 - *earliest_leave);
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn part_b(bus_ids: &Vec<isize>) -> isize {
    let mut residues = Vec::new();
    let mut modulii = Vec::new();

    for (i, bus_id) in bus_ids.iter().enumerate() {
        if *bus_id == 0 {
            continue;
        }
        residues.push(((*bus_id - i as isize) % *bus_id) as i64);
        modulii.push(*bus_id as i64);
    }

    return chinese_remainder(&residues, &modulii).unwrap() as isize;
}

fn main() {
    let inputs = parse_inputs();

    println!("A: {}", part_a(&(inputs[0]), &inputs[1..].to_vec()));
    println!("B: {}", part_b(&inputs[1..].to_vec()));
}
