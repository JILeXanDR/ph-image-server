use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use hyper::header::HeaderValue;
use hyper::{Body, Request, Response, StatusCode};
use user_agent_parser::UserAgentParser;

use crate::models::icon::IconRequest;
use crate::models::useragent::{
    get_browser_from_parsed_user_agent, get_device_from_parsed_user_agent,
    get_os_from_parsed_user_agent,
};
use crate::stats;

// http://127.0.0.1:9123/img.php?v=2&id=eyJpY29uIjoiaWNvbi5wbmciLCJ1aWQiOjExMSwiY2lkIjoyMjIsIm9zIjoxMjMsImJyb3dzZXIiOjEyLCJjb3VudHJ5IjoyMTMsIm9wZXJhdG9yIjoxMjMsInN1YkFjYyI6MjMsInN1YklkIjoyMjIyMjIsImFkdlR5cGUiOiJQdXNoIiwidHJhZmZpY0NoYW5uZWwiOiJGZWVkIn0=
pub fn get_image(
    req: Request<Body>,
    ua_parser: Arc<UserAgentParser>,
) -> Result<Response<Body>, Box<dyn Error>> {
    let mut response: Response<Body> = Response::new(Body::default());

    let params: HashMap<String, String> = req
        .uri()
        .query()
        .map(|v| {
            url::form_urlencoded::parse(v.as_bytes())
                .into_owned()
                .collect()
        })
        .unwrap_or_else(HashMap::new);

    let version = params.get("v");
    if version.is_none() {
        println!("version is missing");
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    }

    let version = version.unwrap();

    if version != "2" {
        println!("version `{:}` not supported", version);
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    }

    let id = params.get("id");
    if id.is_none() {
        println!("id is missing");
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    }

    let id = id.unwrap();
    if id == "" {
        println!("id is empty");
        *response.status_mut() = StatusCode::BAD_REQUEST;
        return Ok(response);
    }

    println!("found id `{:}`", id);

    match IconRequest::from_base64(id) {
        Ok(icon) => {
            println!("decoded icon {:?}", icon);

            let useragent = match req.headers().get("User-Agent") {
                Some(v) => v,
                None => panic!("no User-Agent header"), // TODO: don't panic!
            };

            let useragent = useragent.to_str().unwrap();

            // TODO: go uses uasurfer lib.
            let os = ua_parser.parse_os(useragent);
            let device = ua_parser.parse_device(useragent);
            let browser = ua_parser.parse_product(useragent);

            let url = format!("/icon/{}/{}", icon.web_user_id, icon.icon);

            response
                .headers_mut()
                .insert("X-Accel-Redirect", HeaderValue::from_str(&url).unwrap());

            stats::increment(stats::ShowStatistic {
                uid: icon.web_user_id,
                cid: icon.campaign_id,
                os: get_os_from_parsed_user_agent(os),
                browser: get_browser_from_parsed_user_agent(browser),
                country: icon.country_id,
                sub_acc: icon.sub_acc_id,
                operator: icon.operator,
                adv_type: icon.advertisement_type,
                device: get_device_from_parsed_user_agent(device),
            });
        }
        Err(err) => {
            println!("failed to decode id {:?}", err);
            *response.status_mut() = StatusCode::BAD_REQUEST;
        }
    };

    Ok(response)
}

// http://127.0.0.1:9123/healthz
pub fn check_health(_: Request<Body>) -> Result<Response<Body>, Box<dyn Error>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())?)
}

// http://127.0.0.1:9123/ping
pub fn ping(_: Request<Body>) -> Result<Response<Body>, Box<dyn Error>> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("pong"))?)
}

pub fn not_found(req: Request<Body>) -> Result<Response<Body>, Box<dyn Error>> {
    let body = format!("can't handle {:} {:}", req.method(), req.uri().path());
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(body))?)
}

fn unwrap_hyper_http_err_log_and_respond_500(err: hyper::http::Error) -> Response<Body> {
    println!("Error while writing response {:?}", err);
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("server error"))
        .unwrap()
}
