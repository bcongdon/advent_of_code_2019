use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Read};
use std::iter;
use std::path::Path;

#[derive(Debug)]
struct VM {
    state: Vec<i64>,
    pc: usize,
    input: Vec<i64>,
    output: Vec<i64>,
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
                self.output.push(params[0]);
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

    fn run_until_input(&mut self) {
        while !self.halted && !self.waiting_for_input {
            self.run_one();
        }
    }

    fn get_output(&mut self) -> Vec<i64> {
        let o = self.output.to_vec();
        self.output.clear();
        return o;
    }

    fn run_until_interrupt(&mut self) {
        while !self.halted && !self.waiting_for_input && self.output.is_empty() {
            self.run_one();
        }
    }

    fn new(initial_state: Vec<i64>) -> VM {
        VM {
            state: initial_state,
            pc: 0,
            output: Vec::new(),
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

    pub fn write_string(&mut self, s: String) {
        for c in s.chars() {
            self.push_input(c as u8 as i64);
        }
    }

    pub fn output_str(&mut self) -> String {
        self.get_output().iter().map(|x| *x as u8 as char).collect()
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

#[derive(Debug, Copy, Clone)]
struct Packet {
    x: i64,
    y: i64,
}

pub fn main() {
    let mut input: Vec<i64> = read_file("25.txt")
        .expect("file doesn't exist")
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    input.extend(iter::repeat(0).take(10000));

    let mut vm = VM::new(input);
    let stdin = io::stdin();
    vm.run_until_input();
    println!("{}", vm.output_str());
    let cmds = vec![
        "north",
        "north",
        "take sand",
        "south",
        "south",
        "south",
        "take space heater",
        "south",
        "east",
        "take loom",
        "west",
        "north",
        "west",
        "take wreath",
        "south",
        "take space law space brochure",
        "south",
        "take pointer",
        "north",
        "north",
        "east",
        "north",
        "west",
        "south",
        "take planetoid",
        "north",
        "west",
        "take festive hat",
        "south",
        "west",
    ];

    for c in cmds {
        vm.write_string(c.to_string() + "\n");
        vm.run_until_input();
    }

    let inv = vec![
        "sand",
        "space heater",
        "loom",
        "wreath",
        "space law space brochure",
        "pointer",
        "planetoid",
        "festive hat",
    ];
    for i in 0..2_i64.pow(inv.len() as u32) {
        for j in 0..inv.len() {
            if i & 2_i64.pow(j as u32) == 0 {
                vm.write_string("take ".to_string() + inv[j] + "\n");
            } else {
                vm.write_string("drop ".to_string() + inv[j] + "\n");
            }
            vm.write_string("north\n".to_string());
        }
        let out = vm.output_str();
        if out.contains("Analysis complete") {
            println!("{}", out);
            break;
        }
    }
    vm.run_until_input();
    println!("{}", vm.output_str());

    // println!("{}", vm.output_str());
    // for line in stdin.lock().lines() {
    //     vm.write_string(line.unwrap() + "\n");
    //     vm.run_until_input();
    //     println!("{}", vm.output_str());
    //     if vm.halted {
    //         break;
    //     }
    // }
}
