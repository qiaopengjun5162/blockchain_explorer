use crate::eth_connection_example::eth_connection_example;

pub async fn start() -> anyhow::Result<()> {
    log::warn!("Blockchain explorer application starting...");

    match eth_connection_example().await {
        Ok(_) => log::warn!("Blockchain explorer application started successfully."),
        Err(err) => log::error!("Error starting blockchain explorer application: {}", err),
    }

    Ok(())
}
