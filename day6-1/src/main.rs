use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let groups: Vec<&str> = contents.split("\n\n").collect();
    let mut count: usize = 0;

    for g in &groups {
        let people: Vec<&str> = g.split("\n").collect();
        let mut found = HashMap::new();

        for p in &people {
            let cha: Vec<char> = p.chars().collect();
            for c in cha {
                found.insert(c, true);
            }
        }
        count += found.keys().count();
    }
    println!("Count: {}", count)
}