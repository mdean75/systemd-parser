use std::collections::HashMap;
use pest::Parser;
use pest_derive::Parser;
use serde_derive::{Deserialize, Serialize};

#[derive(Parser)]
#[grammar = "src/parser/ipaddr/ipaddr_grammar.pest"]
pub struct IpaddrParser;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
pub struct Interface {
    pub index: String,
    pub if_name: String,
    pub flags: String,
    pub mtu: String,
    pub qdisc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<String>,
    pub state: String,
    pub group: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qlen: Option<String>,
    pub link: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inet: Vec<Inet>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inet6: Vec<Inet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_name: Option<String>
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all="camelCase")]
pub struct  Inet {
    pub ip: String,
    pub net_prefix: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broadcast: Option<String>,
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conf_flag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub lifetime: String
}

pub fn parse(out: &str) -> Result<(), String> {
    let ip_addr_out = IpaddrParser::parse(Rule::ip_a_out, out)
        .map_err(|e| e.to_string())?
        .next()
        // this is documented as never panics but i don't want to use expect or unwrap
        .ok_or("unable to convert Option to Result")?;

    let mut interfaces: HashMap<String, Interface> = HashMap::new();
    for line in ip_addr_out.into_inner() {
        match line.as_rule() {
            Rule::interface => {
                // println!("{}", line.to_string())
                let mut iface: Interface = Interface::default();
                for interface in line.into_inner() {
                    match interface.as_rule() {
                        Rule::index => {
                            // println!("{}", interface.as_span().as_str())
                            iface.index = interface.as_span().as_str().to_string();
                        },
                        Rule::if_name => {
                            // println!("{}", interface.as_span().as_str())
                            iface.if_name = interface.as_span().as_str().to_string();
                        },
                        Rule::flags => {
                            iface.flags = interface.as_span().as_str().to_string();
                        },
                        Rule::mtu => {
                            for mtu in interface.into_inner() {
                                match mtu.as_rule() {
                                    Rule::mtu_value => {
                                        iface.mtu = mtu.as_span().as_str().to_string();
                                    }

                                    _ => {}
                                }
                            }
                        }
                        Rule::qdisc => {
                            for qdisc_entry in interface.into_inner() {
                                match qdisc_entry.as_rule() {
                                    Rule::qdisc_value => {
                                        iface.qdisc = qdisc_entry.as_span().as_str().to_string();
                                    }

                                    _ => {}
                                }
                            }

                        }
                        Rule::master => {
                            for master_entry in interface.into_inner() {
                                match master_entry.as_rule() {
                                    Rule::master_value => {
                                        iface.master = Some(master_entry.as_span().as_str().to_string())
                                    }

                                    _ => {}
                                }
                            }
                        }
                        Rule::state => {
                            for state_entry in interface.into_inner() {
                                match state_entry.as_rule() {
                                    Rule::state_value => {
                                        iface.state = state_entry.as_span().as_str().to_string()
                                    }

                                    _ => {}
                                }
                            }
                        }
                        Rule::group => {
                            for group_entry in interface.into_inner() {
                                match group_entry.as_rule() {
                                    Rule::group_value => {
                                        iface.group = group_entry.as_span().as_str().to_string()
                                    }

                                    _ => {}
                                }
                            }
                        }
                        Rule::qlen => {
                            for qlen_entry in interface.into_inner() {
                                match qlen_entry.as_rule() {
                                    Rule::qlen_value => {
                                        iface.qlen = Some(qlen_entry.as_span().as_str().to_string())
                                    }

                                    _ => {}
                                }
                            }
                        }
                        Rule::link => {
                            iface.link = interface.as_span().as_str().to_string();
                        }
                        Rule::inet => {
                            let mut inet: Inet = Inet::default();
                            for inet_entry in interface.into_inner() {
                                match inet_entry.as_rule() {
                                    Rule::ip => {
                                        inet.ip = inet_entry.as_span().as_str().to_string()
                                    }
                                    Rule::net_prefix => {
                                        inet.net_prefix = inet_entry.as_span().as_str().to_string()
                                    }
                                    // need to capture ipv4 broadcast
                                    Rule::broadcast4 => {
                                        for brd in inet_entry.into_inner() {
                                            match brd.as_rule() {
                                                Rule::ip => {
                                                    inet.broadcast = Some(brd.as_span().as_str().to_string())
                                                }

                                                _ => {}
                                            }
                                        }
                                    }
                                    Rule::scope => {
                                        for scope in inet_entry.into_inner() {
                                            match scope.as_rule() {
                                                Rule::scope_value => {
                                                    inet.scope = scope.as_span().as_str().to_string()
                                                }

                                                _ => {}
                                            }
                                        }
                                    }
                                    Rule::conf_flag => {
                                        inet.conf_flag = Some(inet_entry.as_span().as_str().to_string())
                                    }
                                    Rule::label => {
                                        inet.label = Some(inet_entry.as_span().as_str().to_string())
                                    }
                                    Rule::lifetime => {
                                        inet.lifetime = inet_entry.as_span().as_str().to_string()
                                    }

                                    _ => {}
                                }
                            }
                            iface.inet.push(inet)
                        }
                        Rule::inet6 => {
                            let mut inet: Inet = Inet::default();
                            for inet_entry in interface.into_inner() {
                                match inet_entry.as_rule() {
                                    Rule::ip6 => {
                                        inet.ip = inet_entry.as_span().as_str().to_string()
                                    }
                                    Rule::net_prefix => {
                                        inet.net_prefix = inet_entry.as_span().as_str().to_string()
                                    }
                                    // need to capture ipv4 broadcast
                                    Rule::scope => {
                                        for scope in inet_entry.into_inner() {
                                            match scope.as_rule() {
                                                Rule::scope_value => {
                                                    inet.scope = scope.as_span().as_str().to_string()
                                                }

                                                _ => {}
                                            }
                                        }
                                    }
                                    Rule::conf_flag => {
                                        inet.conf_flag = Some(inet_entry.as_span().as_str().to_string())
                                    }
                                    Rule::label => {
                                        inet.label = Some(inet_entry.as_span().as_str().to_string())
                                    }
                                    Rule::lifetime => {
                                        inet.lifetime = inet_entry.as_span().as_str().to_string()
                                    }

                                    _ => {}
                                }
                            }
                            iface.inet6.push(inet)
                        }
                        Rule::alt_name => {
                            iface.alt_name = Some(interface.as_span().as_str().to_string());
                        }

                        _ => {}
                    }
                }

                interfaces.insert(iface.if_name.clone(), iface);
            }

            _ => {}
        }
    }

    for i in interfaces {
        println!("{}", serde_json::to_string_pretty(&i).unwrap())
    }
    return Ok(())
}
