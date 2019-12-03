use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug)]
struct VM {
    state: Vec<i32>,
    pc: usize,
}

impl VM {
    fn run(&mut self) -> i32{
        loop {
            let op_code = &self.state[self.pc];
            match op_code {
                1 => {
                    let a = self.state[self.pc + 1] as usize;
                    let b = self.state[self.pc + 2] as usize;
                    let c = self.state[self.pc + 3] as usize;
                    self.state[c] = self.state[a] + self.state[b];
                }
                2 => {
                    let a = self.state[self.pc + 1] as usize;
                    let b = self.state[self.pc + 2] as usize;
                    let c = self.state[self.pc + 3] as usize;
                    self.state[c] = self.state[a] * self.state[b];
                }
                99 => return self.state[0],
                _ => panic!("Unknown opcode: {}", op_code),
            }
            self.pc += 4;
        }
    }

    fn new(initial_state: Vec<i32>) -> VM {
        VM {
            state: initial_state,
            pc: 0,
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
    let input: Vec<i32> = read_file("2.txt")
        .expect("file doesn't exist")
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let mut vm = VM::new(input.to_vec());
    vm.set_state(1, 12);
    vm.set_state(2, 2);
    println!("Part 1: {}", vm.run());

    for noun in 0..99 {
        for verb in 0..99 {
            let mut vm = VM::new(input.to_vec());
            vm.set_state(1, noun);
            vm.set_state(2, verb);
            if vm.run() == 19690720{
                println!("Part 2: {}", 100*noun + verb);
            }
        }
    }
}
