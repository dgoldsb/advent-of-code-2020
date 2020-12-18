use aoc::parse_lines;
use regex::Regex;

fn solve_simple_eq(eq: &String) -> usize {
    println!("simple: {}", &eq);
    let mut eq_iter = eq.split(" ");
    let mut result: usize = eq_iter.next().unwrap().parse().unwrap();

    loop {
        match eq_iter.next() {
            Some(c) => match c {
                "+" => result += eq_iter.next().unwrap().parse::<usize>().unwrap(),
                "*" => result *= eq_iter.next().unwrap().parse::<usize>().unwrap(),
                _ => panic!("Unsupported operand!"),
            },
            _ => return result,
        }
    }
}

fn solve_eq(eq: &String) -> usize {
    let mut mut_eq = eq.clone();

    let nested_regex = Regex::new(r"\(([^\)^\(]+)\)").unwrap();
    loop {
        match nested_regex.captures_iter(&mut_eq).next() {
            Some(c) => {
                let sol = solve_simple_eq(&c[1].to_string());
                mut_eq = mut_eq.replace(&c[0], &sol.to_string());
            }
            None => return solve_simple_eq(&mut_eq),
        }
    }
}

fn part_a(eqs: &Vec<String>) -> usize {
    return eqs.iter().map(|e| solve_eq(e)).sum();
}

fn main() {
    let inputs = parse_lines();

    println!("A: {}", part_a(&inputs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            part_a(&vec![
                "2 * 3 + (4 * 5)".to_string(),
                "5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string(),
                "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string(),
                "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()
            ]),
            26335
        );
    }
}
