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
                if found.contains_key(&c) {
                    found.insert(c, found[&c]+1);
                } else {
                    found.insert(c, 1);
                }
            }
        }
        let mut all: usize = 0;
        for f in found.values() {
            if *f == people.len() {
                all += 1;
            }
        }
        count += all;
    }
    println!("Count: {}", count)
}