use log::info;
use simple_logger::SimpleLogger;

mod peer;
mod wireguard;

#[tokio::main]
async fn main() -> Result<(), String> {
    SimpleLogger::new()
        .with_colors(true)
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    let peers: peer::Peers = vec![peer::Peer {
        name: "Kioubit".to_string(),
    }];

    for peer in peers {
        info!("Creating peering with peer: {}", peer.name);
        wireguard::create_peering(peer).await?;
    }

    Ok(())
}
