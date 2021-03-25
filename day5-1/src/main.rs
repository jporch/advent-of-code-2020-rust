use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let passes: Vec<&str> = contents.split("\n").collect();
    let mut ids: Vec<u32> = vec![];
    for p in &passes {
        ids.push(parse_pass(p));
    }
    ids.sort();
    for i in &ids {
        println!("{}", i)
    }
    println!("total passes: {}", passes.iter().count());
}

fn parse_pass(pass: &str) -> u32 {
    let mut bin_pass = pass.replace("B", "1");
    bin_pass = bin_pass.replace("F", "0");
    bin_pass = bin_pass.replace("L", "0");
    bin_pass = bin_pass.replace("R", "1");
    let id = u32::from_str_radix(&bin_pass, 2).unwrap();
    println!("{}: {}", bin_pass, id);
    return id;
}