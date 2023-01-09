use std::collections::HashMap;
use crate::arg_parser::args::ArgOption;

pub struct ArgParser<'a> {
    options: Vec<Box<dyn ArgOption>>,
    string_to_option: HashMap<&'a str, &'a dyn ArgOption>,
}

impl<'a> ArgParser<'a> {
    pub fn new(options: Vec<Box<dyn ArgOption>>) -> Self {
        let mut string_to_option: HashMap<&'a str, &'a dyn ArgOption> = HashMap::new();
        for option in options.iter() {
            string_to_option.insert(option.name(), option.as_ref());
            if let Some(alias) = option.alias() {
                string_to_option.insert(alias, option.as_ref());
            };
        }

        Self {
            options,
            string_to_option,
        }
    }

    pub fn parse(&self, args: &Vec<String>) -> ParsedArgs {
        todo!()
    }
}

struct ParsedArgs {
    keystore: HashMap<String, String>,
}
//
// impl ParsedArgs {
//     pub fn new(parsed: HashMap<String, String>) -> Self {
//         Self {}
//     }
// }