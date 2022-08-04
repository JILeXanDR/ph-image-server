#![deny(elided_lifetimes_in_paths)]

use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};

use actix_web::{App, HttpResponse, HttpServer};
use clap::Parser;
use tokio::spawn;
use user_agent_parser::UserAgentParser;

use crate::{
    handlers::{check_health, get_image, ping},
    metrics::get_metrics,
};

mod config;
mod handlers;
mod metrics;
mod models;
mod stats;

/// Image CDN server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[clap(short, long, value_parser, default_value = "config/config.yaml")]
    config: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Loading app with config file {}", args.config);

    let config = config::load(args.config).expect("Failed to load config");

    println!("Config is ready {:#}", config);

    let ua_parser = UserAgentParser::from_path("etc/regexes.yaml")
        .expect("Failed to load YAML file with regexes");

    // If prometheus metrics enabled it hosts them on http://127.0.0.1:9010/metrics.
    if config.metrics.enabled {
        spawn(async move {
            let addr: SocketAddr = config.metrics.addr.parse().unwrap();

            let server = HttpServer::new(|| App::new().service(get_metrics))
                .bind(addr)
                .unwrap()
                .run();

            println!("Starting serving prometheus metrics on http://{:}", addr);

            if let Err(e) = server.await {
                eprintln!("Metrics server error: {}", e);
            }
        });
    }

    let addr: SocketAddr = config
        .listen
        .parse()
        .expect("Unable to parse socket address");

    println!("Starting CDN server on http://{:}", addr);

    let ua_parser_data = actix_web::web::Data::new(ua_parser);

    HttpServer::new(move || {
        App::new()
            .app_data(ua_parser_data.clone())
            .service(get_image)
            .service(check_health)
            .service(ping)
            .default_service(actix_web::web::to(|| HttpResponse::NotFound()))
    })
    .shutdown_timeout(Duration::from_secs(10).as_secs())
    .bind(addr)?
    .run()
    .await
}
