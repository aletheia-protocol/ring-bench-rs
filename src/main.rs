use std::sync::Arc;
use warp::Filter;
use crate::ports::stats::stats_recorder_service::StatsRecorderService;
use crate::ports::ws_client_trade;

mod ports;
mod domain;
mod config;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Log the start of the application
    log::info!("Starting application...");

    let websocket_trade_handle = tokio::spawn( async{
        log::info!("Starting Trade Stream WebSocket client...");
        ws_client_trade::start_websocket().await;
    });

    // Serve metrics on port 3030
    let metrics_server = tokio::spawn(async {

        // Metrics route for Prometheus
        let metrics_route = warp::path("metrics")
            .and(warp::get())
            .map(|| {
                let metrics = StatsRecorderService::serve_metrics();
                warp::reply::with_header(metrics, "Content-Type", "text/plain")
            });

        warp::serve(metrics_route).run(([0, 0, 0, 0], 3030)).await;
    });

    tokio::try_join!(websocket_trade_handle,metrics_server).unwrap();
}
