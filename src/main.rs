use std::fs;

use clap::{Parser};
use systemd_parser::parser::systemd::{SystemdFile};

fn main() {

    let args = Cli::parse();

    // let args: Vec<OsString> = args_os().collect();
    // println!("{:?}", args.get(1).unwrap());
    if args.parse {
        let unparsed_file = fs::read_to_string(args.in_file).expect("cannot read file");

        let parsed = systemd_parser::parser::systemd::parse(unparsed_file.as_str());

        // if let Some(out_file) = pargs.out_file {
        //     fs::write(pargs.out_file, serde_json::to_string_pretty(&parsed).unwrap()).expect("TODO: panic message");
        // } else {
        //     println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
        //
        // }
        let json_data = serde_json::to_string_pretty(&parsed).unwrap();

        // let mut d: SystemdFile = serde_json::from_str(json_data.as_str()).unwrap();
        // d.service.exec_start[0] = "ExecStart=/new/path/to/binary".to_string();

        // println!("deserialized: \n\n{d}");
        fs::write(args.out_file, json_data.to_string()).unwrap();
    } else {
        let json_file = fs::read_to_string(args.in_file).expect("cannot read file");
        let systemd_file: SystemdFile = serde_json::from_str(json_file.as_str()).unwrap();

        fs::write(args.out_file, systemd_file.to_string()).unwrap();
    }

}

#[derive(Parser)]
#[command(version)]
#[command(about, long_about = None, arg_required_else_help(true))]
pub struct Cli {

    #[arg(short, long)]
    pub parse: bool,

    // #[arg(short, long)]
    // pub build: bool,

    #[arg(short, long)]
    pub in_file: String,

    #[arg(short, long)]
    pub out_file: String,
}
