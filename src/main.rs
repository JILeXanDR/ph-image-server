#![deny(elided_lifetimes_in_paths)]

extern crate hyper;

use std::borrow::Borrow;
use std::convert::Infallible;
use std::net::SocketAddr;

use clap::Parser;
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use prometheus::gather;
use prometheus::{Counter, Encoder, Opts, Registry, TextEncoder};
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

    let addr: SocketAddr = config
        .listen
        .parse()
        .expect("Unable to parse socket address");

    let make_svc = make_service_fn(|_socket: &AddrStream| {
        async {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| async {
                println!("Match handler for {:} {:}", req.method(), req.uri().path());

                // FIXME: it was the most easiest way to fix lifetime errors to move initialization here.
                //  It makes it very slow about 150 ms per request.
                let ua_parser = UserAgentParser::from_path("etc/regexes.yaml").unwrap();

                // Match incoming request to one of existing handlers.
                let response: Response<Body> = match (req.method(), req.uri().path()) {
                    (&Method::GET, "/img.php") => {
                        // if config.metrics.enabled {
                        metrics::CDN_REQUESTS_COUNTER.inc();
                        // }
                        handlers::get_image(req, ua_parser)
                    }
                    (&Method::GET, "/healthz") => handlers::check_health(req),
                    (&Method::GET, "/ping") => handlers::ping(req),
                    // TODO: it must be another server on config.metrics.addr.
                    (&Method::GET, "/metrics") => Response::builder()
                        .body(Body::from(metrics::to_buffer()))
                        .unwrap(),
                    _ => handlers::not_found(req),
                };

                Ok::<_, Infallible>(response)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("Listening TCP connections on http://{}", addr);

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
