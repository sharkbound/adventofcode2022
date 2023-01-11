use rustutils::arg_parser::args::{ArgCollection, ArgDataType};

fn main() {
    let mut parser = ArgCollection::new();
    parser.bind_env_args();
    parser.add_optional("output", Some("o"), "output file to write to", ArgDataType::String, "LOL");
    parser.add_bool_flag("silent", Some("s"), "optional bool flag", false);
    parser.add_required("day", Some("d"), "the day", ArgDataType::I32);
    parser.check();
    println!("silent: {:?}\noutput: {:?}\nday: {:?}", parser.parse_bool_flag("silent"), parser.parse_string("output"), parser.parse_i32("day"));
}