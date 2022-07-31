use hyper::header::{CONTENT_LENGTH, CONTENT_TYPE};
use hyper::{Body, Request, Response, StatusCode};

pub fn get_image(_: Request<Body>) -> Response<Body> {
    let mut response: Response<Body> = Response::new(Body::default());

    let mut body = response.body_mut();

    response

    // let params: HashMap<String, String> = req
    //     .uri()
    //     .query()
    //     .map(|v| {
    //         url::form_urlencoded::parse(v.as_bytes())
    //             .into_owned()
    //             .collect()
    //     })
    //     .unwrap_or_else(HashMap::new);
    //
    // // TODO: rewrite this in a shorter way 0_0.
    // if let Some(v) = params.get("v") {
    //     if v == "2" {
    //         if let Some(id) = params.get("id") {
    //             if id == "" {
    //                 *response.status_mut() = StatusCode::BAD_REQUEST;
    //             } else {
    //                 match IconRequest::from_base64(id) {
    //                     Ok(icon) => {
    //                         let useragent = match req.headers().get("User-Agent") {
    //                             Some(v) => v,
    //                             None => panic!("no User-Agent header"),
    //                         };
    //
    //                         // TODO: go uses uasurfer lib.
    //                         let ua = match useragent::parse(useragent.to_str().unwrap()) {
    //                             Some(ua) => ua,
    //                             None => panic!("can't parse useragent"),
    //                         };
    //
    //                         // TODO:
    //                         let device = get_device_from_parsed_user_agent(&ua);
    //                         let os = get_os_from_parsed_user_agent(&ua);
    //                         let browser = get_browser_from_parsed_user_agent(&ua);
    //
    //                         let url = format!("/icon/{}/{}", icon.web_user_id, icon.icon);
    //
    //                         response.headers_mut().insert(
    //                             "X-Accel-Redirect",
    //                             HeaderValue::from_str(&url).unwrap(),
    //                         );
    //
    //                         stats::increment(stats::ShowStatistic {
    //                             uid: 0,
    //                             cid: 0,
    //                             os: 0,
    //                             browser: 0,
    //                             country: 0,
    //                             sub_acc: 0,
    //                             operator: 0,
    //                             adv_type: 0,
    //                             device: 0,
    //                         });
    //                     }
    //                     Err(_) => {
    //                         // TODO: don't panic.
    //                         panic!("failed to...");
    //                     }
    //                 };
    //             }
    //         } else {
    //             *response.status_mut() = StatusCode::BAD_REQUEST;
    //         }
    //     } else {
    //         response
    //             .headers_mut()
    //             .insert("Location", HeaderValue::from_static("https://google.com"));
    //         *response.status_mut() = StatusCode::FOUND;
    //     }
    // } else {
    //     response
    //         .headers_mut()
    //         .insert("Location", HeaderValue::from_static("https://google.com"));
    //     *response.status_mut() = StatusCode::FOUND;
    //
    //     response
    // }
}

pub fn check_health(_: Request<Body>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}

pub fn ping(_: Request<Body>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("pong"))
        .unwrap()
}
