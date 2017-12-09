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

    fn subtower_weight(&self, tower: &Tower) -> u32 {
        self.weight +
            self.children
                .iter()
                .map(|c| tower.get(c).map(|p| p.subtower_weight(tower)).unwrap())
                .sum::<u32>()
    }

    fn balanced_weight(&self, tower: &Tower) -> u32 {
        if self.children.is_empty() {
            return 0;
        }

        let weights = self.children
            .iter()
            .map(|p| {
                (
                    tower.get(p).unwrap(),
                    tower.get(p).unwrap().subtower_weight(tower),
                )
            })
            .collect::<Vec<_>>();

        let min_weight = weights.iter().min_by_key(|w| w.1).unwrap();
        let max_weight = weights.iter().max_by_key(|w| w.1).unwrap();

        if min_weight.1 == max_weight.1 {
            return 0;
        }

        let balanced_subweight = max_weight.0.balanced_weight(tower);

        if balanced_subweight > 0 {
            return balanced_subweight;
        }

        max_weight.0.weight - (max_weight.1 - min_weight.1)
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

    pub fn get(&self, p: &str) -> Option<&Program> {
        self.programs.get(p)
    }

    pub fn head(&self) -> Option<&Program> {
        self.programs.values().find(|p| {
            self.programs.values().find(
                |i| i.children.contains(&p.name),
            ) == None
        })
    }

    pub fn balanced_weight(&self) -> u32 {
        if let Some(h) = self.head() {
            h.balanced_weight(self)
        } else {
            0
        }
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

        assert_eq!(Ok(program), "abcd (10)".parse());
    }

    #[test]
    fn parse_program_with_one_child() {
        let program = Program::new("abcd", 10, &["eeee"]);

        assert_eq!(Ok(program), "abcd (10) -> eeee".parse());
    }

    #[test]
    fn parse_program_with_many_children() {
        let program = Program::new("abcd", 10, &["eeee", "xyzw", "ijkl"]);

        assert_eq!(
            Ok(program),
            "abcd (10) -> eeee, xyzw, ijkl".parse()
        );
    }

    fn build_tower() -> Tower {
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

        tower
    }

    #[test]
    fn find_head_of_tower() {
        let tower = build_tower();

        assert_eq!("tknk", tower.head().unwrap().name);
    }

    #[test]
    fn find_balanced_weight() {
        let tower = build_tower();

        assert_eq!(60, tower.balanced_weight());
    }
}
