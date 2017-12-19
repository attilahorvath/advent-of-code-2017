use std::collections::HashMap;
use std::str::FromStr;

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
    Rcv(Value),
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
                let v = parts.next().ok_or(InstructionParseError)?;

                Ok(Instruction::Rcv(v.parse()?))
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

pub struct Vm {
    registers: HashMap<char, i64>,
    instructions: Vec<Instruction>,
    pc: i64,
}

impl Vm {
    pub fn new(instructions: &[Instruction]) -> Self {
        Vm {
            registers: HashMap::new(),
            instructions: instructions.to_vec(),
            pc: 0,
        }
    }

    pub fn execute(&mut self) -> i64 {
        let mut last_frequency = 0;

        loop {
            let mut pc_updated = false;

            match self.instructions[self.pc as usize] {
                Instruction::Snd(ref value) => {
                    last_frequency = self.get_value(value);
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
                Instruction::Rcv(ref value) => {
                    if self.get_value(value) != 0 {
                        return last_frequency;
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
                return last_frequency;
            }
        }
    }

    pub fn get_value(&self, value: &Value) -> i64 {
        match *value {
            Value::Register(name) => self.registers.get(&name).cloned().unwrap_or(0),
            Value::Number(number) => number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_last_frequency() {
        let mut vm = Vm::new(
            &[
                Instruction::Set('a', Value::Number(1)),
                Instruction::Add('a', Value::Number(2)),
                Instruction::Mul('a', Value::Register('a')),
                Instruction::Mod('a', Value::Number(5)),
                Instruction::Snd(Value::Register('a')),
                Instruction::Set('a', Value::Number(0)),
                Instruction::Rcv(Value::Register('a')),
                Instruction::Jgz(Value::Register('a'), Value::Number(-1)),
                Instruction::Set('a', Value::Number(1)),
                Instruction::Jgz(Value::Register('a'), Value::Number(-2)),
            ],
        );

        assert_eq!(4, vm.execute());
    }
}
