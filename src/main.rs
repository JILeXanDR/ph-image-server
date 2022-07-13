use std::fmt::Error;

use clap::Parser;

use crate::config::Config;

mod config;

/// Image CDN server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[clap(short, long, value_parser, default_value = "config/config.yaml")]
    config: String,
}

fn main() {
    let args = Args::parse();

    println!("Loading app with config file {}", args.config);

    let config = config::load(args.config).expect("Failed to load config");

    print!("Listen to {}", config.listen);
}
