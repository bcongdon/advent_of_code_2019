use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::iter;
use std::path::Path;
use std::{thread, time};

const INTERACTIVE: bool = false;

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

fn read_file<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename)?;
    let mut out = String::new();
    file.read_to_string(&mut out)?;
    Ok(out)
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Up => "U".to_string(),
            Direction::Left => "L".to_string(),
            Direction::Down => "D".to_string(),
            Direction::Right => "R".to_string(),
        }
    }
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Direction {
        self.left().left().left()
    }

    fn offset(&self) -> (i32, i32) {
        match self {
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Left => (0, -1),
            Direction::Down => (1, 0),
        }
    }
}

pub fn main() {
    let mut input: Vec<i64> = read_file("17.txt")
        .expect("file doesn't exist")
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    input.extend(iter::repeat(0).take(10000));

    let mut output = String::new();
    let mut vm = VM::new(input.to_vec());
    while !vm.halted {
        if let Some(o) = vm.run_until_output() {
            output.push(o as u8 as char);
        }
    }

    // println!("{}", output);
    let mut grid: Vec<Vec<char>> = output
        .trim()
        .lines()
        .map(|x| x.trim().to_string().chars().collect())
        .collect();
    let mut part1 = 0;
    let (mut sx, mut sy) = (0, 0);
    for x in 1..grid.len() - 1 {
        for y in 1..grid[0].len() - 1 {
            if grid[x][y] == '#' {
                let is_intersection = vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
                    .iter()
                    .all(|(a, b)| grid[*a][*b] == '#');
                if is_intersection {
                    part1 += x * y;
                }
            }
            if grid[x][y] == '^' {
                sx = x as i32;
                sy = y as i32;
                grid[x][y] = 'O';
            }
        }
    }
    println!("Part 1: {}", part1);

    let (mut x, mut y) = (sx, sy);
    let mut d = Direction::Right;
    let mut turns: Vec<Direction> = vec![Direction::Right];
    let mut dists: Vec<i32> = vec![0];
    loop {
        let (dx, dy) = d.offset();
        let (nx, ny) = (x + dx, y + dy);

        // Path continue
        if nx >= 0
            && (nx as usize) < grid.len()
            && ny >= 0
            && (ny as usize) < grid[0].len()
            && grid[nx as usize][ny as usize] == '#'
        {
            x = nx;
            y = ny;
            *dists.last_mut().unwrap() += 1;
            continue;
        }

        // Turn necessary
        let (rdx, rdy) = d.right().offset();
        let (nx, ny) = (x + rdx, y + rdy);
        if nx >= 0
            && (nx as usize) < grid.len()
            && ny >= 0
            && (ny as usize) < grid[0].len()
            && grid[nx as usize][ny as usize] == '#'
        {
            x = nx;
            y = ny;
            d = d.right();
            turns.push(Direction::Right);
            dists.push(1);
            continue;
        }

        let (ldx, ldy) = d.left().offset();
        let (nx, ny) = (x + ldx, y + ldy);
        if nx >= 0
            && (nx as usize) < grid.len()
            && ny >= 0
            && (ny as usize) < grid.len()
            && grid[nx as usize][ny as usize] == '#'
        {
            x = nx;
            y = ny;
            d = d.left();
            turns.push(Direction::Left);
            dists.push(1);
            continue;
        }
        break;
    }

    // println!("Uncompressed: ");
    // for (dir, dist) in turns.iter().zip(dists) {
    //     print!("{},{},", dir.to_string(), dist);
    // }
    // println!("");

    let compressed = "A,B,A,C,A,B,C,A,B,C\nR,12,R,4,R,10,R,12\nR,6,L,8,R,10\nL,8,R,4,R,4,R,6\nn\n".to_string();
    input[0] = 2;
    let mut vm = VM::new(input);

    for c in compressed.chars() {
        vm.push_input(c as u8 as i64);
    }

    let mut out = 0;
    while !vm.halted {
        if let Some(o) = vm.run_until_output() {
            out = o;
        }
    }
    println!("Part 2: {}", out);
}
