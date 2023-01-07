use std::collections::VecDeque;
use std::fmt::{Debug, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rustutils::{str_split};
use day_11::{OperationSign, SubstituteValue};

/*
Operation shows how your worry level changes as that monkey inspects an item.

After each monkey inspects an item but before it tests your worry level,
    your relief that the monkey's inspection didn't damage the item causes your worry level...
    to be divided by three and rounded down to the nearest integer.

The monkeys take turns inspecting and throwing items.
On a single monkey's turn, it inspects and throws all of the items it is holding one at a time and in the order listed.
*/


// #[derive(Debug)]
struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    operation: OperationSign,
    div_test: u64,
    true_target: usize,
    false_target: usize,
    inspect_count: u64,
}

impl Monkey {
    fn add_item(&mut self, item: u64) {
        self.items.push_back(item);
    }

    fn next_item(&mut self) -> Option<u64> {
        self.items.pop_front()
    }

    fn inspect(&mut self, item: u64) -> u64 {
        let result = self.operation.apply(item);
        self.inspect_count += 1;
        result
    }

    fn throw_target(&self, item: u64) -> usize {
        match item % self.div_test {
            0 => self.true_target,
            _ => self.false_target,
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ Monkey: ID: {}, ITEMS: {:?}, INSPECT_COUNT: {} }}", self.id, self.items, self.inspect_count)
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

pub struct Day11Part1<'a> {
    input_path: &'a Path,
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
                SubstituteValue::LITERAL(int.parse::<u64>()
                    .expect(format!(r#"invalid u64 when parsing substitute value "{}""#, string).as_str()))
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

    fn parse_u64_at_end_of_line(line: &str) -> u64 {
        line.trim()[line.trim().rfind(" ").unwrap() + 1..].parse::<u64>().unwrap()
    }

    fn parse_usize_at_end_of_line(line: &str) -> usize {
        line.trim()[line.trim().rfind(" ").unwrap() + 1..].parse::<usize>().unwrap()
    }

    fn parse_monkey(&self, group: Vec<String>) -> Monkey {
        Monkey {
            id: group[0].chars().into_iter().filter(|x| x.is_digit(10)).collect::<String>().parse::<u64>().unwrap(),
            items: group[1][group[1].find(":").unwrap() + 1..].trim().split(',').map(|x| x.trim().parse::<u64>().unwrap()).collect(),
            operation: self.parse_monkey_operation(group[2].as_str()),
            div_test: Self::parse_u64_at_end_of_line(group[3].as_str()),
            true_target: Self::parse_usize_at_end_of_line(group[4].as_str()),
            false_target: Self::parse_usize_at_end_of_line(group[5].as_str()),
            inspect_count: 0,
        }
    }

    fn do_round_for_single_monkey(monkey: &mut Monkey) -> Vec<(usize, u64)> {
        let mut dests = vec![];
        while let Some(item) = monkey.next_item() {
            let div_result = monkey.inspect(item) / 3;
            dests.push((monkey.throw_target(div_result), div_result));
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

        monkeys.sort_by_key(|x| x.inspect_count);
        println!("day 11 part 1 answer => {}", monkeys.pop().unwrap().inspect_count * monkeys.pop().unwrap().inspect_count)
    }
}
