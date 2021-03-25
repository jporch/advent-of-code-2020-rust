use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut rows: Vec<Vec<char>> = vec![];

    for l in &lines {
        rows.push(l.chars().collect());
    }

    let slope: [[usize; 2]; 5] = [ [1,1], [3,1], [5,1], [7,1], [1,2] ];
    let mut product = 1;

    for s in &slope {
        product *= checkRoute(&s, &rows);
    }
    println!("Final product: {}", product);


}

fn checkRoute(slope: &[usize; 2], map: &Vec<Vec<char>>) -> u32 {
    let mut r: usize = 0;
    let mut c: usize = 0;
    let mut trees = 0;

    while r < map.iter().count()-1 {
        c = (c + slope[0]) % map[0].iter().count();
        r = r + slope[1];
        if map[r][c] == '#' {
            trees += 1;
        }
    }
     println!("Hit {} trees", trees);
     return trees;
}
