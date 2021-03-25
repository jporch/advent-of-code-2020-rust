use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut rows: Vec<Vec<char>> = vec![];

    for l in &lines {
        rows.push(l.chars().collect());
    }

    let slope = [3,1];

    let mut r = 0;
    let mut c = 0;
    let mut trees = 0;

    while r < lines.iter().count()-1 {
        c = (c + slope[0]) % rows[0].iter().count();
        r = r + slope[1];
        if rows[r][c] == '#' {
            trees += 1;
        }
    }
     println!("Hit {} trees", trees);
}
