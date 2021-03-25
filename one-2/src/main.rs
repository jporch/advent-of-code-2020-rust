use std::fs;

fn main() {
    let mut vals: Vec<i32> = Vec::new();
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let tokens: Vec<&str> = contents.split("\n").collect();

    for t in &tokens {
        vals.push(t.parse::<i32>().unwrap());
    }
    vals.sort();

    let mut start = 1;
    let mut cur = vals[0];

    while start < vals.iter().count() {
        for i in start..vals.iter().count()-1 {
            for j in start+1..vals.iter().count() {
                if cur + vals[i] + vals[j] == 2020 {
                    println!("You win! {} * {} * {} = {}", cur, vals[i], vals[j], cur * vals[i] * vals[j]);
                    return;
                }
            }
        }
        cur = vals[start];
        start = start + 1;
    }
}
