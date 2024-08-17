use log::info;
use simple_logger::SimpleLogger;
use std::error::Error;

mod peer;
mod wireguard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let peers: peer::Peers = vec![peer::Peer {
        name: "sunnet".to_string(),
        public_key: "G/ggwlVSy5jKWFlJM01hxcWnL8VDXsD5EXZ/S47SmhM=".to_string(),
        endpoint: "sjc1-us.dn42.6700.cc:22799".to_string(),
    }];

    for peer in peers {
        info!("Creating peering with peer: {}", peer.name);
        wireguard::create_peering(peer).await?;
    }

    Ok(())
}
