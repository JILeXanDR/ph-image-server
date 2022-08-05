use actix_web::{get, http::StatusCode, web, HttpRequest, HttpResponseBuilder, Responder};
use log::{debug, error, info};
use serde::Deserialize;
use user_agent_parser::UserAgentParser;

use crate::{
    metrics,
    models::icon::IconRequest,
    models::useragent::{
        get_browser_from_parsed_user_agent, get_device_from_parsed_user_agent,
        get_os_from_parsed_user_agent,
    },
    stats,
};

#[derive(Debug, Deserialize)]
pub struct GetImageRequest {
    #[serde(rename(deserialize = "v"))]
    version: Option<String>,
    #[serde(rename(deserialize = "id"))]
    payload: Option<String>,
}

// http://127.0.0.1:9123/img.php?v=2&id=eyJpY29uIjoiaWNvbi5wbmciLCJ1aWQiOjExMSwiY2lkIjoyMjIsIm9zIjoxMjMsImJyb3dzZXIiOjEyLCJjb3VudHJ5IjoyMTMsIm9wZXJhdG9yIjoxMjMsInN1YkFjYyI6MjMsInN1YklkIjoyMjIyMjIsImFkdlR5cGUiOjAsInRyYWZmaWNDaGFubmVsIjoyfQ==
#[get("/img.php")]
pub async fn get_image(
    req: HttpRequest,
    params: web::Query<GetImageRequest>,
    // TODO: if miss this, in "app_data" call or set wrong struct we will get "Requested application data is not configured correctly. View/enable debug logs for more details." in runtime.
    ua_parser: web::Data<UserAgentParser>,
) -> impl Responder {
    metrics::CDN_REQUESTS_COUNTER.inc();

    debug!(target: "get_image", "called");

    if params.version.is_none() {
        info!("version is missing");
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    }

    let version = params.version.as_ref().unwrap();

    if version != "2" {
        info!("version `{:}` not supported", version);
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    }

    if params.payload.is_none() {
        info!("id is missing");
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    }

    let id = params.payload.as_ref().unwrap();
    if id == "" {
        info!("id is empty");
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    }

    info!("found id `{:}`", id);

    let icon = IconRequest::from_base64(&id);

    if let Err(err) = icon {
        error!("failed to decode id {:?}", err);
        return HttpResponseBuilder::new(StatusCode::BAD_REQUEST).finish();
    }

    let icon = icon.unwrap();

    info!("decoded icon {:?}", icon);

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

    let icon_json = serde_json::to_string(&icon).expect("Can't encode to JSON");

    stats::increment(stats::ShowStatistic {
        uid: icon.web_user_id.clone(),
        cid: icon.campaign_id.clone(),
        os: get_os_from_parsed_user_agent(os),
        browser: get_browser_from_parsed_user_agent(browser),
        country: icon.country_id.clone(),
        sub_acc: icon.sub_acc_id.clone(),
        operator: icon.operator.clone(),
        adv_type: icon.advertisement_type.clone(),
        device: get_device_from_parsed_user_agent(device),
    });

    HttpResponseBuilder::new(StatusCode::OK)
        .insert_header(("X-Accel-Redirect", url))
        .body(icon_json)
}

// http://127.0.0.1:9123/healthz
#[get("/healthz")]
pub async fn check_health() -> impl Responder {
    HttpResponseBuilder::new(StatusCode::OK)
}

// http://127.0.0.1:9123/ping
#[get("/ping")]
pub async fn ping() -> String {
    "pong".to_owned()
}
