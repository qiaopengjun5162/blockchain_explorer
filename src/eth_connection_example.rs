use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use futures_util::StreamExt;

use crate::chains::{
    chain_provider::{self, ChainProvider, ConnectionType},
    constants::alchemy,
};

pub async fn eth_connection_example() -> anyhow::Result<()> {
    let chain_provider = ChainProvider::new()
        .with_name("sepolia".to_string())
        .with_connection_type(ConnectionType::Ws)
        .with_is_mainnet(false)
        .with_rpc_url(alchemy::ethereum::MAINNET.to_string())
        .with_provider_service(chain_provider::ProviderService::Alchemy)
        .with_api_key(std::env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY must be set"));

    log::info!("Generated URL: {}", chain_provider.to_string());
    // Create the provider.
    let ws = WsConnect::new(chain_provider.to_string());
    let provider = ProviderBuilder::new().connect_ws(ws).await?;

    // Subscribe to new blocks.
    let sub = provider.subscribe_blocks().await?;

    // Wait and take the next 4 blocks.
    let mut stream = sub.into_stream().take(4);
    // let mut stream = sub.into_stream();

    println!("Awaiting block headers...");

    // Take the stream and print the block number upon receiving a new block.
    let handle = tokio::spawn(async move {
        while let Some(header) = stream.next().await {
            println!("Latest block number: {}", header.number);
        }
    });

    handle.await?;

    Ok(())
}
