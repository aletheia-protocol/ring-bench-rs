use prometheus::{Gauge, Encoder, TextEncoder, register_gauge};
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::config::CONFIG;

// Create a globally shared gauge using lazy_static
lazy_static! {
    static ref PRICE_GAUGE: Mutex<Gauge> = Mutex::new(register_gauge!(
        "crypto_price",
        CONFIG.default.trading_pair.as_str()
    ).unwrap());
}

pub fn track_price(price: f64) {
    // Update the shared gauge
    let mut gauge = PRICE_GAUGE.lock().unwrap();
    gauge.set(price);
}

pub fn serve_metrics() -> String {
    // Serve the Prometheus metrics via HTTP
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    let metrics = prometheus::gather();
    encoder.encode(&metrics, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}