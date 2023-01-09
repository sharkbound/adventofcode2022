pub enum ArgType {
    Required,
    Optional,
    Flag,
}

pub enum ArgOptionValueType {
    Bool,
    Int,
    UnsignedInt,
    String,
}

pub enum ParsedArgValue {
    Bool(bool),
    Int(i64),
    UnsignedInt(u64),
    String(String),
}

pub trait ArgOption {
    fn arg_type(&self) -> &ArgType;
    fn arg_value_type(&self) -> &ArgOptionValueType;
    fn name(&self) -> &str;
    fn alias(&self) -> Option<&str>;
    fn help(&self) -> &str;
    fn default(&self) -> Option<&str>;
    fn parse(&self, value: &str) -> Option<ParsedArgValue>;
}

pub struct BasicArg {
    pub value_type: ArgOptionValueType,
    pub arg_type: ArgType,
    pub name: String,
    pub alias: Option<String>,
    pub help: String,
    pub default: Option<String>,
}

impl BasicArg {
    pub fn new(arg_type: ArgType, value_type: ArgOptionValueType, name: String, alias: Option<String>, help: String, default: Option<String>) -> Self {
        Self { arg_type, name, alias, help, default, value_type }
    }

    pub fn boxed(arg_type: ArgType, value_type: ArgOptionValueType, name: String, alias: Option<String>, help: String, default: Option<String>) -> Box<BasicArg> {
        Box::new(Self { arg_type, name, alias, help, default, value_type })
    }
}

impl ArgOption for BasicArg {
    fn arg_type(&self) -> &ArgType {
        &self.arg_type
    }

    fn arg_value_type(&self) -> &ArgOptionValueType {
        todo!()
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn alias(&self) -> Option<&str> {
        match &self.alias {
            Some(alias) => Some(alias.as_str()),
            None => None,
        }
    }

    fn help(&self) -> &str {
        &self.help
    }

    fn default(&self) -> Option<&str> {
        match &self.default {
            Some(default) => Some(default),
            None => None,
        }
    }

    fn parse(&self, value: &str) -> Option<ParsedArgValue> {
        match &self.value_type {
            ArgOptionValueType::String => {
                Some(ParsedArgValue::String(value.to_owned()))
            }

            ArgOptionValueType::Bool => {
                match value.to_lowercase().trim() {
                    "true" => Some(ParsedArgValue::Bool(true)),
                    _ => Some(ParsedArgValue::Bool(false)),
                }
            }

            ArgOptionValueType::Int => {
                match value.parse::<i64>() {
                    Ok(value) => Some(ParsedArgValue::Int(value)),
                    Err(_) => None,
                }
            }

            ArgOptionValueType::UnsignedInt => {
                match value.parse::<u64>() {
                    Ok(value) => Some(ParsedArgValue::UnsignedInt(value)),
                    Err(_) => None,
                }
            }
        }
    }
}
