use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader, Result, Lines};
use std::fmt::Formatter;

struct Instruction {
    opcode: i32,
    steps_next: i32,
}

const I_ADD: Instruction = Instruction { opcode: 1, steps_next: 4 };
const I_MUL: Instruction = Instruction { opcode: 2, steps_next: 4 };
const I_IN: Instruction = Instruction { opcode: 3, steps_next: 2 };
const I_OUT: Instruction = Instruction { opcode: 4, steps_next: 2 };
const I_HALT: Instruction = Instruction { opcode: 99, steps_next: 0 };

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.opcode {
            1 => write!(f, "I_ADD({})", self.opcode),
            2 => write!(f, "I_MUL({})", self.opcode),
            3 => write!(f, "I_IN({})", self.opcode),
            4 => write!(f, "I_OUT({})", self.opcode),
            _ => write!(f, "UNKNOWN({}", self.opcode)
        }
    }
}

struct VM {
    program: Vec<i32>,
    ip: i32,
    in_p: i32,
    out_p: i32,
    halted: bool,
    inputs: Vec<i32>,
    outputs: Vec<i32>,
}

impl VM {
    fn new(program: Vec<i32>, inputs: Vec<i32>) -> VM {
        VM {
            program,
            ip: 0,
            in_p: 0,
            out_p: 0,
            halted: false,
            inputs,
            outputs: vec!(),
        }
    }

    fn curr_instr(&self) -> Instruction {
        let opcode = self.program[self.ip as usize] % 100;
        match opcode {
            1 => I_ADD,
            2 => I_MUL,
            3 => I_IN,
            4 => I_OUT,
            99 => I_HALT,
            _ => {
                println!("Unknown opcode at ip={}: {}", self.ip, opcode);
                panic!("Uknown opcode")
            }
        }
    }

    fn read_input(&mut self) -> i32 {
        let input = self.inputs[self.in_p as usize];
        self.in_p += 1;
        input
    }

    fn i_add(&mut self) {
        let adr1 = self.program[(self.ip + 1) as usize] as usize;
        let adr2 = self.program[(self.ip + 2) as usize] as usize;
        let adr3 = self.program[(self.ip + 3) as usize] as usize;
        self.program[adr3] = self.program[adr1] + self.program[adr2];
        println!("I_ADD [{}] = [{}]={}+[{}]={}", adr3, adr1, self.program[adr1], adr2, self.program[adr2]);
        self.ip = self.ip + I_ADD.steps_next;
    }

    fn i_mul(&mut self) {
        self.ip = self.ip + I_MUL.steps_next;
    }

    fn i_input(&mut self) {
        let adr = self.program[(self.ip + 1) as usize] as usize;
        let input = self.read_input();
        self.program[adr] = input;
        println!("I_INPUT: setting [{}] to {}", adr, input);
        self.ip = self.ip + I_IN.steps_next;
    }

    fn i_output(&mut self) {
        let adr = self.program[(self.ip + 1) as usize] as usize;
        let output = self.program[adr];
        self.outputs.push(output);
        self.out_p += 1;
        println!("I_OUTPUT: outputting [{}] = {}", adr, output);
        self.ip = self.ip + I_OUT.steps_next;
    }

    fn i_halt(&mut self) {
        println!("I_HALT");
        self.halted = true;
    }

    fn exec_inst(&mut self) {
        let opcode = self.curr_instr().opcode;
        println!("Executing: {} ip={}", opcode, self.ip);
        if opcode == 99 { return self.i_halt(); };
        if opcode == 1 { return self.i_add(); };
        if opcode == 2 { return self.i_mul(); };
        if opcode == 3 { return self.i_input(); };
        if opcode == 4 { return self.i_output(); };
        println!("Unknown instruction: {}, halting", opcode);
        self.i_halt();
    }

    fn is_halted(&self) -> bool {
        self.halted
    }

    fn run(&mut self) {
        self.ip = 0;
        while !self.is_halted() {
            self.exec_inst();
        }
    }
}

fn main() {
//    let program = read_program();
    let program = vec!(3, 0, 4, 0, 99);
    let mut vm: VM = VM::new(program, vec!(1));
    vm.run();
}

fn read_program() -> Vec<i32> {
    if let Ok(lines) = getLines("input.txt") {
        for maybe_line in lines {
            if let Ok(line) = maybe_line {
                println!("{}", line);
                let mut result: Vec<i32> = vec!();
                for item in line.split(",") {
                    let byte: i32 = item.parse().unwrap();
                    result.push(byte);
                }
                return result;
            }
        }
    }
    panic!("no input");
}

fn getLines<P>(file_name: P) -> Result<Lines<BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(file_name)?;
    Ok(BufReader::new(file).lines())
}