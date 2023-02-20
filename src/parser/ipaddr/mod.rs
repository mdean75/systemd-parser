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
    pub inet: Vec<String>,
    pub inet6: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_name: Option<String>
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
                            iface.mtu = interface.as_span().as_str().to_string();
                        }
                        Rule::qdisc => {
                            iface.qdisc = interface.as_span().as_str().to_string();
                        }
                        Rule::master => {
                            iface.master = Some(interface.as_span().as_str().to_string());
                        }
                        Rule::state => {
                            iface.state = interface.as_span().as_str().to_string();
                        }
                        Rule::group => {
                            iface.group = interface.as_span().as_str().to_string();
                        }
                        Rule::qlen => {
                            iface.qlen = Some(interface.as_span().as_str().to_string());
                        }
                        Rule::link => {
                            iface.link = interface.as_span().as_str().to_string();
                        }
                        Rule::inet => {
                            iface.inet.push(interface.as_span().as_str().to_string());
                        }
                        Rule::inet6 => {
                            iface.inet6.push(interface.as_span().as_str().to_string());
                        }
                        Rule::alt_name => {
                            iface.alt_name = Some(interface.as_span().as_str().to_string());
                        }

                        _ => {}
                    }
                }

                // println!("{:?}", iface);
                interfaces.insert(iface.if_name.clone(), iface);
            }

            _ => {}
        }
        // println!("{}", line.as_span().as_str());
    }

    // println!("{:?}", interfaces);
    for i in interfaces {
        println!("{}", serde_json::to_string_pretty(&i).unwrap())
    }
    return Ok(())
}
