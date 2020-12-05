use std::collections::HashMap;
use hex;

fn main() {
    let input_raw = include_str!("../input.txt");
    println!("{}", part_1(input_raw));
    println!("{}", part_2(input_raw));
}

fn part_1(input: &str) -> usize {
    let valid_keys = [
        &&"ecl", &&"pid", &&"eyr", &&"hcl", &&"byr", &&"iyr", &&"hgt",
    ];
    let mut counter: usize = 0;
    let input: Vec<HashMap<&str, &str>> = input
        .split("\n\n")
        .map(|x: &str| {
            x.split_whitespace()
                .map(|y: &str| {
                    let mut tmp = y.split(":");
                    (tmp.next().unwrap(), tmp.next().unwrap())
                })
                .collect()
        })
        .collect();
    for i in &input {
        let keys: Vec<&&str> = i.keys().collect();
        if valid_keys.iter().all(|x| keys.contains(x)) {
            counter += 1;
        }
    }
    counter
}

fn part_2(input: &str) -> usize {
    let valid_keys = [
        &&"ecl", &&"pid", &&"eyr", &&"hcl", &&"byr", &&"iyr", &&"hgt"
    ];
    let mut counter: usize = 0;
    let input: Vec<HashMap<&str, &str>> = input
        .split("\n\n")
        .map(|x: &str| {
            x.split_whitespace()
                .map(|y: &str| {
                    let mut tmp = y.split(":");
                    (tmp.next().unwrap(), tmp.next().unwrap())
                })
                .collect()
        })
        .collect();
    for i in &input {
        if !valid_keys.iter().all(|x| i.contains_key(**x)) {
            continue;
        }
        let passport = Passport {
            byr: {
                match i["byr"].parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                }
            },
            iyr: {
                match i["iyr"].parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                }
            },
            eyr: {
                match i["eyr"].parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                }
            },
            pid: String::from(i["pid"]),
            hgt: String::from(i["hgt"]),
            hcl: String::from(i["hcl"]),
            ecl: String::from(i["ecl"]),
        };
        if !(passport.byr >= 1920 && passport.byr <= 2002) {
            continue;
        }
        if !(passport.iyr >= 2010 && passport.iyr <= 2020) {
            continue;
        }
        if !(passport.eyr >= 2020 && passport.eyr <= 2030) {
            continue;
        }
        let units: &str = &passport.hgt[passport.hgt.len() - 2..];
        match units {
            "in" => {
                if passport.hgt.len() != 4 {
                    continue;
                }
                let height = passport.hgt[..2].parse::<usize>().unwrap();
                if !(height >= 59 && height <= 76) {
                    continue;
                }
            },
            "cm" => {
                if passport.hgt.len() != 5 {
                    continue;
                }
                let height = passport.hgt[..3].parse::<usize>().unwrap();
                if !(height >= 150 && height <= 193) {
                    continue;
                }
            }
            _ => continue,
        }
        if !(passport.hcl.chars().nth(0).unwrap() == '#' && passport.hcl.len() == 7) {
            continue;
        }
        match hex::decode(&passport.hcl[1..7]) {
            Ok(_) => (),
            Err(_) => continue,
        }
        if !(["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&passport.ecl[..])) {
            continue;
        }
        if !(match passport.pid.parse::<usize>(){
            Ok(num) => num,
            Err(_) => continue,
        } <= 1000000000 && passport.pid.len() == 9) {
            continue;
        }
        counter += 1;
    }
    counter
}

struct Passport {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: String,
    pid: String,
    hcl: String,
    ecl: String,
}
