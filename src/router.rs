use std::convert::Infallible;

use hyper::{Body, Method, Request, Response, StatusCode};

use crate::handlers::{check_health, get_image, ping};

pub type HandlerFn = fn(Request<Body>) -> Response<Body>;

pub async fn get_matcher(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    println!("Match handler for {:} {:}", req.method(), req.uri().path());

    // Match incoming request to one of existing handlers.
    let handler: Option<HandlerFn> = match (req.method(), req.uri().path()) {
        (&Method::GET, "/img.php") => Some(get_image),
        (&Method::GET, "/healthz") => Some(check_health),
        (&Method::GET, "/ping") => Some(ping),
        _ => None,
    };

    let response = match handler {
        Some(handle) => handle(req),
        None => not_found(req),
    };

    Ok(response)
}

fn not_found(req: Request<Body>) -> Response<Body> {
    let body = format!("can't handle {:} {:}", req.method(), req.uri().path());
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(body))
        .unwrap()
}
