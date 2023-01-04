use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Index;
use std::path::Path;

pub struct Day11Part1<'a> {
    input_path: &'a Path,
}

enum OperationSign {
    PLUS,
    MUL,
}

impl OperationSign {
    pub fn apply_to(&self, value1: u32, value2: u32) -> u32 {
        match self {
            OperationSign::PLUS => value1 + value2,
            OperationSign::MUL => value1 * value2,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            OperationSign::PLUS => "PLUS",
            OperationSign::MUL => "MUL",
        }
    }
}


struct Monkey<'a> {
    id: u32,
    items: Vec<u32>,
    operation: OperationSign,
    test: &'a dyn Fn(u32) -> bool,
    test_pass_target: u32,
    test_fail_target: u32,
}

impl<'a> Debug for Monkey<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"Monkey {{ id: {} }}"#, self.id)
    }
}


impl<'a> Day11Part1<'a> {
    pub fn new(path: &'a str) -> Self {
        Self { input_path: Path::new(path) }
    }

    fn parse(&self) -> Vec<Monkey> {
        let file = match File::open(self.input_path) {
            Ok(f) => f,
            Err(_) => {
                panic!("======!  Cannot open file path: {:?} !======", self.input_path)
            }
        };

        let reader = BufReader::new(file);

        let mut groups = Vec::new();
        let mut current_group = Vec::new();

        for line in reader.lines() {
            match line {
                Ok(val) => {
                    if val.is_empty() {
                        groups.push(current_group);
                        current_group = Vec::new();
                    } else {
                        current_group.push(val);
                    }
                }
                Err(_) => { break; }
            }
        };

        groups.into_iter().map(|group| self.parse_monkey(group)).collect()
    }

    fn separate_string(&self, line: &'a String, sep: &str) -> Vec<&str> {
        line.split(sep).collect::<Vec<_>>()
    }

    fn parse_monkey_operation(&self, line: &String) -> OperationSign {
        let parts = self.separate_string(line, " ");
        let calc_parts = parts[parts.iter().position(|x| *x == "=").unwrap() + 1..].iter().collect::<Vec<_>>();
        match *calc_parts[1] {
            "+" => OperationSign::PLUS,
            "*" => OperationSign::MUL,
            symbol => { panic!(r#"unexpected operation "{}""#, symbol) }
        }
    }

    fn parse_monkey(&self, group: Vec<String>) -> Monkey {
        Monkey {
            id: group[0].chars().into_iter().filter(|x| x.is_digit(10)).collect::<String>().parse::<u32>().unwrap(),
            items: group[1][group[1].find(":").unwrap() + 1..].trim().split(',').map(|x| x.trim().parse::<u32>().unwrap()).collect(),
            operation: self.parse_monkey_operation(&group[2]),
            test: &|_| true,
            test_pass_target: 0,
            test_fail_target: 0,
        }
    }

    pub fn solve(&self) {
        let monkeys = self.parse();
        for m in monkeys {
            println!("id: {} operation: {}", m.id, m.operation.name())
        }
    }
}
