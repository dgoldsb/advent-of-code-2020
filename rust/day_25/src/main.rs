use aoc::parse_lines;

const CHRISTMAS: usize = 20201227;
const SUBJECT_NUMBER: usize = 7;

fn find_loop_size(pk: &usize) -> usize {
    let mut loop_size = 0;
    let mut value = 1;
    loop {
        value = (value * SUBJECT_NUMBER) % CHRISTMAS;
        loop_size += 1;

        if value == *pk {
            break;
        }
    }
    return loop_size;
}

fn find_encryption_key(sn: &usize, ls: &usize) -> usize {
    let mut value = 1;
    for _ in 0..*ls {
        value = (value * sn) % CHRISTMAS;
    }
    return value;
}

fn get_encryption_key(pk_a: &usize, pk_b: &usize) -> usize {
    let ls_a = find_loop_size(pk_a);
    return find_encryption_key(pk_b, &ls_a);
}

fn main() {
    let inputs = parse_lines();
    let mut inputs_iter = inputs.iter();

    let pk_a: usize = inputs_iter.next().unwrap().parse().unwrap();
    let pk_b: usize = inputs_iter.next().unwrap().parse().unwrap();

    println!("A: {}", get_encryption_key(&pk_a, &pk_b));
}
