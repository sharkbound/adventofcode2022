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
}

pub struct ArgCollection {
    args: Vec<Arg>,
    strings: Option<Vec<String>>,
}

impl<'a> ArgCollection {
    pub fn new() -> Self {
        ArgCollection { args: Vec::new(), strings: None }
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
        self.strings = Some(strings);
    }

    pub fn get_arg_ref(&self, name: &str) -> Option<&Arg> {
        self.args.iter().find(|a| a.name == name)
    }

    pub fn bind_env_args(&mut self) {
        self.strings = std::env::args().collect()
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
}

type BoxedError = Box<dyn std::error::Error>;

impl ArgCollection {
    fn search_strings_for_arg(&self, arg: &Arg) -> Option<String> {
        match self.strings {
            Some(ref strings) => {
                Self::_search_for_arg_match(arg, strings)
            }
            None => None,
        }
    }

    fn _search_for_arg_match(arg: &Arg, strings: &Vec<String>) -> Option<String> {
        match strings.iter().position(|s| s == arg.name.as_str()) {
            Some(index) => {
                Self::_check_match_length(arg, strings, index)
            }
            None => None,
        }
    }

    fn _check_match_length(arg: &Arg, strings: &Vec<String>, index: usize) -> Option<String> {
        match arg.ty {
            ArgType::Required => {
                todo!()
            }
            _ => None
        }
    }
    pub fn parse_bool(&self, name: &str) -> Result<bool, BoxedError> {
        todo!()
    }
}
