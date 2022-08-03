#![deny(elided_lifetimes_in_paths)]

extern crate hyper;

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use clap::Parser;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Response};
use hyper::header::USER_AGENT;
use hyper::Request;
use hyper::Server;
use tokio::spawn;
use user_agent_parser::UserAgentParser;

mod config;
mod handlers;
mod metrics;
mod models;
mod router;
mod stats;

/// Image CDN server
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Path to config file
    #[clap(short, long, value_parser, default_value = "config/config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    println!("Loading app with config file {}", args.config);

    let config = config::load(args.config).expect("Failed to load config");

    let ua_parser = Arc::new(
        UserAgentParser::from_path("etc/regexes.yaml").expect("Loading YAML file with regexes"));

    // If prometheus metrics enabled it hosts them on http://127.0.0.1:9010/metrics.
    if config.metrics.enabled {
        spawn(async {
            if let Err(e) = metrics::serve(config.metrics.addr).await {
                eprintln!("metrics server error: {}", e);
            }
        });
    }

    let server = serve(config.listen, ua_parser);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn serve(addr: String, ua_parser: Arc<UserAgentParser>) -> hyper::Result<()> {
    let addr: SocketAddr = addr.parse().expect("Unable to parse socket address");

    let make_svc = make_service_fn(move |_socket: &AddrStream| {
        let ua_parser = ua_parser.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let ua_parser = ua_parser.clone();
                async move {
                    println!("Requested {:} {:}", req.method(), req.uri().path());
                    let response = router::match_request_to_handler(req, ua_parser);
                    Ok::<_, Infallible>(response)
                }
            }))
        }
    });

    let server = Server::bind(&addr)
        .serve(make_svc)
        .with_graceful_shutdown(shutdown_signal());

    println!("Listening TCP connections on http://{}", addr);

    server.await
}
