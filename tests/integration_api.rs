use reqwest::header::USER_AGENT;
use reqwest::StatusCode;
use serde_json::json;
use tokio::spawn;

use ph_image_server::config::{Config, Metrics};

// TODO: how to do it static instead?
//  move occurs because `DEFAULT_CONFIG` has type `ph_image_server::config::Config`, which does not implement the `Copy` trait
fn get_default_config() -> Config {
    Config {
        listen: "127.0.0.1:44122".to_string(),
        report_to_v2: "http://some-host:9999/report-stats-v2?token=token".to_string(),
        metrics: Metrics {
            enabled: false,
            addr: "".to_string(),
        },
    }
}

#[actix_web::test]
async fn call_ping_endpoint() {
    spawn(async { ph_image_server::run(get_default_config()).await.unwrap() });

    let body = reqwest::get("http://localhost:44122/ping")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!(body, "pong");
}

#[actix_web::test]
async fn call_healthz_endpoint() {
    spawn(async { ph_image_server::run(get_default_config()).await.unwrap() });

    let body = reqwest::get("http://localhost:44122/healthz")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    assert_eq!(body, "");
}

#[actix_web::test]
async fn call_imgphp_endpoint_without_params() {
    spawn(async { ph_image_server::run(get_default_config()).await.unwrap() });

    let response = reqwest::get("http://localhost:44122/img.php")
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(response.text().await.unwrap(), "");
}

#[actix_web::test]
async fn call_imgphp_endpoint_with_all_params() {
    spawn(async { ph_image_server::run(get_default_config()).await.unwrap() });

    let client = reqwest::Client::new();

    let response= client.get("http://localhost:44122/img.php?v=2&id=eyJpY29uIjoiaWNvbi5wbmciLCJ1aWQiOjExMSwiY2lkIjoyMjIsIm9zIjoxMjMsImJyb3dzZXIiOjEyLCJjb3VudHJ5IjoyMTMsIm9wZXJhdG9yIjoxMjMsInN1YkFjYyI6MjMsInN1YklkIjoyMjIyMjIsImFkdlR5cGUiOjAsInRyYWZmaWNDaGFubmVsIjoyfQ==")
        .header(USER_AGENT, "alexandr")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("X-Accel-Redirect")
            .unwrap()
            .to_str()
            .unwrap(),
        "/icon/111/icon.png"
    );
    assert_eq!(response.text().await.unwrap(), "");
}
