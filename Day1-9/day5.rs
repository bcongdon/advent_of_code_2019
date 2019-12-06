use std::fs::File;
use std::io::{self, Read};
use std::iter;
use std::path::Path;

#[derive(Debug)]
struct VM {
    state: Vec<i32>,
    pc: usize,
    input: i32,
    output: i32,
}

impl VM {
    fn next_value(&mut self) -> i32 {
        let v = self.state[self.pc];
        self.pc += 1;
        v
    }

    fn get_params(&mut self, code: i32, num_params: i32) -> Vec<i32> {
        let mut params = Vec::with_capacity(num_params as usize);
        for i in 0..num_params {
            let immediate = (code % 10_i32.pow(3 + i as u32)) / (10_i32.pow(2 + i as u32)) == 1;
            let p = self.next_value();
            if immediate {
                params.push(p);
            } else {
                params.push(self.state[p as usize]);
            }
        }
        params
    }

    fn run(&mut self) -> i32 {
        loop {
            let op_code = self.next_value();
            match op_code % 100 {
                // Add
                1 => {
                    let params = self.get_params(op_code, 2);
                    let pos = self.next_value();
                    self.state[pos as usize] = params[0] + params[1];
                }
                // Mult
                2 => {
                    let params = self.get_params(op_code, 2);
                    let pos = self.next_value();
                    self.state[pos as usize] = params[0] * params[1]
                }
                // Input
                3 => {
                    let idx = self.next_value();
                    self.state[idx as usize] = self.input;
                }
                // Output
                4 => {
                    let params = self.get_params(op_code, 1);
                    self.output = params[0];
                }
                // Jump-Non-Zero
                5 => {
                    let params = self.get_params(op_code, 2);
                    if params[0] != 0 {
                        self.pc = params[1] as usize;
                    }
                }
                // Jump-Eq-Zero
                6 => {
                    let params = self.get_params(op_code, 2);
                    if params[0] == 0 {
                        self.pc = params[1] as usize;
                    }
                }
                // Less Than
                7 => {
                    let params = self.get_params(op_code, 2);
                    let idx = self.next_value() as usize;
                    self.state[idx] = if params[0] < params[1] { 1 } else { 0 };
                }
                // Equal
                8 => {
                    let params = self.get_params(op_code, 2);
                    let idx = self.next_value() as usize;
                    self.state[idx] = if params[0] == params[1] { 1 } else { 0 };
                }
                // Halt
                99 => return self.state[0],
                _ => panic!("Unknown opcode: {}", op_code % 100),
            }
        }
    }

    fn new(initial_state: Vec<i32>, input: i32) -> VM {
        VM {
            state: initial_state,
            pc: 0,
            output: -1,
            input,
        }
    }

    pub fn set_state(&mut self, pc: i32, val: i32) {
        self.state[pc as usize] = val;
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

pub fn main() {
    let mut input: Vec<i32> = read_file("5.txt")
        .expect("file doesn't exist")
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    input.extend(iter::repeat(0).take(1000));

    let mut vm = VM::new(input.to_vec(), 1);
    vm.run();
    println!("Part 1: {}", vm.output);

    let mut vm = VM::new(input, 5);
    vm.run();
    println!("Part 2: {}", vm.output);
}
