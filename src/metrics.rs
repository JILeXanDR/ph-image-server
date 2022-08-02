use std::convert::Infallible;
use std::error::Error;
use std::net::SocketAddr;

use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server};
use lazy_static::lazy_static;
use prometheus::{self, Encoder, IntCounter, TextEncoder};
use prometheus::{gather, labels, opts, register_int_counter};

use crate::handlers;

lazy_static! {
    pub static ref CDN_REQUESTS_COUNTER: IntCounter = register_int_counter!(opts!(
        "pushhouse_cdn_requests",
        "Number of received CDN requests",
        // labels! {}
    ),)
    .unwrap();
}

fn to_buffer() -> Vec<u8> {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = gather();

    encoder.encode(&metric_families, &mut buffer).unwrap();

    buffer
}

pub async fn serve_metrics(addr: String) -> Result<hyper::Result<()>, Box<dyn Error>> {
    let addr: SocketAddr = addr.parse().expect("Unable to parse socket address");

    let make_svc = make_service_fn(|_socket: &AddrStream| async {
        Ok::<_, Infallible>(service_fn(move |req: Request<Body>| async {
            let response: Response<Body> = match (req.method(), req.uri().path()) {
                (&Method::GET, "/metrics") => {
                    Response::builder().body(Body::from(to_buffer())).unwrap()
                }
                _ => handlers::not_found(req),
            };
            Ok::<_, Infallible>(response)
        }))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening metrics TCP connections on http://{}", addr);

    Ok(server.await)
}
