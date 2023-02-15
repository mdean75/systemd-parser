use std::env::{args_os};
use std::ffi::OsString;
use std::fs;

use systemd_parser::parser::systemd::{SystemdFile};

fn main() {

    let args: Vec<OsString> = args_os().collect();
    println!("{:?}", args.get(1).unwrap());
    let unparsed_file = fs::read_to_string(args.get(1).unwrap()).expect("cannot read file");

    let parsed = systemd_parser::parser::systemd::parse(unparsed_file.as_str());

    if let Some(out_file) = args.get(2) {
        fs::write(out_file, serde_json::to_string_pretty(&parsed).unwrap()).expect("TODO: panic message");
    } else {
        println!("{}", serde_json::to_string_pretty(&parsed).unwrap());

    }
    let json_data = serde_json::to_string_pretty(&parsed).unwrap();

    let mut d: SystemdFile = serde_json::from_str(json_data.as_str()).unwrap();
    d.service.exec_start[0] = "ExecStart=/new/path/to/binary".to_string();

    println!("deserialized: \n\n{d}");
    fs::write("updated_unit.service", d.to_string()).unwrap();
}
