use std::collections::VecDeque;
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
            if self.output.is_some() {
                return self.get_output();
            }
        }
        None
    }

    fn get_output(&mut self) -> Option<i64> {
        let o = self.output;
        self.output = None;
        return o;
    }

    fn run_until_interrupt(&mut self) {
        while !self.halted && !self.waiting_for_input && self.output.is_none() {
            self.run_one();
        }
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

    pub fn write_string(&mut self, s: String) {
        for c in s.chars() {
            self.push_input(c as u8 as i64);
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

#[derive(Debug, Copy, Clone)]
struct Packet {
    x: i64,
    y: i64,
}

struct Network {
    vms: Vec<VM>,
    partial_packets: Vec<Vec<i64>>,
    mailboxes: Vec<VecDeque<Packet>>,
    nat_packet: Option<Packet>,
    prev_packet: Option<Packet>,
}

impl Network {
    pub fn setup(input: Vec<i64>, num_devices: usize) -> Network {
        let vms = (0..num_devices)
            .map(|i| {
                let mut vm = VM::new(input.to_vec());
                vm.push_input(i as i64);
                vm
            })
            .collect();

        Network {
            vms,
            partial_packets: iter::repeat_with(|| Vec::new()).take(num_devices).collect(),
            mailboxes: iter::repeat_with(|| VecDeque::new())
                .take(num_devices)
                .collect(),
            nat_packet: None,
            prev_packet: None,
        }
    }

    fn run_one(&mut self) {
        for (idx, vm) in self.vms.iter_mut().enumerate() {
            // if !vm.input.is_empty() {
            //     println!("{} {} {:?}", idx, vm.halted, vm.input);
            // }
            vm.run_until_interrupt();
            if let Some(packet) = self.mailboxes[idx].pop_front() {
                vm.push_input(packet.x);
                vm.push_input(packet.y);
            } else if vm.waiting_for_input {
                vm.push_input(-1);
            }
            if let Some(v) = vm.get_output() {
                // println!("{:?}", self.partial_packets[idx]);
                self.partial_packets[idx].push(v);
                assert!(self.partial_packets[idx].len() <= 3);
                if self.partial_packets[idx].len() == 3 {
                    let data = &self.partial_packets[idx];
                    let addr = data[0] as usize;
                    let packet = Packet {
                        x: data[1],
                        y: data[2],
                    };
                    self.partial_packets[idx].clear();
                    if addr == 255 {
                        if self.nat_packet.is_none() {
                            println!("Part 1: {}", packet.y);
                        }
                        self.nat_packet = Some(packet);
                    } else if addr < self.mailboxes.len() {
                        self.mailboxes[addr as usize].push_back(packet);
                    }
                }
            }
        }
    }

    fn is_idle(&self) -> bool {
        self.vms
            .iter()
            .all(|vm| vm.input.len() == 1 && vm.input.first() == Some(&-1))
    }

    pub fn run(&mut self) {
        while self.vms.iter().any(|vm| !vm.halted) {
            self.run_one();
            if self.is_idle() && self.nat_packet.is_some() {
                if let Some(old) = self.prev_packet {
                    if self.nat_packet.unwrap().y == old.y {
                        println!("Part 2: {}", old.y);
                        return;
                    }
                }
                self.prev_packet = self.nat_packet;
                self.mailboxes[0].push_back(self.nat_packet.unwrap());
            }
        }
    }
}

pub fn main() {
    let mut input: Vec<i64> = read_file("23.txt")
        .expect("file doesn't exist")
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    input.extend(iter::repeat(0).take(10000));

    let mut network = Network::setup(input, 50);
    network.run();
}
