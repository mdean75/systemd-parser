use std::fs;

use systemd_parser::parser::systemd::{SystemdFile};

fn main() {

    let unparsed_file = fs::read_to_string("unit.service").expect("cannot read file");

    let file_struct = systemd_parser::parser::systemd::parse(unparsed_file.as_str());

    println!("{}", serde_json::to_string_pretty(&file_struct).unwrap());
    let json_data = serde_json::to_string_pretty(&file_struct).unwrap();

    let mut d: SystemdFile = serde_json::from_str(json_data.as_str()).unwrap();
    d.service.exec_start[0] = "ExecStart=/new/path/to/binary".to_string();

    println!("deserialized: \n\n{d}");
    fs::write("updated_unit.service", d.to_string()).unwrap();
}
