use crate::ports::ws_client_trade;

mod ports;
mod domain;
mod config;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Log the start of the application
    log::info!("Starting application...");

    //let trade_history_service = Arc::new(TradeHistoryService);

    let websocket_trade_handle = tokio::spawn( async{
        log::info!("Starting Trade Stream WebSocket client...");
        ws_client_trade::start_websocket().await;
    });

    tokio::try_join!(websocket_trade_handle).unwrap();
}
