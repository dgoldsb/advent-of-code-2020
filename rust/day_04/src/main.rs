use std::collections::HashMap;
use std::io::{self, Read};


fn parse_lines() -> Vec<HashMap<String, String>> {
    let mut vec = Vec::new();

    // Get the stdin and read it into a buffer.
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    match stdin.read_to_string(&mut buffer) {
        Ok(n) => println!("Parsed {}", n),
        Err(_) => panic!("Could not read from stdin"),
    };

    // Iterate over all passwords.
    for raw_passport in buffer.split("\n\n") {
        let mut passport = HashMap::new();

        for raw_entry in raw_passport.replace(" ", "\n").split("\n") {
            if raw_entry.len() == 0 {
                continue;
            }
            let split: Vec<&str> = raw_entry.split(":").collect();
            passport.insert(split[0].to_string(), split[1].to_string());
        }

        vec.push(passport);
    }

    return vec;
}

fn count_valid(passports: &Vec<HashMap<String, String>>) -> usize {
    let mut counter = 0;

    for passport in passports {
        if (passport.len() == 8) || (passport.len() == 7 && !passport.contains_key("cid")) {
            counter += 1;
        }
    }

    return counter;
}

fn count_valid_strict(passports: &Vec<HashMap<String, String>>) -> usize {
    let mut counter = 0;

    for passport in passports {
        if (passport.len() < 7) || !(passport.len() == 7 && !passport.contains_key("cid")) {
            // There are keys missing, no point in validating.
            continue;
        }

        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        {
            let year: i32 = passport["byr"].parse().unwrap();
            if year < 1920 || year > 2002 {
                continue;
            }
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        {
            let year: i32 = passport["iyr"].parse().unwrap();
            if year < 2010 || year > 2020 {
                continue;
            }
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        {
            let year: i32 = passport["eyr"].parse().unwrap();
            if year < 2020 || year > 2030 {
                continue;
            }
        }

        // hgt (Height) - a number followed by either cm or in:
        // If cm, the number must be at least 150 and at most 193.
        // If in, the number must be at least 59 and at most 76.

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if passport["pid"].len() != 9 {
            continue;
        }

        counter += 1;
    }

    return counter;
}

fn main() {
    let inputs = parse_lines();
    println!("A: {}", count_valid(&inputs));
    println!("B: {}", count_valid_strict(&inputs));
}
