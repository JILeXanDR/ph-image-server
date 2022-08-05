#![deny(elided_lifetimes_in_paths)]

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

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

    pretty_env_logger::init_timed();

    debug!("Loading app with config file {}", args.config);

    let config = config::from_yaml_file_path(args.config).expect("Failed to get config");

    debug!("Config is ready {:#}", config);
    debug!("App is starting...");

    match run(config).await {
        Ok(_) => info!("App stopped"),
        Err(err) => error!("App error {:?}", err),
    };

    Ok(())
}
