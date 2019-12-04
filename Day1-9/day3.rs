use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

struct Instruction {
    dx: i32,
    dy: i32,
    dist: i32,
}

fn parse_instruction(w: &String) -> Instruction {
    let (dx, dy) = match w.chars().nth(0).unwrap() {
        'U' => (0, 1),
        'D' => (0, -1),
        'L' => (-1, 0),
        'R' => (1, 0),
        _ => panic!("Invalid direction"),
    };
    let dist: i32 = w[1..].parse().unwrap();
    Instruction { dx, dy, dist }
}

fn wire_to_point_set(wire: &Vec<String>) -> (HashSet<(i32, i32)>, HashMap<(i32, i32), i32>) {
    let mut s = HashSet::new();
    let mut d: HashMap<(i32, i32), i32> = HashMap::new();

    let (mut x, mut y, mut steps) = (0, 0, 0);
    for w in wire {
        let i = parse_instruction(w);
        for _ in 0..i.dist {
            x += i.dx;
            y += i.dy;
            steps += 1;
            s.insert((x, y));
            if !d.contains_key(&(x, y)) {
                d.insert((x, y), steps);
            }
        }
    }
    (s, d)
}

fn main() -> io::Result<()> {
    let file = File::open("3.txt")?;
    let lines = io::BufReader::new(file).lines();

    let wires: Vec<Vec<String>> = lines
        .map(|l| l.unwrap().split(',').map(|x| x.to_string()).collect())
        .collect();

    let (w1, d1) = wire_to_point_set(&wires[0]);
    let (w2, d2) = wire_to_point_set(&wires[1]);

    let intersections = w1.intersection(&w2);

    let part1 = intersections
        .clone()
        .min_by_key(|(x, y)| x.abs() + y.abs())
        .unwrap();
    println!("Part 1: {}", part1.0.abs() + part1.1.abs());

    let part2 = intersections
        .min_by_key(|p| d1.get(p).unwrap() + d2.get(p).unwrap())
        .unwrap();
    println!(
        "Part 2: {}",
        d1.get(part2).unwrap() + d2.get(part2).unwrap()
    );

    Ok(())
}
