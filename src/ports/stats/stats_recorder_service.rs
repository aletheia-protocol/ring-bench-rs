use std::collections::VecDeque;
use prometheus::{Gauge, Encoder, TextEncoder, register_gauge, default_registry, Registry};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::config::CONFIG;
use crate::domain::entities::trade::TradeData;

// Static singleton for global price gauge storage
pub static PRICE_GAUGE: Lazy<Arc<Mutex<Gauge>>> = Lazy::new(|| {
    let gauge = register_gauge!(
        "crypto_price",
        "Current price of the trading pair"
    ).unwrap();

    let registry = Registry::new();

    registry.register(Box::new(gauge.clone())).expect("Failed to register gauge");
    // Register gauge with the default registry manually
    //default_registry().register(Box::new(gauge.clone())).expect("Failed to register gauge");
    log::info!("Prometheus Gauge  was registered");
    Arc::new(Mutex::new(gauge))
});

#[derive(Debug, Clone, Default)]
pub struct StatsRecorderService;

impl StatsRecorderService{
    pub fn track_price(price: f64) {
        // Update the shared gauge
        let gauge = PRICE_GAUGE.lock().unwrap();
        //log::info!("PRICE:: {}",price);
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
}