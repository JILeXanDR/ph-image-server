use lazy_static::lazy_static;
use prometheus::{self, Encoder, IntCounter, TextEncoder};
use prometheus::{gather, labels, opts, register_int_counter};

lazy_static! {
    pub static ref CDN_REQUESTS_COUNTER: IntCounter = register_int_counter!(opts!(
        "pushhouse_cdn_requests",
        "Number of received CDN requests",
        // labels! {}
    ),)
    .unwrap();
}

pub fn to_buffer() -> Vec<u8> {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = gather();

    encoder.encode(&metric_families, &mut buffer).unwrap();

    buffer
}
