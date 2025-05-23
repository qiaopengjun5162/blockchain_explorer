mod app;
mod eth_connection_example;
mod chains;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");
    env_logger::init();

    match app::start().await {
        Ok(_) => log::info!("Server stopped"),
        Err(e) => log::error!("Server stopped with error: {}", e),
    }
}
