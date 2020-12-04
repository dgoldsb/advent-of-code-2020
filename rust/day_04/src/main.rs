use regex::Regex;
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

        passport.insert("cid".to_string(), "foo".to_string());
        vec.push(passport);
    }

    return vec;
}

fn count_valid(passports: &Vec<HashMap<String, String>>) -> usize {
    let mut counter = 0;

    for passport in passports {
        if passport.len() == 8 {
            counter += 1;
        }
    }

    return counter;
}

fn count_valid_strict(passports: &Vec<HashMap<String, String>>) -> usize {
    let mut counter = 0;

    let hcl = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let ecl = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    let pid = Regex::new(r"^[0-9]{9}$").unwrap();

    for passport in passports {
        if passport.len() < 8 {
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
        if passport["hgt"].contains("cm") {
            let height: usize = passport["hgt"].replace("cm", "").parse().unwrap();
            if height < 150 || height > 193 {
                continue;
            }
        }
        // If in, the number must be at least 59 and at most 76.
        if passport["hgt"].contains("in") {
            let height: usize = passport["hgt"].replace("in", "").parse().unwrap();
            if height < 59 || height > 76 {
                continue;
            }
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if !hcl.is_match(&passport["hcl"]) {
            continue;
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if !ecl.is_match(&passport["ecl"]) {
            continue;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if !pid.is_match(&passport["pid"]) {
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
