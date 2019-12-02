use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn fuel_req(m: &i32) -> i32 {
    let f = (m / 3) - 2;
    if f > 0 {
        f
    } else {
        0
    }
}

fn fuel_req_2(m: &i32) -> i32 {
    let mut f = fuel_req(m);
    if f > 0 {
        f += fuel_req_2(&f);
    }
    f
}

pub fn main() {
    let lines = read_lines("1.txt").expect("file not found");
    let modules: Vec<i32> = lines
        .map(|x| x.unwrap().parse().unwrap())
        .collect();

    let part1: i32 = modules.iter().map(fuel_req).sum();
    println!("Part 1: {}", part1);

    let part2: i32 = modules.iter().map(fuel_req_2).sum();
    println!("Part 2: {}", part2);
}
