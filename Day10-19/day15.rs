use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, Read};
use std::iter;
use std::path::Path;
use std::time::Instant;
use std::{thread, time};

#[derive(Debug)]
struct VM {
    state: Vec<i64>,
    pc: usize,
    input: Vec<i64>,
    output: Option<i64>,
    halted: bool,
    relative_base: i64,
    waiting_for_input: bool,
}

impl VM {
    fn next_value(&mut self) -> i64 {
        let v = self.state[self.pc];
        self.pc += 1;
        v
    }

    fn get_params(&mut self, code: i64, num_params: i64) -> Vec<i64> {
        let mut params = Vec::with_capacity(num_params as usize);
        for i in 0..num_params {
            let p = self.next_value();
            let mode = (code % 10_i64.pow(3 + i as u32)) / (10_i64.pow(2 + i as u32));
            match mode {
                // Position
                0 => params.push(self.state[p as usize]),
                // Immediate
                1 => params.push(p),
                // Relative
                2 => params.push(self.state[(p + self.relative_base) as usize]),
                _ => panic!("Invalid mode: {}", mode),
            }
            // println!("\t{} => {}", p, params.last().unwrap());
        }
        // println!("{} {:?}", code, params);
        params
    }

    fn get_write_location(&mut self, code: i64, num_params: i64) -> usize {
        let mode = (code % 10_i64.pow(2 + num_params as u32)) / (10_i64.pow(1 + num_params as u32));
        let p = self.next_value() as usize;
        let pos = match mode {
            // Position
            0 => p,
            // Relative
            2 => ((p as i64) + self.relative_base) as usize,
            _ => panic!("Invalid mode: {}", mode),
        };
        pos
    }

    fn run(&mut self) {
        while !self.halted {
            self.run_one();
        }
    }

    fn run_one(&mut self) {
        if self.halted || self.waiting_for_input {
            return;
        }

        // println!("---");
        // println!("instruction: {:?}", &self.state[self.pc..=self.pc+3]);
        // println!("relative base: {}", self.relative_base);
        let op_code = self.next_value();
        match op_code % 100 {
            // Add
            1 => {
                let params = self.get_params(op_code, 2);
                let pos = self.get_write_location(op_code, 3);
                self.set_state(pos, params[0] + params[1]);
            }
            // Mult
            2 => {
                let params = self.get_params(op_code, 2);
                let pos = self.get_write_location(op_code, 3);
                self.set_state(pos, params[0] * params[1]);
            }
            // Input
            3 => {
                let idx = self.get_write_location(op_code, 1);
                if !self.input.is_empty() {
                    let input = self.input.remove(0);
                    self.set_state(idx, input);
                } else {
                    self.pc -= 2;
                    self.waiting_for_input = true;
                }
            }
            // Output
            4 => {
                let params = self.get_params(op_code, 1);
                self.output = Some(params[0]);
                // println!("Output: {:?}", self.output);
            }
            // Jump-Non-Zero
            5 => {
                let params = self.get_params(op_code, 2);
                if params[0] != 0 {
                    self.pc = params[1] as usize;
                    // println!("jumped to {}", params[1]);
                }
            }
            // Jump-Eq-Zero
            6 => {
                let params = self.get_params(op_code, 2);
                if params[0] == 0 {
                    self.pc = params[1] as usize;
                    // println!("jumped to {}", params[1]);
                }
            }
            // Less Than
            7 => {
                let params = self.get_params(op_code, 2);
                let idx = self.get_write_location(op_code, 3);
                self.set_state(idx, if params[0] < params[1] { 1 } else { 0 });
            }
            // Equal
            8 => {
                let params = self.get_params(op_code, 2);
                let idx = self.get_write_location(op_code, 3);
                self.set_state(idx, if params[0] == params[1] { 1 } else { 0 });
            }
            // Adjust relative base
            9 => {
                let params = self.get_params(op_code, 1);
                self.relative_base += params[0];
                // println!("new relative base: {}", self.relative_base);
            }
            // Halt
            99 => self.halted = true,
            _ => panic!("Unknown opcode: {}", op_code % 100),
        }
    }

    fn run_until_output(&mut self) -> Option<i64> {
        while !self.halted && !self.waiting_for_input {
            self.run_one();
            if let Some(output) = self.output {
                self.output = None;
                return Some(output);
            }
        }
        None
    }

    fn new(initial_state: Vec<i64>) -> VM {
        VM {
            state: initial_state,
            pc: 0,
            output: None,
            input: Vec::new(),
            halted: false,
            relative_base: 0,
            waiting_for_input: false,
        }
    }

    pub fn set_state(&mut self, pc: usize, val: i64) {
        self.state[pc as usize] = val;
        // println!("set location {} to value {}", pc, val);
    }

    pub fn push_input(&mut self, input: i64) {
        self.input.push(input);
        self.waiting_for_input = false;
    }
}

#[derive(std::cmp::PartialEq, Clone, Copy)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Oxygen = 2,
}

impl Tile {
    pub fn from_int(i: i64) -> Tile {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Oxygen,
            _ => panic!("Invalid tile type"),
        }
    }

    pub fn string_repr(&self) -> String {
        match self {
            Tile::Empty => String::from(" "),
            Tile::Wall => String::from("#"),
            Tile::Oxygen => String::from("O"),
        }
    }
}

#[derive(std::cmp::PartialEq, Clone, Copy)]
enum StatusCode {
    HitWall = 0,
    Moved = 1,
    FoundOxygen = 2,
}

impl StatusCode {
    pub fn from_int(i: i64) -> StatusCode {
        match i {
            0 => StatusCode::HitWall,
            1 => StatusCode::Moved,
            2 => StatusCode::FoundOxygen,
            _ => panic!("Invalid status code"),
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    pub fn from_int(i: i64) -> Direction {
        match i {
            1 => Direction::North,
            2 => Direction::South,
            3 => Direction::West,
            4 => Direction::East,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn apply(&self, x: i64, y: i64) -> (i64, i64) {
        match self {
            Direction::North => (x, y + 1),
            Direction::South => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
        }
    }

    pub fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn right(&self) -> Direction {
        return self.left().left().left();
    }
}

fn read_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut out = String::new();
    file.read_to_string(&mut out)?;
    Ok(out)
}

struct Robot {
    x: i64,
    y: i64,
    grid: HashMap<(i64, i64), Tile>,
    vm: VM,
    oxygen_location: (i64, i64),
}

impl Robot {
    fn new(mut initial_state: Vec<i64>) -> Robot {
        let mut grid = HashMap::new();
        grid.insert((0, 0), Tile::Empty);
        Robot {
            x: 0,
            y: 0,
            grid: grid,
            vm: VM::new(initial_state),
            oxygen_location: (-1, -1),
        }
    }

    fn run_move(&mut self, d: Direction) -> StatusCode {
        self.vm.push_input(d as i64);
        let code = StatusCode::from_int(self.vm.run_until_output().unwrap());
        let (nx, ny) = d.apply(self.x, self.y);
        match code {
            StatusCode::Moved => {
                self.x = nx;
                self.y = ny;
                self.grid.insert((nx, ny), Tile::Empty);
            }
            StatusCode::FoundOxygen => {
                self.x = nx;
                self.y = ny;
                self.grid.insert((nx, ny), Tile::Oxygen);
            }
            StatusCode::HitWall => {
                self.grid.insert((nx, ny), Tile::Wall);
            }
        }
        code
    }

    fn explore(&mut self, steps: i64) {
        while self.run_move(Direction::North) != StatusCode::HitWall {}

        let mut counter = 0;
        let mut d = Direction::East;
        while counter < steps {
            counter += 1;
            if self.run_move(d.right()) != StatusCode::HitWall {
                d = d.right();
            } else if self.run_move(d) == StatusCode::HitWall {
                d = d.left();
            }
        }
    }

    fn render(&self) {
        let max_x = *self.grid.keys().map(|(x, _)| x).max().unwrap();
        let max_y = *self.grid.keys().map(|(_, y)| y).max().unwrap();
        let min_x = *self.grid.keys().map(|(x, _)| x).min().unwrap();
        let min_y = *self.grid.keys().map(|(_, y)| y).min().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if x == self.x && y == self.y {
                    print!("X");
                    continue;
                }
                print!(
                    "{}",
                    self.grid
                        .get(&(x, y))
                        .map(|c| c.string_repr())
                        .unwrap_or(" ".to_string())
                )
            }
            println!();
        }
    }
}

fn bfs_to_oxygen(grid: &HashMap<(i64, i64), Tile>) -> i64 {
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    let mut discovered: HashSet<(i64, i64)> = HashSet::new();
    let mut distances: HashMap<(i64, i64), i64> = HashMap::new();
    discovered.insert((0, 0));
    queue.push_back((0, 0));
    distances.insert((0, 0), 0);
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        if *grid.get(&v).unwrap() == Tile::Oxygen {
            return *distances.get(&v).unwrap();
        }
        for npos in vec![
            (v.0 + 1, v.1),
            (v.0 - 1, v.1),
            (v.0, v.1 - 1),
            (v.0, v.1 + 1),
        ] {
            if *grid.get(&npos).unwrap_or(&Tile::Wall) == Tile::Wall {
                continue;
            } else if discovered.contains(&npos) {
                continue;
            }
            discovered.insert(npos);
            queue.push_back(npos);
            distances.insert(npos, distances.get(&v).unwrap_or(&0) + 1);
        }
    }
    -1
}

fn oxygen_spread_time(grid: &HashMap<(i64, i64), Tile>) -> i64 {
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();
    let mut discovered: HashSet<(i64, i64)> = HashSet::new();
    let mut distances: HashMap<(i64, i64), i64> = HashMap::new();

    let start_pos = *grid
        .iter()
        .filter(|(_, v)| **v == Tile::Oxygen)
        .map(|(k, _)| k)
        .next()
        .unwrap();

    discovered.insert(start_pos);
    queue.push_back(start_pos);
    distances.insert(start_pos, 0);
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        for npos in vec![
            (v.0 + 1, v.1),
            (v.0 - 1, v.1),
            (v.0, v.1 - 1),
            (v.0, v.1 + 1),
        ] {
            if *grid.get(&npos).unwrap_or(&Tile::Wall) == Tile::Wall {
                continue;
            } else if discovered.contains(&npos) {
                continue;
            }
            discovered.insert(npos);
            queue.push_back(npos);
            distances.insert(npos, distances.get(&v).unwrap_or(&0) + 1);
        }
    }
    *distances.values().max().unwrap()
}

pub fn main() {
    let mut input: Vec<i64> = read_file("15.txt")
        .expect("file doesn't exist")
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    input.extend(iter::repeat(0).take(10000));

    let mut r = Robot::new(input.to_vec());
    r.explore(2000);
    r.render();
    println!("Part 1: {}", bfs_to_oxygen(&r.grid));
    println!("Part 2: {}", oxygen_spread_time(&r.grid));
}
