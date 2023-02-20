use std::fs;
use std::process::Command;

use clap::{Parser};
use systemd_parser::parser::systemd::{SystemdFile};
use systemd_parser::parser;

fn main() {

    let args = Cli::parse();

    if args.ipaddr {

        let cmd_out = Command::new("/usr/sbin/ip").arg("a").output().expect("command out");
        let parse_result = parser::ipaddr::parse(String::from_utf8(cmd_out.stdout).unwrap().as_str());
        match parse_result {
            Ok(_) => println!("Successful parse"),
            Err(e) => eprintln!("failed to parse: {e}")
        }
        return;
    }

    if args.validate {
        match parser::systemd::parse(args.in_file.as_str()) {
            Ok(_) => {
                println!("File has valid syntax");
                std::process::exit(0)},
            Err(e) => {
                eprintln!("An error occurred: \n{e}");
                std::process::exit(1)
            }
        }
    }

    if args.parse {
        let unparsed_file = fs::read_to_string(args.in_file).expect("cannot read file");

        let parsed = parser::systemd::parse(unparsed_file.as_str()).unwrap();

        let json_data = serde_json::to_string_pretty(&parsed).unwrap();

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
    pub validate: bool,

    #[arg(short, long)]
    pub in_file: String,

    #[arg(short, long)]
    pub out_file: String,

    #[arg(short='a', long)]
    pub ipaddr: bool,
}
