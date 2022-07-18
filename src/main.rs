#![deny(elided_lifetimes_in_paths)]

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

use clap::Parser;
use hyper::header::HeaderValue;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{Method, StatusCode};

use models::icon::IconRequest;

mod config;
mod models;

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
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    let server = Server::bind(&addr).serve(make_service);

    // And now add a graceful shutdown signal...
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    println!("Open HTTP server on http://{}", addr);

    // Run this server for... forever!
    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/img.php") => {
            let params: HashMap<String, String> = req
                .uri()
                .query()
                .map(|v| {
                    url::form_urlencoded::parse(v.as_bytes())
                        .into_owned()
                        .collect()
                })
                .unwrap_or_else(HashMap::new);

            // TODO: rewrite this in a shorter way 0_0.
            if let Some(v) = params.get("v") {
                if v == "2" {
                    if let Some(id) = params.get("id") {
                        if id == "" {
                            *response.status_mut() = StatusCode::BAD_REQUEST;
                        } else {
                            match IconRequest::from_base64(id) {
                                Ok(ir) => {
                                    // base64
                                    response.headers_mut().insert(
                                        "Location",
                                        HeaderValue::from_str(&ir.icon).unwrap(), // TODO: don't panic here!
                                    );
                                    *response.status_mut() = StatusCode::FOUND;
                                }
                                Err(_) => {
                                    // TODO: don't panic.
                                    panic!("failed to...");
                                }
                            };
                        }
                    } else {
                        *response.status_mut() = StatusCode::BAD_REQUEST;
                    }
                } else {
                    response
                        .headers_mut()
                        .insert("Location", HeaderValue::from_static("https://google.com"));
                    *response.status_mut() = StatusCode::FOUND;
                }
            } else {
                response
                    .headers_mut()
                    .insert("Location", HeaderValue::from_static("https://google.com"));
                *response.status_mut() = StatusCode::FOUND;
            }
        }
        (&Method::GET, "/healthz") => {
            *response.status_mut() = StatusCode::OK;
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    };

    Ok(response)
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
