#![deny(elided_lifetimes_in_paths)]

extern crate hyper;

use std::convert::Infallible;
use std::net::SocketAddr;

use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;

mod config;
mod handlers;
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

    let addr: SocketAddr = config
        .listen
        .parse()
        .expect("Unable to parse socket address");

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_service = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(router::get_matcher))
    });

    let server = Server::bind(&addr).serve(make_service);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("Listening TCP connections on http://{}", addr);

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_readme() {}
}
