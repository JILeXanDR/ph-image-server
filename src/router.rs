use hyper::{Body, Method, Request, Response, StatusCode};

pub type HandlerFn = fn(Request<Body>) -> Response<Body>;

// pub async fn prepare_matcher(
//     handlers: Handlers,
// ) -> Fn(Request<Body>) -> Result<Response<Body, Infallible>> {
// }

// pub async fn get_matcher(req: Request<Body>) -> Result<Response<Body>, Infallible> {
//     println!("Match handler for {:} {:}", req.method(), req.uri().path());
//
//     // Match incoming request to one of existing handlers.
//     let handler: Option<HandlerFn> = match (req.method(), req.uri().path()) {
//         (&Method::GET, "/img.php") => Some(get_image),
//         (&Method::GET, "/healthz") => Some(check_health),
//         (&Method::GET, "/ping") => Some(ping),
//         _ => None,
//     };
//
//     let response = match handler {
//         Some(handle) => handle(req),
//         None => not_found(req),
//     };
//
//     Ok(response)
// }
