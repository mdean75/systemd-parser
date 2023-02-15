use std::fs;

use clap::{Parser};
use systemd_parser::parser::systemd::{SystemdFile};

fn main() {

    let args = Cli::parse();

    if args.parse {
        let unparsed_file = fs::read_to_string(args.in_file).expect("cannot read file");

        let parsed = systemd_parser::parser::systemd::parse(unparsed_file.as_str());

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
    pub in_file: String,

    #[arg(short, long)]
    pub out_file: String,
}
