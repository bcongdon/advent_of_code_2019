use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::iter;
use std::path::Path;

#[derive(Debug)]
struct VM {
    state: Vec<i64>,
    pc: usize,
    input: Vec<i64>,
    output: Option<i64>,
    halted: bool,
    relative_base: i64,
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
        if self.halted {
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
        while !self.halted {
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
        }
    }

    pub fn set_state(&mut self, pc: usize, val: i64) {
        self.state[pc as usize] = val;
        // println!("set location {} to value {}", pc, val);
    }

    pub fn push_input(&mut self, input: i64) {
        self.input.push(input);
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
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
            Direction::Up => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Down => (0, -1),
            Direction::Right => (1, 0),
        }
    }
}

enum Color {
    Black = 0,
    White = 1,
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

fn run_robot(initial_state: Vec<i64>, initial_color: i32) -> HashMap<(i32, i32), i64> {
    let (mut x, mut y) = (0, 0);
    let mut vm = VM::new(initial_state);
    let mut d = Direction::Up;
    let mut grid: HashMap<(i32, i32), i64> = HashMap::new();

    vm.push_input(1);
    while !vm.halted {
        if let Some(color) = vm.run_until_output() {
            grid.insert((x, y), color);
        } else {
            break;
        }
        if let Some(direction) = vm.run_until_output() {
            match direction {
                0 => d = d.left(),
                1 => d = d.right(),
                _ => panic!("Unknown direction: {}", direction),
            }
        } else {
            break;
        }
        let (dx, dy) = d.offset();
        x += dx;
        y += dy;
        let color = grid.get(&(x, y)).unwrap_or(&0);
        vm.push_input(*color);
    }
    grid
}

pub fn main() {
    let mut input: Vec<i64> = read_file("11.txt")
        .expect("file doesn't exist")
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    input.extend(iter::repeat(0).take(10000));

    let grid1 = run_robot(input.to_vec(), 0);
    println!("Part 1: {}", grid1.len());

    let grid2 = run_robot(input, 1);
    let max_x = *grid2.keys().map(|(x, _)| x).max().unwrap();
    let min_x = *grid2.keys().map(|(x, _)| x).min().unwrap();
    let max_y = *grid2.keys().map(|(_, y)| y).max().unwrap();
    let min_y = *grid2.keys().map(|(_, y)| y).min().unwrap();

    println!("Part 2:");
    for y in (min_y..=max_y).rev() {
        for x in (min_x..=max_x) {
            print!(
                "{}",
                grid2
                    .get(&(x, y))
                    .map(|c| if *c == 0 { " " } else { "*" })
                    .unwrap_or(" ")
            )
        }
        println!("");
    }
}
