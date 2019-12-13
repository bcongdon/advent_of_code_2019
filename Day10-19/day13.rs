use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use std::iter;
use std::path::Path;
use std::{thread, time};

const INTERACTIVE: bool= false;

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

#[derive(std::cmp::PartialEq)]
#[derive(Clone, Copy)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl Tile {
    pub fn from_int(i: i64) -> Tile {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Invalid tile type"),
        }
    }

    pub fn string_repr(&self) -> String {
        match self {
            Tile::Empty => String::from(" "),
            Tile::Wall => String::from("|"),
            Tile::Block => String::from("X"),
            Tile::Paddle => String::from("-"),
            Tile::Ball => String::from("o"),
        }
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

struct Arcade {
    grid: HashMap<(i64, i64), Tile>,
    vm: VM,
    score: i64,
    ball_x: i64,
    paddle_x: i64,
    max_x: i64,
    max_y: i64,
}

impl Arcade {
    fn new(mut initial_state: Vec<i64>, insert_quarters: bool) -> Arcade {
        if insert_quarters {
            initial_state[0] = 2;
        }
        let mut a = Arcade {
            grid: HashMap::new(),
            vm: VM::new(initial_state),
            score: -1,
            ball_x: -1,
            paddle_x: -1,
            max_x: -1,
            max_y: -1,
        };
        a.run_arcade_step();
        a.max_x = *a.grid.keys().map(|(x, _)| x).max().unwrap();
        a.max_y = *a.grid.keys().map(|(_, y)| y).max().unwrap();
        a
    }

    fn run_arcade_step(&mut self) {
        loop {
            let x = self.vm.run_until_output();
            let y = self.vm.run_until_output();
            let t = self.vm.run_until_output();
            if !x.is_some() || !y.is_some() || !t.is_some() {
                return
            }
            let (x, y, t) = (x.unwrap(), y.unwrap(), t.unwrap());
            if x == -1 && y == 0 {
                self.score = t;
            } else {
                let tile = Tile::from_int(t);
                self.grid.insert((x, y), tile);

                match tile {
                    Tile::Ball => self.ball_x = x,
                    Tile::Paddle => self.paddle_x = x,
                    _ => {}
                }
            }
        }
    }

    fn render(&self) {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
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



pub fn main() {
    let mut input: Vec<i64> = read_file("13.txt")
        .expect("file doesn't exist")
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    input.extend(iter::repeat(0).take(10000));

    let mut a = Arcade::new(input.to_vec(), false);
    while !a.vm.halted {
        a.run_arcade_step()
    }
    println!("Part 1: {}", a.grid.values().filter(|v| **v == Tile::Block).count());

    let mut a = Arcade::new(input.to_vec(), true);
    while !a.vm.halted {
        a.run_arcade_step();
        if a.ball_x < a.paddle_x {
            a.vm.push_input(-1);
        } else if a.ball_x > a.paddle_x {
            a.vm.push_input(1)
        } else {
            a.vm.push_input(0);
        }
        if INTERACTIVE {
            print!("{}[2J", 27 as char);
            thread::sleep(time::Duration::from_millis(25));
            a.render();
            println!("{}, {}, {}", a.ball_x, a.paddle_x, a.score);
            thread::sleep(time::Duration::from_millis(25));
        }
    }
    println!("Part 2: {}", a.score);
}
