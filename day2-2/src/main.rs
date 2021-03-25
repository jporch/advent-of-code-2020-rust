use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut validPasswords: u32 = 0;

    for l in &lines {
        let tokens: Vec<&str> = l.split(" ").collect();
        let positions: Vec<&str> = tokens[0].split("-").collect();
        let pos1 = positions[0].parse::<usize>().unwrap() - 1;
        let pos2 = positions[1].parse::<usize>().unwrap() - 1;
        let testCharVec: Vec<char> = tokens[1].chars().collect();
        let testChar = testCharVec[0];
        let searchString = tokens[2];
        if  checkPass(searchString, testChar, pos1, pos2) {
            validPasswords += 1;
        }
    }
    println!("Total valid: {}", validPasswords);
}

fn checkPass(search: &str, test: char, pos1: usize, pos2: usize) -> bool {
    let searchChars: Vec<char> = search.chars().collect();
    if (searchChars[pos1] == test && searchChars[pos2] != test) || (searchChars[pos1] != test && searchChars[pos2] == test) {
        return true;
    }

    return false;
}
