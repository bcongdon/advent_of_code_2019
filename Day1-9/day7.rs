use std::fs::File;
use std::io::{self, Read};
use std::iter;
use std::path::Path;

#[derive(Debug)]
struct VM {
    state: Vec<i32>,
    pc: usize,
    input: Vec<i32>,
    output: Option<i32>,
    halted: bool,
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

    fn run(&mut self) {
        while !self.halted {
            self.run_one();
        }
    }

    fn run_one(&mut self) {
        if self.halted {
            return;
        }

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
                if !self.input.is_empty() {
                    self.state[idx as usize] = self.input.remove(0);
                } else {
                    self.pc -= 2;
                }
            }
            // Output
            4 => {
                let params = self.get_params(op_code, 1);
                self.output = Some(params[0]);
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
            99 => self.halted = true,
            _ => panic!("Unknown opcode: {}", op_code % 100),
        }
    }

    fn get_output(&mut self) -> Option<i32> {
        let o = self.output;
        self.output = None;
        o
    }

    fn new(initial_state: Vec<i32>) -> VM {
        VM {
            state: initial_state,
            pc: 0,
            output: None,
            input: Vec::new(),
            halted: false,
        }
    }

    pub fn set_state(&mut self, pc: i32, val: i32) {
        self.state[pc as usize] = val;
    }

    pub fn push_input(&mut self, input: i32) {
        self.input.push(input);
    }
}

fn test_amplifier(input: Vec<i32>, phases: Vec<i32>) -> i32 {
    let mut output = 0;
    for p in phases {
        let mut vm = VM::new(input.to_vec());
        vm.push_input(p);
        vm.push_input(output);
        vm.run();
        output = vm.output.unwrap();
    }
    output
}

fn amplifier_part2(input: Vec<i32>, phases: Vec<i32>) -> i32 {
    let mut vms: Vec<VM> = phases
        .iter()
        .map(|i| {
            let mut vm = VM::new(input.to_vec());
            vm.push_input(*i);
            vm
        })
        .collect();
    vms.first_mut().unwrap().push_input(0);
    let mut last_e = 0;
    let mut outputs: Vec<Option<i32>> = iter::repeat(None).take(vms.len()).collect();

    while vms.iter().any(|v| !v.halted) {
        for (idx, vm) in (&mut vms).iter_mut().enumerate() {
            let prev_idx = (idx + phases.len() - 1) % phases.len();
            if let Some(prev_output) = outputs[prev_idx] {
                vm.push_input(prev_output);
                outputs[prev_idx] = None;
            }
            vm.run_one();
            if let Some(o) = vm.get_output() {
                if idx == phases.len() - 1 {
                    last_e = o;
                }
                outputs[idx] = Some(o);
            }
        }
    }
    last_e
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

// Yes, this is hideous.
fn five_combos(low: i32, high: i32) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    for a in low..=high {
        for b in low..=high {
            if b == a {
                continue;
            }
            for c in low..=high {
                if c == a || c == b {
                    continue;
                }
                for d in low..=high {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in low..=high {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        out.push(vec![a, b, c, d, e]);
                    }
                }
            }
        }
    }
    out
}

pub fn main() {
    let mut input: Vec<i32> = read_file("7.txt")
        .expect("file doesn't exist")
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    input.extend(iter::repeat(0).take(1000));
    let mut max_signal = 0;

    let part1 = five_combos(0, 4).iter().map(|phases| test_amplifier(input.to_vec(), phases.to_vec())).max();
    println!("Part 1: {}", part1.unwrap());

    let part2 = five_combos(5, 9).iter().map(|phases| amplifier_part2(input.to_vec(), phases.to_vec())).max();
    println!("Part 2: {}", part2.unwrap());
}
