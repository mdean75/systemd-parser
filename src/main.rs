extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;
use pest::Parser;

#[derive(Debug, Clone)]
/// Represents a variant type of Systemd unit file values.
pub enum SystemdValue {
    /// Wraps a String vector that contains multiple values for a duplicate key.
    List(Vec<String>),
    /// Wraps a String value of a respective key in the systemd unit file.
    Str(String),
}

#[derive(Parser)]
#[grammar = "systemd.pest"]
pub struct SystemDParser;

pub enum SectionType {
    Unit(String),
    Service(String),
    Install(String)
}

#[derive(Default, Debug)]
pub struct UnitSection {
    head: String,
    description: String,
    after: Option<Vec<String>>,
    documentation: Option<String>,
    requires: Option<Vec<String>>,
    wants: Option<Vec<String>>,
    binds_to: Option<String>, // not sure if this can be repeated
    before: Option<Vec<String>>,
    conflicts: Option<Vec<String>>,
    condition: Option<String>, // not sure if this can be repeated
    assert: Option<Vec<String>>,
}

#[derive(Default, Debug)]
pub struct ServiceSection {
    head: String,
    exec_start: Vec<String>,
}

#[derive(Default, Debug)]
pub struct InstallSection {
    head: String
}

#[derive(Default, Debug)]
pub struct SystemdFile {
    // section: SectionType
    unit: UnitSection,
    service: ServiceSection,
    install: InstallSection,
}

fn main() {

    let unparsed_file = fs::read_to_string("unit.service").expect("cannot read file");


    let file = SystemDParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap(); // get and unwrap the `file` rule; never fails

    let mut file_struct = SystemdFile::default();


    for line in file.into_inner() {
        match line.as_rule() {
            Rule::unit_section => {
                for item in line.into_inner() {
                    match item.as_rule() {
                        Rule::unit_section_heading => {
                            file_struct.unit.head = item.as_span().as_str().to_string();
                        }

                        Rule::unit_section_props => {
                            for prop in item.into_inner() {
                                match prop.as_rule() {
                                    Rule::description => {
                                        file_struct.unit.description = prop.as_span().as_str().to_string()
                                    }
                                    Rule::after => {
                                        if let Some(mut x) = file_struct.unit.after.clone() {
                                            x.push(prop.as_str().to_string());
                                            file_struct.unit.after = Some(x);
                                        } else {
                                            file_struct.unit.after = Some(vec![prop.as_str().to_string()]);
                                        }
                                    }

                                    _ => {}
                                }
                            }
                        }

                        _ => {println!("other: {:?}", item)}
                    }
                }
            }
            Rule::service_section => {
                for item in line.into_inner() {
                    match item.as_rule() {
                        Rule::service_section_heading => {
                            file_struct.service.head = item.as_span().as_str().to_string()
                        },
                        Rule::exec_start => {
                            file_struct.service.exec_start = item.as_span().as_str().to_string().split_whitespace().map(|v| v.to_string()).collect();
                        },
                        _ => {}
                    }
                }
            }
            Rule::install_section_heading => {
                file_struct.install.head = line.as_span().as_str().to_string();
            }

            _ => {}
        }
    }

    println!("file struct: {:?}", file_struct);
}
