use std::collections::HashMap;

#[derive(Debug)]
pub enum ArgOptionType {
    FLAG,
    OPTIONAL { default: String },
    REQUIRED,
}

#[derive(Debug)]
pub struct ArgOption {
    pub name: String,
    pub arg_type: ArgOptionType,
}

impl ArgOption {
    pub fn new(name: String, arg_type: ArgOptionType) -> Self {
        Self {
            name,
            arg_type,
        }
    }

    pub fn default(&self) -> Option<&String> {
        match self.arg_type {
            ArgOptionType::OPTIONAL { ref default } => Some(default),
            _ => None
        }
    }
}

struct ArgParser {
    options: Vec<ArgOption>,
}

impl ArgParser {
    pub fn new(options: Vec<ArgOption>) -> Self {
        Self {
            options,
        }
    }
}