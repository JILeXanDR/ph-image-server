#![deny(elided_lifetimes_in_paths)]

use std::error::Error;

use clap::Parser;

use ph_image_server::{config, run};

/// Image CDN server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[clap(short, long, value_parser, default_value = "etc/config.yaml")]
    config: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Loading app with config file {}", args.config);

    let config = config::from_yaml_file_path(args.config).expect("Failed to get config");

    println!("Config is ready {:#}", config);
    println!("App is starting...");

    match run(config).await {
        Ok(_) => {
            println!("App stopped");
            Ok(())
        }
        Err(err) => {
            eprintln!("App error {:?}", err);
            Ok(())
        }
    }
}
