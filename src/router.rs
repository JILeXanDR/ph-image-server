use hyper::{Body, Method, Request, Response, StatusCode};
use tokio::time::Instant;
use user_agent_parser::UserAgentParser;

use crate::{handlers, metrics};

pub fn match_request_to_handler(req: Request<Body>) -> Response<Body> {
    let started = Instant::now();

    // FIXME: it must in main.rs.
    let ua_parser =
        UserAgentParser::from_path("etc/regexes.yaml").expect("Loading YAML file with regexes");

    let result = match (req.method(), req.uri().path()) {
        (&Method::GET, "/img.php") => {
            // FIXME:    | |_____________^ returns an `async` block that contains a reference to a captured variable, which then escapes the closure body
            // if config.metrics.enabled {
            metrics::CDN_REQUESTS_COUNTER.inc();
            // }
            handlers::get_image(req, &ua_parser)
        }
        (&Method::GET, "/healthz") => handlers::check_health(req),
        (&Method::GET, "/ping") => handlers::ping(req),
        _ => handlers::not_found(req),
    };

    let response = match result {
        Ok(response) => response,
        Err(err) => {
            println!("Error while executing handler {:?}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("internal server error"))
                .expect("Failed to write HTTP response")
        }
    };

    println!(
        "Handler processing done in {:?}",
        Instant::now().duration_since(started)
    );

    response
}
