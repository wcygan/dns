use clap::builder::{PossibleValuesParser, TypedValueParser};
use clap::Parser;

use dns_common::QueryType;

#[derive(Parser)]
pub struct Args {
    /// The DNS server to query
    #[clap(short = 's', long = "server", default_value = "8.8.8.8")]
    pub server: String,
    /// The port to query
    #[clap(short = 'p', long = "port", default_value = "53")]
    pub port: u16,
    /// The name to find the IP address for
    #[clap(short = 'n', long = "name", default_value = "google.com")]
    pub name: String,
    #[clap(
    short = 't',
    long = "type",
    value_parser = PossibleValuesParser::new(["A"]).map(| s | s.parse::< QueryType > ().unwrap()),
    default_value = "A"
    )]
    pub qtype: QueryType,
}
