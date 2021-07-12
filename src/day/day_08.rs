use aoc_core::{bail, AoCDay, ErrorWrapper};
use std::collections::HashMap;

pub struct Day08;

type Num = i64;

#[derive(Debug, Clone, PartialEq)]
enum Op {
    NOP,
    ACC(Num),
    JMP(Num),
}

fn parse(input: &str) -> Vec<Op> {
    input.lines().filter(|l| !l.is_empty()).map(|line| {
        let mut parts = line.trim().split(' ');
        let op_str = parts.next().expect("op_str");
        let operand = parts.next().expect("operand_str").parse::<Num>().expect("parse Num");
        match op_str {
            "nop" => Op::NOP,
            "acc" => Op::ACC(operand),
            "jmp" => Op::JMP(operand),
            _ => panic!("Invalid op: {}", op_str),
        }
    }).collect()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Status {
    Stopped,
    Running,
    LoopDetected,
    Errored,
}

#[derive(Debug, Clone)]
struct VM {
    acc: Num,
    ip: usize,
    code: Vec<Op>,
    status: Status,
}
impl VM {
    fn new(code: Vec<Op>) -> Self {
        VM { acc: 0, ip: 0, code, status: Status::Running }
    }
    fn tick(&mut self) -> Status {
        use std::cmp::Ordering;
        if self.ip == self.code.len() {
            self.status = Status::Stopped;
        } else if self.ip > self.code.len() {
            self.status = Status::Errored;
        } else {
            let op = &self.code[self.ip];
            match op {
                Op::NOP => self.ip += 1,
                Op::ACC(v) => { self.acc += v; self.ip += 1; },
                Op::JMP(v) => {
                    match v.cmp(&0) {
                        Ordering::Greater => self.ip += v.abs() as usize,
                        Ordering::Less => self.ip -= v.abs() as usize,
                        Ordering::Equal => (),
                    }
                },
            }
            self.status = Status::Running;
        }
        self.status
    }

    fn exec(&mut self) -> Status {
        let mut instr_exec_count: HashMap<usize, usize> = HashMap::new();
        while self.status == Status::Running {
            let count = instr_exec_count.entry(self.ip).or_insert(0);
            if *count > 0 {
                self.status = Status::LoopDetected;
            } else {
                *count += 1;
                self.tick();
            }
        }
        self.status
    }
}

impl AoCDay for Day08 {
    fn day(&self) -> usize {
        08
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("1394"), Some("1626"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let code = parse(input);
        let mut vm = VM::new(code);
        if vm.exec() == Status::LoopDetected {
            Ok(vm.acc.to_string())
        } else {
            bail!("Loop not found")
        }
    }
    // TODO: NOP => JMP. My input didn't require that, so no way to test atm.
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let code = parse(input);
        for (i, op) in code.iter().enumerate() {
            match op {
                Op::JMP(_) => {
                    let mut new_code = code.clone();
                    new_code[i] = Op::NOP;
                    let mut vm = VM::new(new_code);
                    if vm.exec() == Status::Stopped {
                        return Ok(vm.acc.to_string())
                    }
                },
                _ => ()
            }
        }
        bail!("Unable to create working code from given input")
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day08)
}
