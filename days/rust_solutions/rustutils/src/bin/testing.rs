use rustutils::arg_parser::args::{ArgOption, ArgOptionValueType, ArgType, BasicArg};
use rustutils::arg_parser::parsers::ArgParser;

fn main() {
    let parser = ArgParser::new(vec![
        BasicArg::boxed(
            ArgType::Optional,
            ArgOptionValueType::UnsignedInt,
            "passes".to_owned(),
            None,
            "the number of passes to do".to_owned(), None)
    ]);

    dbg!(BasicArg::boxed(
        ArgType::Optional,
        ArgOptionValueType::UnsignedInt,
        "passes".to_owned(),
        None,
        "the number of passes to do".to_owned(), None).parse("-1")
    );
}