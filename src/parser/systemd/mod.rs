use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, format, Formatter};
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
#[grammar = "simplified_grammar.pest"]
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
    documentation: Option<Vec<String>>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    condition_path_exists: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    comments: Option<Vec<String>>,
}

impl Display for UnitSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        if let Some(comment) = self.comments.as_ref() {
            s.push_str(format!("{}\n", comment.join("\n").as_str()).as_str());
        }
        s.push_str(format!("{}\n", self.head.as_str()).as_str());
        s.push_str(format!("{}\n", self.description.as_str()).as_str());

        if let Some(after) = self.after.as_ref() {
            s.push_str(format!("{}\n", after.join("\n").as_str()).as_str()); // add a space once the line ending chars are removed from parsing
        }

        if let Some(doc) = self.documentation.as_ref() {
            s.push_str(format!("{}\n", doc.join("\n")).as_str());
        }

        if let Some(req) = self.requires.as_ref() {
            s.push_str(format!("{}\n", req.join(" ").as_str()).as_str());
        }

        if let Some(wants) = self.wants.as_ref() {
            s.push_str(format!("{}\n", wants.join(" ").as_str()).as_str());
        }

        if let Some(binds) = self.binds_to.as_ref() {
            s.push_str(format!("{}\n", binds).as_str());
        }

        if let Some(before) = self.before.as_ref() {
            s.push_str(format!("{}\n", before.join(" ").as_str()).as_str());
        }

        if let Some(conflicts) = self.conflicts.as_ref() {
            s.push_str(format!("{}\n", conflicts.join(" ").as_str()).as_str());
        }

        if let Some(condition) = self.condition.as_ref() {
            s.push_str(format!("{}\n", condition).as_str());
        }

        if let Some(assert) = self.assert.as_ref() {
            s.push_str(format!("{}\n", assert.join(" ").as_str()).as_str());
        }

        if let Some(condition_path) = self.condition_path_exists.as_ref() {
            s.push_str(format!("{}\n", condition_path).as_str());
        }

        writeln!(f, "{s}")

    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct ServiceSection {
    pub head: String,
    pub service_type: String,
    pub exec_start: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_pre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_post: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reload: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_stop: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_post: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_sec: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kill_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_prevent_exit_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_directory_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_no_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<String>>
}

impl Display for ServiceSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        s.push_str(format!("{}\n", self.head.as_str()).as_str());
        if let Some(comment) = self.comments.as_ref() {
            s.push_str(format!("{}\n", comment.join("\n")).as_str());
        }
        s.push_str(format!("{}\n", self.service_type.as_str()).as_str());
        s.push_str(format!("{}\n", self.exec_start.join(" ").as_str()).as_str());

        if let Some(start_pre) = self.start_pre.as_ref() {
            s.push_str(format!("{}\n", start_pre).as_str());
        }
        if let Some(start_post) = self.start_post.as_ref() {
            s.push_str(format!("{}\n", start_post).as_str());
        }
        if let Some(reload) = self.reload.as_ref() {
            s.push_str(format!("{}\n", reload.join("\n")).as_str());
        }
        if let Some(exec_stop) = self.exec_stop.as_ref() {
            s.push_str(format!("{}\n", exec_stop).as_str());
        }
        if let Some(stop_post) = self.stop_post.as_ref() {
            s.push_str(format!("{}\n", stop_post).as_str());
        }
        if let Some(restart_sec) = self.restart_sec.as_ref() {
            s.push_str(format!("{}\n", restart_sec).as_str());
        }
        if let Some(restart) = self.restart.as_ref() {
            s.push_str(format!("{}\n", restart).as_str());
        }
        if let Some(timeout_sec) = self.timeout_sec.as_ref() {
            s.push_str(format!("{}\n", timeout_sec).as_str());
        }
        if let Some(environment_file) = self.environment_file.as_ref() {
            s.push_str(format!("{}\n", environment_file).as_str());
        }
        if let Some(kill_mode) = self.kill_mode.as_ref() {
            s.push_str(format!("{}\n", kill_mode).as_str());
        }
        if let Some(restart_prevent_exit_status) = self.restart_prevent_exit_status.as_ref() {
            s.push_str(format!("{}\n", restart_prevent_exit_status).as_str());
        }
        if let Some(runtime_directory) = self.runtime_directory.as_ref() {
            s.push_str(format!("{}\n", runtime_directory).as_str());
        }
        if let Some(runtime_directory_mode) = self.runtime_directory_mode.as_ref() {
            s.push_str(format!("{}\n", runtime_directory_mode).as_str());
        }
        if let Some(environment) = self.environment.as_ref() {
            s.push_str(format!("{}\n", environment).as_str());
        }
        if let Some(standard_output) = self.standard_output.as_ref() {
            s.push_str(format!("{}\n", standard_output).as_str());
        }
        if let Some(limit_no_file) = self.limit_no_file.as_ref() {
            s.push_str(format!("{}\n", limit_no_file).as_str());
        }

        writeln!(f, "{s}")
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct InstallSection {
    pub head: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wanted_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub also: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<String>>
}

impl Display for InstallSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        s.push_str(format!("{}\n", self.head.as_str()).as_str());
        if let Some(comment) = self.comments.as_ref() {
            s.push_str(format!("{}\n", comment.join(" ").as_str()).as_str());
        }
        if let Some(wanted_by) = self.wanted_by.as_ref() {
            s.push_str(format!("{}\n", wanted_by).as_str());
        }
        if let Some(required_by) = self.required_by.as_ref() {
            s.push_str(format!("{}\n", required_by).as_str());
        }
        if let Some(alias) = self.alias.as_ref() {
            s.push_str(format!("{}\n", alias).as_str());
        }
        if let Some(also) = self.also.as_ref() {
            s.push_str(format!("{}\n", also).as_str());
        }
        if let Some(default_instance) = self.default_instance.as_ref() {
            s.push_str(format!("{}\n", default_instance).as_str());
        }

        writeln!(f, "{s}")
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
        write!(f, "{}{}{}", self.unit, self.service, self.install)
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
            Rule::unit => {
                for item in line.into_inner() {
                    match item.as_rule() {
                        Rule::unit_tag => {
                            file_struct.unit.head = item.as_span().as_str().to_string();
                        }

                        Rule::unit_prop => {
                            for prop in item.into_inner() {
                                match prop.as_rule() {
                                    Rule::description => {
                                        file_struct.unit.description = prop.as_span().as_str().to_string()
                                    },
                                    Rule::documentation => {
                                        match file_struct.unit.documentation {
                                            None => file_struct.unit.documentation = Some(vec![prop.as_span().as_str().to_string()]),
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.unit.documentation = Some(x);
                                            }
                                        }
                                    },
                                    Rule::requires => {
                                        match file_struct.unit.requires {
                                            None => { file_struct.unit.requires = Some(vec![prop.as_str().to_string()]); }
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.unit.requires = Some(x);
                                            }
                                        }
                                    },
                                    Rule::wants => {
                                        match file_struct.unit.wants {
                                            None => file_struct.unit.wants = Some(vec![prop.as_str().to_string()]),
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.unit.wants = Some(x);
                                            }
                                        }
                                    },
                                    Rule::binds_to => {
                                        file_struct.unit.binds_to = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::before => {
                                        match file_struct.unit.before {
                                            None => file_struct.unit.before = Some(vec![prop.as_str().to_string()]),
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.unit.before = Some(x);

                                            }
                                        }
                                    },
                                    Rule::after => {
                                        match file_struct.unit.after {
                                            None => { file_struct.unit.after = Some(vec![prop.as_str().to_string()]); }
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.unit.after = Some(x);
                                            }
                                        }
                                    },
                                    Rule::conflicts => {
                                        match file_struct.unit.conflicts {
                                            None => file_struct.unit.comments = Some(vec![prop.as_str().to_string()]),
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.unit.conflicts = Some(x);
                                            }
                                        }
                                    },
                                    Rule::condition => {
                                        file_struct.unit.condition = Some(prop.as_str().to_string());
                                    },
                                    Rule::assert => {
                                        match file_struct.unit.assert {
                                            None => file_struct.unit.assert = Some(vec![prop.as_str().to_string()]),
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.unit.assert = Some(x);
                                            }
                                        }
                                    }
                                    Rule::condition_path_exists => {
                                        file_struct.unit.condition_path_exists = Some(prop.as_str().to_string())
                                    },
                                    Rule::comment => {
                                        match file_struct.unit.comments {
                                            None => file_struct.unit.comments = Some(vec![prop.as_str().to_string()]),
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.unit.comments = Some(x);
                                            }
                                        }
                                    },

                                    _ => {}
                                }
                            }
                        }

                        _ => {println!("other: {item:?}")}
                    }
                }
            }
            Rule::service => {
                for item in line.into_inner() {
                    match item.as_rule() {
                        Rule::service_tag => {
                            file_struct.service.head = item.as_span().as_str().to_string()
                        },
                        Rule::service_prop => {
                            for prop in item.into_inner() {
                                match prop.as_rule() {
                                    Rule::service_type => {
                                        file_struct.service.service_type = prop.as_span().as_str().to_string()
                                    },
                                    Rule::exec_start => {
                                        file_struct.service.exec_start = prop.as_span().as_str()
                                            .to_string()
                                            .split_whitespace()
                                            .map(|v| v.to_string())
                                            .collect();
                                    },
                                    Rule::start_pre => {
                                        file_struct.service.start_pre = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::start_post => {
                                        file_struct.service.start_post = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::reload => {
                                        // file_struct.service.reload = Some(prop.as_span().as_str().to_string())
                                        match file_struct.service.reload {
                                            None => file_struct.service.reload = Some(vec![prop.as_span().as_str().to_string()]),
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.service.reload = Some(x);
                                            }
                                        }
                                    },
                                    Rule::exec_stop => {
                                        file_struct.service.exec_stop = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::stop_post => {
                                        file_struct.service.stop_post = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::restart_sec => {
                                        file_struct.service.restart_sec = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::restart => {
                                        file_struct.service.restart = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::timeout_sec => {
                                        file_struct.service.timeout_sec = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::environment_file => {
                                        file_struct.service.environment_file = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::kill_mode => {
                                        file_struct.service.kill_mode = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::restart_prevent_exit_status => {
                                        file_struct.service.restart_prevent_exit_status = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::runtime_directory => {
                                        file_struct.service.runtime_directory = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::runtime_directory_mode => {
                                        file_struct.service.runtime_directory_mode = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::environment => {
                                        file_struct.service.environment = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::standard_output => {
                                        file_struct.service.standard_output = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::limit_no_file => {
                                        file_struct.service.limit_no_file = Some(prop.as_span().as_str().to_string())
                                    },
                                    Rule::comment => {
                                        match file_struct.service.comments {
                                            None => file_struct.service.comments = Some(vec![prop.as_str().to_string()]),
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.service.comments = Some(x);
                                            }
                                        }
                                    },
                                    // Rule::unknown_symbols => {
                                    //     println!("********** UNKNOWN SYMBOL FOUND *****************");
                                    //     println!("{}", item.as_span().as_str());
                                    // }
                                    _ => {}
                                }
                            }
                        }
                        _ => {println!("other: {item:?}")}
                    }
                }
            }
            Rule::install => {
                // file_struct.install.head = line.as_span().as_str().to_string();
                for item in line.into_inner() {
                    match item.as_rule() {
                        Rule::install_tag => {
                            file_struct.install.head = item.as_span().as_str().to_string()
                        }
                        Rule::install_prop => {
                            for prop in item.into_inner() {
                                match prop.as_rule() {
                                    Rule::wanted_by => {
                                        file_struct.install.wanted_by = Some(prop.as_span().as_str().to_string())
                                    }
                                    Rule::required_by => {
                                        file_struct.install.required_by = Some(prop.as_span().as_str().to_string())
                                    }
                                    Rule::alias => {
                                        file_struct.install.alias = Some(prop.as_span().as_str().to_string())
                                    }
                                    Rule::also => {
                                        file_struct.install.also = Some(prop.as_span().as_str().to_string())
                                    }
                                    Rule::default_instance => {
                                        file_struct.install.default_instance = Some(prop.as_span().as_str().to_string())
                                    }
                                    Rule::comment => {
                                        match file_struct.install.comments {
                                            None => file_struct.install.comments = Some(vec![prop.as_str().to_string()]),
                                            Some(mut x) => {
                                                x.push(prop.as_str().to_string());
                                                file_struct.install.comments = Some(x);
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                        _ => {println!("other: {item:?}")}
                    }
                }
            }

            _ => {}
        }
    }

    file_struct
}
