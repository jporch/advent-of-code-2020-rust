use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let passports: Vec<&str> = contents.split("\n\n").collect();
    println!("total passports: {}", passports.iter().count());

    let mut validPassports = 0;

    for p in &passports {
        if check_passport(p) {
            validPassports += 1;
        }
    }

    println!("Valid passports: {}", validPassports);
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
            if i[0] == *f {
                found = true;
            }

        }
        if found == false {
            return false;
        }
    }
    return true;
}
