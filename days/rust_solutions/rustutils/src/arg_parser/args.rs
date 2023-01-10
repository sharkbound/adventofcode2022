use std::fmt::{Debug};
use crate::errors::GeneralError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArgDataType {
    String,
    Bool,

    I32,
    I64,

    U32,
    U64,

    F32,
    F64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArgType {
    BoolFlag { default: bool },
    Required,
    Optional { default: String },
}


pub struct Arg {
    pub name: String,
    pub alias: Option<String>,
    pub help: String,
    pub data_type: ArgDataType,
    pub ty: ArgType,
}

impl Arg {
    fn new(name: &str, alias: Option<&str>, help: &str, data_type: ArgDataType, ty: ArgType) -> Self {
        Arg {
            name: name.to_owned(),
            alias: alias.map(|s| s.to_owned()),
            help: help.to_owned(),
            ty,
            data_type,
        }
    }

    pub fn dashed_name(&self) -> String {
        format!("--{}", self.name)
    }

    pub fn dashed_alias(&self) -> Option<String> {
        self.alias.as_ref().map(|s| format!("-{}", s))
    }
}

pub struct ArgCollection {
    args: Vec<Arg>,
    strings: Vec<String>,
}

impl<'a> ArgCollection {
    pub fn new() -> Self {
        ArgCollection { args: Vec::new(), strings: vec![] }
    }

    pub fn add_arg(&mut self, name: &str, alias: Option<&str>, help: &str, data_type: ArgDataType, ty: ArgType) -> &Arg {
        self.args.push(Arg::new(name, alias, help, data_type, ty));
        self.args.last().unwrap()
    }

    pub fn add_optional(&mut self, name: &str, alias: Option<&str>, help: &str, data_type: ArgDataType, default: &str) -> &Arg {
        self.add_arg(name, alias, help, data_type, ArgType::Optional { default: default.to_owned() })
    }

    pub fn add_required(&mut self, name: &str, alias: Option<&str>, help: &str, data_type: ArgDataType) -> &Arg {
        self.add_arg(name, alias, help, data_type, ArgType::Required)
    }

    pub fn add_bool_flag(&mut self, name: &str, alias: Option<&str>, help: &str, default: bool) -> &Arg {
        self.add_arg(name, alias, help, ArgDataType::Bool, ArgType::BoolFlag { default })
    }

    pub fn bind(&mut self, strings: Vec<String>) {
        self.strings = strings;
    }

    pub fn get_arg_ref(&self, name: &str) -> Option<&Arg> {
        self.args.iter().find(|a| a.name == name)
    }

    pub fn bind_env_args(&mut self) {
        // dbg!(std::env::args().collect::<Vec<String>>());
        self.strings = std::env::args().collect();
    }

    pub fn print_help_and_exit(&self, error: &str) {
        let mut output = String::new();
        output.push_str(&format!("Invalid Usage: {}", error));

        for arg in self.args.iter() {
            output.push('\n');
            if let Some(alias) = &arg.alias {
                output.push_str(&format!("-{} | ", alias));
            }
            output.push_str(&format!("--{} => {}", arg.name, arg.help));
        }

        println!("{}", output);
        std::process::exit(1);
    }

    fn get_strings_starting_at_arg_match(&self, arg: &Arg, length: usize) -> Option<&[String]> {
        // todo: add support for aliases matching
        let match_index = self.strings.iter().position(|s| s.eq_ignore_ascii_case(arg.dashed_name().as_str()))?;
        let strings = self.strings.get(match_index..(match_index + length))?;
        if strings.len() == 0 {
            return None;
        }
        Some(strings)
    }

    pub fn parse_bool(&self, name: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let arg = self.args.iter()
            .find(|x| x.name == name)
            .ok_or(GeneralError::boxed("ArgNotFoundError", &format!("Could not find any arg by the name: {}", name)))?;

        let strings = self.get_strings_starting_at_arg_match(arg, 1)
            .ok_or(GeneralError::boxed("ArgMatchNotFound", &format!("Could not find any match for arg: {} ", arg.name)))?;

        match strings.len() {
            1 => Ok(true),
            2.. => {
                strings.get(1).and_then(|x| x.parse::<bool>().ok()).ok_or(
                    GeneralError::boxed("ParseBoolError", &format!(r#"Could not convert "{}" to a bool"#, strings[1]))
                )
            }
            _ => Err(GeneralError::boxed("Error", &format!("this should not happen")))
        }
    }
}

