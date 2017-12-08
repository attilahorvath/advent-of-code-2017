use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct ProgramParseError;

#[derive(Debug, PartialEq)]
pub struct Program {
    name: String,
    weight: u32,
    children: Vec<String>,
}

impl Program {
    pub fn new(name: &str, weight: u32, children: &[&str]) -> Self {
        Program {
            name: name.to_owned(),
            weight,
            children: children.iter().map(|c| c.to_string()).collect(),
        }
    }
}

impl FromStr for Program {
    type Err = ProgramParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();

        let name = match parts.next() {
            Some(n) => n.to_owned(),
            None => return Err(ProgramParseError),
        };

        let weight = match parts.next() {
            Some(w) => {
                w.trim_matches(|p| p == '(' || p == ')').parse().unwrap_or(
                    0,
                )
            }
            None => return Err(ProgramParseError),
        };

        let mut children = Vec::new();

        if let Some(_) = parts.next() {
            children = parts.map(|p| p.trim_matches(',').to_owned()).collect();
        }

        Ok(Program {
            name,
            weight,
            children,
        })
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Tower {
    programs: HashMap<String, Program>,
}

impl Tower {
    pub fn new() -> Self {
        Tower { programs: HashMap::new() }
    }

    pub fn add(&mut self, program: Program) {
        self.programs.insert(program.name.clone(), program);
    }

    pub fn head(&self) -> Option<&Program> {
        self.programs.values().find(|p| {
            self.programs.values().find(
                |i| i.children.contains(&p.name),
            ) == None
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_string() {
        assert_eq!(Err(ProgramParseError), "".parse::<Program>());
    }

    #[test]
    fn parse_name_only() {
        assert_eq!(Err(ProgramParseError), "abcd".parse::<Program>());
    }

    #[test]
    fn parse_simple_program() {
        let program = Program::new("abcd", 10, &[]);

        assert_eq!(Ok(program), "abcd (10)".parse::<Program>());
    }

    #[test]
    fn parse_program_with_one_child() {
        let program = Program::new("abcd", 10, &["eeee"]);

        assert_eq!(Ok(program), "abcd (10) -> eeee".parse::<Program>());
    }

    #[test]
    fn parse_program_with_many_children() {
        let program = Program::new("abcd", 10, &["eeee", "xyzw", "ijkl"]);

        assert_eq!(
            Ok(program),
            "abcd (10) -> eeee, xyzw, ijkl".parse::<Program>()
        );
    }

    #[test]
    fn find_head_of_tower() {
        let mut tower = Tower::new();

        tower.add("pbga (66)".parse().unwrap());
        tower.add("xhth (57)".parse().unwrap());
        tower.add("ebii (61)".parse().unwrap());
        tower.add("havc (66)".parse().unwrap());
        tower.add("ktlj (57)".parse().unwrap());
        tower.add("fwft (72) -> ktlj, cntj, xhth".parse().unwrap());
        tower.add("qoyq (66)".parse().unwrap());
        tower.add("padx (45) -> pbga, havc, qoyq".parse().unwrap());
        tower.add("tknk (41) -> ugml, padx, fwft".parse().unwrap());
        tower.add("jptl (61)".parse().unwrap());
        tower.add("ugml (68) -> gyxo, ebii, jptl".parse().unwrap());
        tower.add("gyxo (61)".parse().unwrap());
        tower.add("cntj (57)".parse().unwrap());

        assert_eq!("tknk", tower.head().unwrap().name);
    }
}
