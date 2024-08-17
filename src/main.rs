mod wireguard;

struct Peer {
    name: String,
}

type Peers = Vec<Peer>;

#[tokio::main]
async fn main() -> Result<(), String> {
    let peers: Peers = vec![Peer {
        name: "Kioubit".to_string(),
    }];

    for peer in peers {
        println!("Creating peering with peer: {}", peer.name);
        wireguard::create_peering(peer).await?;
    }

    Ok(())
}
