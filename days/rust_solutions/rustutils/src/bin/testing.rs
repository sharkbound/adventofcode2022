use rustutils::arg_parser::args::{ArgCollection, ArgDataType};

fn main() {
    let mut parser = ArgCollection::new();
    parser.bind_env_args();
    parser.add_optional("output", Some("o"), "output file to write to", ArgDataType::String, "output.txt");
    parser.add_bool_flag("silent", Some("s"), "optional bool flag", false);
    // parser.print_help_and_exit("FOOL! YOU CALLED ME! HAHAHAHA!");
    println!("{:?}", parser.parse_bool("silent"));
}