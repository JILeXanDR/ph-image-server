use reqwest::StatusCode;
use tokio::spawn;

use ph_image_server::config::{Config, Metrics};

#[actix_web::test]
async fn call_ping_endpoint() {
    spawn(async {
        ph_image_server::run(Config {
            listen: "127.0.0.1:44122".to_string(),
            report_to_v2: "".to_string(),
            metrics: Metrics {
                enabled: false,
                addr: "".to_string(),
            },
        })
        .await
        .unwrap()
    });

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
    spawn(async {
        ph_image_server::run(Config {
            listen: "127.0.0.1:44122".to_string(),
            report_to_v2: "".to_string(),
            metrics: Metrics {
                enabled: false,
                addr: "".to_string(),
            },
        })
        .await
        .unwrap()
    });

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
    spawn(async {
        ph_image_server::run(Config {
            listen: "127.0.0.1:44122".to_string(),
            report_to_v2: "".to_string(),
            metrics: Metrics {
                enabled: false,
                addr: "".to_string(),
            },
        })
        .await
        .unwrap()
    });

    let response = reqwest::get("http://localhost:44122/img.php")
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(response.text().await.unwrap(), "");
}
