use std::collections::VecDeque;
use std::fmt::{Debug};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rustutils::{str_split};

/*
Operation shows how your worry level changes as that monkey inspects an item.

After each monkey inspects an item but before it tests your worry level,
    your relief that the monkey's inspection didn't damage the item causes your worry level...
    to be divided by three and rounded down to the nearest integer.

The monkeys take turns inspecting and throwing items.
On a single monkey's turn, it inspects and throws all of the items it is holding one at a time and in the order listed.
*/

pub struct Day11Part1<'a> {
    input_path: &'a Path,
}

#[derive(Debug)]
enum OperationSign {
    PLUS(SubstituteValue, SubstituteValue),
    MUL(SubstituteValue, SubstituteValue),
}

impl OperationSign {
    pub fn apply(&self, item: u32) -> u32 {
        match self {
            OperationSign::PLUS(sub1, sub2) => sub1.resolve(item) + sub2.resolve(item),
            OperationSign::MUL(sub1, sub2) => sub1.resolve(item) * sub2.resolve(item),
        }
    }
}

#[derive(Debug)]
enum SubstituteValue {
    ITEM,
    LITERAL(u32),
}

impl SubstituteValue {
    fn resolve(&self, item: u32) -> u32 {
        match &self {
            SubstituteValue::ITEM => item,
            SubstituteValue::LITERAL(v) => *v,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    items: VecDeque<u32>,
    operation: OperationSign,
    div_test: u32,
    true_target: usize,
    false_target: usize,
    inspect_count: u32,
}

impl Monkey {
    fn add_item(&mut self, item: u32) {
        self.items.push_back(item);
    }

    fn next_item(&mut self) -> Option<u32> {
        self.items.pop_front()
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
                Err(_) => {
                    println!("EERRRRROOORRR");
                    break;
                }
            }
        };

        if !current_group.is_empty() {
            groups.push(current_group);
        }

        groups.into_iter().map(|group| self.parse_monkey(group)).collect()
    }

    fn parse_substitute(string: &str) -> SubstituteValue {
        match string {
            "old" => SubstituteValue::ITEM,
            int => {
                SubstituteValue::LITERAL(int.parse::<u32>()
                    .expect(format!(r#"invalid u32 when parsing substitute value "{}""#, string).as_str()))
            }
        }
    }

    fn parse_monkey_operation(&self, line: &str) -> OperationSign {
        let parts = str_split!(line);
        let calc_parts = parts[parts.iter().position(|x| *x == "=").unwrap() + 1..].iter().collect::<Vec<_>>();
        match *calc_parts[1] {
            "+" => OperationSign::PLUS(Self::parse_substitute(calc_parts[0]), Self::parse_substitute(calc_parts[2])),
            "*" => OperationSign::MUL(Self::parse_substitute(calc_parts[0]), Self::parse_substitute(calc_parts[2])),
            symbol => { panic!(r#"unexpected operation "{}""#, symbol) }
        }
    }

    fn parse_u32_at_end_of_line(line: &str) -> u32 {
        line.trim()[line.trim().rfind(" ").unwrap() + 1..].parse::<u32>().unwrap()
    }

    fn parse_usize_at_end_of_line(line: &str) -> usize {
        line.trim()[line.trim().rfind(" ").unwrap() + 1..].parse::<usize>().unwrap()
    }

    fn parse_monkey(&self, group: Vec<String>) -> Monkey {
        Monkey {
            id: group[0].chars().into_iter().filter(|x| x.is_digit(10)).collect::<String>().parse::<u32>().unwrap(),
            items: group[1][group[1].find(":").unwrap() + 1..].trim().split(',').map(|x| x.trim().parse::<u32>().unwrap()).collect(),
            operation: self.parse_monkey_operation(group[2].as_str()),
            div_test: Self::parse_u32_at_end_of_line(group[3].as_str()),
            true_target: Self::parse_usize_at_end_of_line(group[4].as_str()),
            false_target: Self::parse_usize_at_end_of_line(group[5].as_str()),
            inspect_count: 0,
        }
    }

    fn do_round_for_single_monkey(monkey: &mut Monkey) -> Vec<(usize, u32)> {
        let mut dests = vec![];
        while let Some(item) = monkey.next_item() {
            let div_result = monkey.operation.apply(item) / 3;

            if div_result / monkey.div_test == 0 {
                dests.push((monkey.true_target, div_result));
            } else {
                dests.push((monkey.false_target, div_result));
            }
        }
        dests
    }

    fn do_round(monkeys: &mut Vec<Monkey>) {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            for (dest, val) in Self::do_round_for_single_monkey(monkey) {
                monkeys.get_mut(dest).unwrap().add_item(val);
            }
        }
    }

    pub fn solve(&self) {
        let mut monkeys = self.parse();
        for _ in 0..20 {
            Self::do_round(&mut monkeys);
        }

        /*
            these values after round 20 are wrong:
                0: [21, 13, 12, 6]
                1: [30, 22, 26, 29, 17, 17]
                2: []
                3: []

            should be:
                Monkey 0: 10, 12, 14, 26, 34
                Monkey 1: 245, 93, 53, 199, 115
                Monkey 2:
                Monkey 3:
         */
        for m in monkeys {
            println!("{}: {:?}", m.id, m.items);
        }
    }
}
