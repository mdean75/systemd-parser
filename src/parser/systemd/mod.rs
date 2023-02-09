use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use pest::Parser;
use pest_derive::Parser;

#[derive(Debug, Clone)]
/// Represents a variant type of Systemd unit file values.
pub enum SystemdValue {
    /// Wraps a String vector that contains multiple values for a duplicate key.
    List(Vec<String>),
    /// Wraps a String value of a respective key in the systemd unit file.
    Str(String),
}

#[derive(Parser)]
#[grammar = "grammar/systemd.pest"]
pub struct SystemDParser;

pub enum SectionType {
    Unit(String),
    Service(String),
    Install(String)
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct UnitSection {
    pub head: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documentation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    requires: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wants: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binds_to: Option<String>, // not sure if this can be repeated
    #[serde(skip_serializing_if = "Option::is_none")]
    before: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conflicts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>, // not sure if this can be repeated
    #[serde(skip_serializing_if = "Option::is_none")]
    assert: Option<Vec<String>>,
}

impl Display for UnitSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push_str(format!("{}\n", self.head.as_str()).as_str());
        s.push_str(format!("{}\n", self.description.as_str()).as_str());

        if let Some(after) = self.after.as_ref() {
            s.push_str(after.join("\n").as_str()); // add a space once the line ending chars are removed from parsing
        }

        if let Some(doc) = self.documentation.as_ref() {
            s.push_str(doc);
        }

        if let Some(req) = self.requires.as_ref() {
            s.push_str(req.join(" ").as_str());
        }

        if let Some(wants) = self.wants.as_ref() {
            s.push_str(wants.join(" ").as_str());
        }

        if let Some(binds) = self.binds_to.as_ref() {
            s.push_str(binds);
        }

        if let Some(before) = self.before.as_ref() {
            s.push_str(before.join(" ").as_str());
        }

        if let Some(conflicts) = self.conflicts.as_ref() {
            s.push_str(conflicts.join(" ").as_str());
        }

        if let Some(condition) = self.condition.as_ref() {
            s.push_str(condition);
        }

        if let Some(assert) = self.assert.as_ref() {
            s.push_str(assert.join(" ").as_str());
        }

        writeln!(f, "{s}")

    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ServiceSection {
    pub head: String,
    pub exec_start: Vec<String>,
}

impl Display for ServiceSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n{}", self.head, self.exec_start.join(" "))
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct InstallSection {
    pub head: String
}

impl Display for InstallSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.head)
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct SystemdFile {
    // section: SectionType
    pub unit: UnitSection,
    pub service: ServiceSection,
    pub install: InstallSection,
}

impl Display for SystemdFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n{}\n{}", self.unit, self.service, self.install)
    }
}

pub fn parse(unparsed_file: &str) -> SystemdFile {
    let file = SystemDParser::parse(Rule::file, &unparsed_file)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap(); // get and unwrap the `file` rule; never fails

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
                                        match file_struct.unit.after {
                                            None => { file_struct.unit.after = Some(vec![prop.as_str().to_string()]); }
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.unit.after = Some(x);
                                            }
                                        }
                                    }

                                    _ => {}
                                }
                            }
                        }

                        _ => {println!("other: {item:?}")}
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

    file_struct
}
