use clap::Parser;
use std::net::IpAddr;

#[derive(Parser, Debug)]
#[command(author = "Blockprint Collective", version, long_about = None)]
#[command(about = "Measure the accuracy of a blockprint instance using synthetic blocks")]
pub struct Config {
    /// Lighthouse node to fetch block reward data from.
    #[arg(long, value_name = "URL")]
    pub lighthouse_url: String,

    /// Blockprint instance to use for classifying blocks.
    #[arg(long, value_name = "URL")]
    pub blockprint_url: String,

    /// Address to listen on.
    #[arg(long, value_name = "IP", default_value = "127.0.0.1")]
    pub listen_address: Vec<IpAddr>,

    /// Port to listen on.
    #[arg(long, value_name = "N", default_value = "8002")]
    pub port: u16,
}
