use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Inc(i32),
    Dec(i32),
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    Gt(i32),
    Lt(i32),
    Ge(i32),
    Le(i32),
    Eq(i32),
    Ne(i32),
}

#[derive(Debug, PartialEq)]
pub struct InstructionParseError;

#[derive(Debug, PartialEq)]
pub struct Instruction {
    register: String,
    operation: Operation,
    conditional_register: String,
    condition: Condition,
}

impl Instruction {
    pub fn new(
        register: &str,
        operation: Operation,
        conditional_register: &str,
        condition: Condition,
    ) -> Self {
        Instruction {
            register: register.to_string(),
            operation,
            conditional_register: conditional_register.to_string(),
            condition,
        }
    }
}

fn next_part<T: FromStr>(parts: &mut Iterator<Item = &str>) -> Result<T, InstructionParseError> {
    if let Some(s) = parts.next() {
        s.parse().map_err(|_| InstructionParseError)
    } else {
        Err(InstructionParseError)
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let register = next_part(&mut parts)?;
        let oper = next_part::<String>(&mut parts)?;
        let argument = next_part(&mut parts)?;

        let operation = match oper.as_str() {
            "inc" => Operation::Inc(argument),
            "dec" => Operation::Dec(argument),
            _ => return Err(InstructionParseError),
        };

        let _if_keyword = next_part::<String>(&mut parts)?;

        let conditional_register = next_part(&mut parts)?;
        let condition_oper = next_part::<String>(&mut parts)?;
        let condition_argument = next_part(&mut parts)?;

        let condition = match condition_oper.as_str() {
            "<" => Condition::Lt(condition_argument),
            ">" => Condition::Gt(condition_argument),
            ">=" => Condition::Ge(condition_argument),
            "<=" => Condition::Le(condition_argument),
            "==" => Condition::Eq(condition_argument),
            "!=" => Condition::Ne(condition_argument),
            _ => return Err(InstructionParseError),
        };

        Ok(Instruction {
            register,
            operation,
            conditional_register,
            condition,
        })
    }
}

pub struct Processor {
    registers: HashMap<String, i32>,
    largest_value_overall: i32,
}

impl Processor {
    pub fn new() -> Self {
        Processor {
            registers: HashMap::new(),
            largest_value_overall: 0,
        }
    }

    fn check_condition(&mut self, instruction: &Instruction) -> bool {
        let conditional_register = self.registers
            .entry(instruction.conditional_register.clone())
            .or_insert(0);

        match instruction.condition {
            Condition::Lt(argument) => *conditional_register < argument,
            Condition::Gt(argument) => *conditional_register > argument,
            Condition::Le(argument) => *conditional_register <= argument,
            Condition::Ge(argument) => *conditional_register >= argument,
            Condition::Eq(argument) => *conditional_register == argument,
            Condition::Ne(argument) => *conditional_register != argument,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) {
        if !self.check_condition(instruction) {
            return;
        }

        let register = self.registers
            .entry(instruction.register.clone())
            .or_insert(0);

        match instruction.operation {
            Operation::Inc(argument) => *register += argument,
            Operation::Dec(argument) => *register -= argument,
        }

        if *register > self.largest_value_overall {
            self.largest_value_overall = *register;
        }
    }

    pub fn value(&self, register: &str) -> i32 {
        self.registers
            .get(&register.to_string())
            .unwrap_or(&0)
            .clone()
    }

    pub fn largest_value(&self) -> i32 {
        self.registers.values().max().cloned().unwrap_or(0)
    }

    pub fn largest_value_overall(&self) -> i32 {
        self.largest_value_overall
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gt_instruction() {
        let instruction = Instruction::new("b", Operation::Inc(5), "a", Condition::Gt(1));

        assert_eq!(Ok(instruction), "b inc 5 if a > 1".parse());
    }

    #[test]
    fn parse_lt_instruction() {
        let instruction = Instruction::new("a", Operation::Inc(1), "b", Condition::Lt(5));

        assert_eq!(Ok(instruction), "a inc 1 if b < 5".parse());
    }

    #[test]
    fn parse_ge_instruction() {
        let instruction = Instruction::new("c", Operation::Dec(-10), "a", Condition::Ge(1));

        assert_eq!(Ok(instruction), "c dec -10 if a >= 1".parse());
    }

    #[test]
    fn parse_eq_instruction() {
        let instruction = Instruction::new("c", Operation::Inc(-20), "c", Condition::Eq(10));

        assert_eq!(Ok(instruction), "c inc -20 if c == 10".parse());
    }

    #[test]
    fn execute_instructions() {
        let mut processor = Processor::new();

        processor.execute(&Instruction::new(
            "b",
            Operation::Inc(5),
            "a",
            Condition::Gt(1),
        ));
        processor.execute(&Instruction::new(
            "a",
            Operation::Inc(1),
            "b",
            Condition::Lt(5),
        ));
        processor.execute(&Instruction::new(
            "c",
            Operation::Dec(-10),
            "a",
            Condition::Ge(1),
        ));
        processor.execute(&Instruction::new(
            "c",
            Operation::Inc(-20),
            "c",
            Condition::Eq(10),
        ));

        assert_eq!(1, processor.value("a"));
        assert_eq!(0, processor.value("b"));
        assert_eq!(-10, processor.value("c"));
    }

    #[test]
    fn get_largest_value() {
        let mut processor = Processor::new();

        processor.execute(&Instruction::new(
            "b",
            Operation::Inc(5),
            "a",
            Condition::Gt(1),
        ));
        processor.execute(&Instruction::new(
            "a",
            Operation::Inc(1),
            "b",
            Condition::Lt(5),
        ));
        processor.execute(&Instruction::new(
            "c",
            Operation::Dec(-10),
            "a",
            Condition::Ge(1),
        ));
        processor.execute(&Instruction::new(
            "c",
            Operation::Inc(-20),
            "c",
            Condition::Eq(10),
        ));

        assert_eq!(1, processor.largest_value());
    }

    #[test]
    fn get_largest_value_overall() {
        let mut processor = Processor::new();

        processor.execute(&Instruction::new(
            "b",
            Operation::Inc(5),
            "a",
            Condition::Gt(1),
        ));
        processor.execute(&Instruction::new(
            "a",
            Operation::Inc(1),
            "b",
            Condition::Lt(5),
        ));
        processor.execute(&Instruction::new(
            "c",
            Operation::Dec(-10),
            "a",
            Condition::Ge(1),
        ));
        processor.execute(&Instruction::new(
            "c",
            Operation::Inc(-20),
            "c",
            Condition::Eq(10),
        ));

        assert_eq!(10, processor.largest_value_overall());
    }
}
