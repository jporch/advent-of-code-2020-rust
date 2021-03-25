use std::fs;

extern crate regex;

use regex::Regex;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let passports: Vec<&str> = contents.split("\n\n").collect();
    println!("total passports: {}", passports.iter().count());

    let mut valid_passports = 0;

    for p in &passports {
        if check_passport(p) {
            valid_passports += 1;
        }
    }

    println!("Valid passports: {}", valid_passports);
}

fn check_passport(passport: &str) -> bool {
    let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let mut data: Vec<&str> = vec![];


    let lines: Vec<&str> = passport.split("\n").collect();
    for l in &lines {
        let tokens: Vec<&str> = l.split(" ").collect();
        for t in &tokens  {
            data.push(t);
        }
    }

    for f in &fields {
        let mut found = false;
        for d in &data {
            let i: Vec<&str> = d.split(":").collect();
            if i[0] == *f && validate_field(i[0], i[1]) {
                found = true;
            }

        }
        if found == false {
            return false;
        }
    }
    return true;
}

fn validate_field(key: &str, val: &str) -> bool {
    match key {
        "byr" => {
            let pass = (*val).parse::<i32>().unwrap() >= 1920 && (*val).parse::<i32>().unwrap() <= 2002;
            //println!("key: {}, val:{}, pass:{}", key, val, pass);
            return pass;
        },
        "iyr" => {
            let pass = (*val).parse::<i32>().unwrap() >= 2010 && (*val).parse::<i32>().unwrap() <= 2020;
            //println!("key: {}, val:{}, pass:{}", key, val, pass);
            return pass;
        },
        "eyr" => {
            let pass = (*val).parse::<i32>().unwrap() >= 2020 && (*val).parse::<i32>().unwrap() <= 2030;
            //println!("key: {}, val:{}, pass:{}", key, val, pass);
            return pass;
        },
        "hgt" => {
            let reg = Regex::new(r"^(\d{2,3})(cm|in)$").unwrap();
            if reg.is_match(val) {
                let params = reg.captures(val).unwrap();
                if &params[2] == "in" {
                    let size = params[1].parse::<i32>().unwrap();
                    return size >= 59 && size <= 76;
                }
                if &params[2] == "cm" {
                    let size = params[1].parse::<i32>().unwrap();
                    return size >= 150 && size <= 193;
                }
                //println!("key: {}, val:{}, size:{}{}", key, val, &params[1], &params[2]);
            }
            return false;
        },
        "hcl" => {
            let reg = Regex::new(r"^#[\d|a|b|c|d|e|f]{6}$").unwrap();
            let pass = reg.is_match(val);
            //println!("key: {}, val:{}, pass:{}", key, val, pass);
            return pass;
        },
        "ecl" => {
            let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            let pass = valid.contains(&val);
            //println!("key: {}, val:{}, pass:{}", key, val, pass);
            return pass;
        },
        "pid" => {
            let reg = Regex::new(r"^\d{9}$").unwrap();
            let pass = reg.is_match(val);
            //println!("key: {}, val:{}, pass:{}", key, val, pass);
            return pass;
        },
        _ => return true,
    }
}
