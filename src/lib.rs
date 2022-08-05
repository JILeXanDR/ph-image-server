use std::{error::Error, net::SocketAddr, time::Duration};

use actix_web::{App, HttpResponse, HttpServer};
use log::{debug, error, info};
use tokio::spawn;
use user_agent_parser::UserAgentParser;

use crate::{
    config::Config,
    handlers::{check_health, get_image, ping},
    metrics::get_metrics,
};

pub mod config;
mod handlers;
mod metrics;
mod models;
mod stats;

/// Runs app with behavior based on input config.
pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let ua_parser = UserAgentParser::from_path("etc/regexes.yaml")
        .or_else(|err| Err(format!("Failed to load YAML file with regexes: {:?}", err)))?;

    // If prometheus metrics enabled it hosts them on http://127.0.0.1:9010/metrics.
    if config.metrics.enabled {
        spawn(async move {
            let addr: SocketAddr = config.metrics.addr.parse().unwrap();

            let server = HttpServer::new(|| App::new().service(get_metrics))
                .bind(addr)
                .expect("Binding failed")
                .run();

            info!("Starting serving prometheus metrics on http://{:}", addr);

            if let Err(e) = server.await {
                error!("Metrics server error: {}", e);
            }
        });
    }

    let addr: SocketAddr = config
        .listen
        .parse()
        .expect("Unable to parse socket address");

    debug!("Starting CDN server on http://{:}", addr);

    let ua_parser_data = actix_web::web::Data::new(ua_parser);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(ua_parser_data.clone())
            .service(get_image)
            .service(check_health)
            .service(ping)
            .default_service(actix_web::web::to(|| HttpResponse::NotFound()))
    })
    .shutdown_timeout(Duration::from_secs(10).as_secs())
    .bind(addr)?
    .run();

    match server.await {
        Ok(_) => Ok(()),
        Err(err) => Err(format!("Server error {:?}", err).into()),
    }
}
