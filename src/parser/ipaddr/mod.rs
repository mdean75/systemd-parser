use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "src/parser/ipaddr/ipaddr_grammar.pest"]
pub struct IpaddrParser;

pub fn parse(out: &str) -> Result<(), String> {
    let ip_addr_out = IpaddrParser::parse(Rule::ip_a_out, out)
        .map_err(|e| e.to_string())?
        .next()
        // this is documented as never panics but i don't want to use expect or unwrap
        .ok_or("unable to convert Option to Result")?;

    for line in ip_addr_out.into_inner() {
        println!("{}", line.as_span().as_str());
    }
    return Ok(())
}
