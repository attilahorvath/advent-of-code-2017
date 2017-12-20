use std::collections::HashMap;
use std::str::FromStr;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Value {
    Register(char),
    Number(i64),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

#[derive(Debug, Clone)]
pub struct InstructionParseError;

impl FromStr for Value {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i64>() {
            Ok(Value::Number(n))
        } else {
            s.chars().next().map(|n| Value::Register(n)).ok_or(
                InstructionParseError,
            )
        }
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        match parts.next() {
            Some("snd") => {
                let v = parts.next().ok_or(InstructionParseError)?;

                Ok(Instruction::Snd(v.parse()?))
            }
            Some("set") => {
                let n = parts.next().ok_or(InstructionParseError)?;
                let v = parts.next().ok_or(InstructionParseError)?;

                Ok(Instruction::Set(
                    n.chars().next().ok_or(InstructionParseError)?,
                    v.parse()?,
                ))
            }
            Some("add") => {
                let n = parts.next().ok_or(InstructionParseError)?;
                let v = parts.next().ok_or(InstructionParseError)?;

                Ok(Instruction::Add(
                    n.chars().next().ok_or(InstructionParseError)?,
                    v.parse()?,
                ))
            }
            Some("mul") => {
                let n = parts.next().ok_or(InstructionParseError)?;
                let v = parts.next().ok_or(InstructionParseError)?;

                Ok(Instruction::Mul(
                    n.chars().next().ok_or(InstructionParseError)?,
                    v.parse()?,
                ))
            }
            Some("mod") => {
                let n = parts.next().ok_or(InstructionParseError)?;
                let v = parts.next().ok_or(InstructionParseError)?;

                Ok(Instruction::Mod(
                    n.chars().next().ok_or(InstructionParseError)?,
                    v.parse()?,
                ))
            }
            Some("rcv") => {
                let n = parts.next().ok_or(InstructionParseError)?;

                Ok(Instruction::Rcv(
                    n.chars().next().ok_or(InstructionParseError)?,
                ))
            }
            Some("jgz") => {
                let v = parts.next().ok_or(InstructionParseError)?;
                let o = parts.next().ok_or(InstructionParseError)?;

                Ok(Instruction::Jgz(v.parse()?, o.parse()?))
            }
            _ => Err(InstructionParseError),
        }
    }
}

pub struct Program {
    pid: usize,
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    pc: i64,
    receiver: mpsc::Receiver<i64>,
    senders: HashMap<usize, mpsc::Sender<i64>>,
}

impl Program {
    pub fn new(pid: usize, instructions: &[Instruction]) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', pid as i64);

        let (own_sender, receiver) = mpsc::channel();

        let mut senders = HashMap::new();
        senders.insert(pid, own_sender);

        Program {
            pid,
            registers,
            instructions: instructions.to_vec(),
            pc: 0,
            receiver,
            senders,
        }
    }

    pub fn execute(&mut self) -> u32 {
        let mut values_sent = 0;

        loop {
            let mut pc_updated = false;

            match self.instructions[self.pc as usize] {
                Instruction::Snd(ref value) => {
                    let v = self.get_value(value);

                    for (_, sender) in self.senders.iter().filter(|&(pid, _)| pid != &self.pid) {
                        sender.send(v).unwrap();
                    }

                    values_sent += 1;
                }
                Instruction::Set(name, ref value) => {
                    let v = self.get_value(value);
                    *self.registers.entry(name).or_insert(0) = v;
                }
                Instruction::Add(name, ref value) => {
                    let v = self.get_value(value);
                    *self.registers.entry(name).or_insert(0) += v;
                }
                Instruction::Mul(name, ref value) => {
                    let v = self.get_value(value);
                    *self.registers.entry(name).or_insert(0) *= v;
                }
                Instruction::Mod(name, ref value) => {
                    let v = self.get_value(value);
                    *self.registers.entry(name).or_insert(0) %= v;
                }
                Instruction::Rcv(name) => {
                    if let Ok(v) = self.receiver.recv_timeout(Duration::from_secs(1)) {
                        *self.registers.entry(name).or_insert(0) = v;
                    } else {
                        return values_sent;
                    }
                }
                Instruction::Jgz(ref value, ref offset) => {
                    let v = self.get_value(value);
                    let o = self.get_value(offset);

                    if v > 0 {
                        self.pc += o;
                        pc_updated = true;
                    }
                }
            }

            if !pc_updated {
                self.pc += 1;
            }

            if self.pc < 0 || self.pc >= self.instructions.len() as i64 {
                return values_sent;
            }
        }
    }

    pub fn get_value(&self, value: &Value) -> i64 {
        match *value {
            Value::Register(name) => self.registers.get(&name).cloned().unwrap_or(0),
            Value::Number(number) => number,
        }
    }

    pub fn own_sender(&self) -> &mpsc::Sender<i64> {
        self.senders.get(&self.pid).unwrap()
    }
}

pub struct Vm {
    programs: Vec<Program>,
}

impl Vm {
    pub fn new() -> Self {
        Vm { programs: Vec::new() }
    }

    pub fn init_program(&mut self, instructions: &[Instruction]) {
        let mut program = Program::new(self.programs.len(), instructions);

        for p in self.programs.iter_mut() {
            p.senders.insert(program.pid, program.own_sender().clone());
            program.senders.insert(p.pid, p.own_sender().clone());
        }

        self.programs.push(program);
    }

    pub fn execute(self) -> u32 {
        let mut handles = Vec::new();

        for mut p in self.programs {
            handles.push(thread::spawn(move || p.execute()));
        }

        let mut values_sent = 0;

        for handle in handles {
            values_sent = handle.join().unwrap();
        }

        values_sent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_values_sent_by_last_program() {
        let instructions = [
            Instruction::Snd(Value::Number(1)),
            Instruction::Snd(Value::Number(2)),
            Instruction::Snd(Value::Register('p')),
            Instruction::Rcv('a'),
            Instruction::Rcv('b'),
            Instruction::Rcv('c'),
            Instruction::Rcv('d'),
        ];

        let mut vm = Vm::new();

        vm.init_program(&instructions);
        vm.init_program(&instructions);

        assert_eq!(3, vm.execute());
    }
}
