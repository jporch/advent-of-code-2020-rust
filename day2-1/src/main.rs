use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut validPasswords: u32 = 0;

    for l in &lines {
        let tokens: Vec<&str> = l.split(" ").collect();
        let numChars: Vec<&str> = tokens[0].split("-").collect();
        let minChar = numChars[0].parse::<u32>().unwrap();
        let maxChar = numChars[1].parse::<u32>().unwrap();
        let testCharVec: Vec<char> = tokens[1].chars().collect();
        let testChar = testCharVec[0];
        let searchString = tokens[2];
        let count = countChars(searchString, testChar);
        let mut pass = "";
        if  count >= minChar && count <= maxChar {
            validPasswords += 1;
            pass = "PASSED"
        }
        println!("{}: Need between {} and {} {}'s, found {}. {}", searchString, minChar, maxChar, testChar, count, pass);
    }
    println!("Total valid: {}", validPasswords);
}

fn countChars(search: &str, test: char) -> u32 {
    let searchChars: Vec<char> = search.chars().collect();
    let mut totalChars: u32 = 0;
    for s in searchChars {
        if s == test {
            totalChars += 1;
        }
    }
    return totalChars;
}
