use actix_web::{get, http::StatusCode, HttpResponseBuilder, Responder};
use lazy_static::lazy_static;
use prometheus::{self, gather, opts, register_int_counter, Encoder, IntCounter, TextEncoder};

lazy_static! {
    pub static ref CDN_REQUESTS_COUNTER: IntCounter = register_int_counter!(opts!(
        "pushhouse_cdn_requests",
        "Number of received CDN requests",
        // labels! {}
    ),)
    .expect("metric can't be created");
}

#[get("/metrics")]
pub async fn get_metrics() -> impl Responder {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = gather();

    encoder.encode(&metric_families, &mut buffer).unwrap();

    HttpResponseBuilder::new(StatusCode::OK)
        .insert_header(("Content-Type", "text/plain"))
        .body(buffer)
}
