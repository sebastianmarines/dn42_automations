use crate::peer;
use defguard_wireguard_rs::key::Key;
use defguard_wireguard_rs::net::IpAddrMask;
use defguard_wireguard_rs::{InterfaceConfiguration, WGApi, WireguardInterfaceApi};
use futures::TryStreamExt;
use log::{info, warn};
use rtnetlink::{new_connection, Error, Handle};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, ToSocketAddrs};
use x25519_dalek::PublicKey;

pub async fn create_peering(peer: peer::Peer) -> Result<(), Box<dyn std::error::Error>> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    if interface_exists(handle.clone(), peer.name.clone())
        .await
        .map_err(|e| e.to_string())?
    {
        warn!("Interface {} already exists", peer.name);
    } else {
        info!("Creating interface {}", peer.name);
        create_interface(handle.clone(), peer.name.clone()).await?;
    }

    create_wg_interface(&peer).await?;
    Ok(())
}

pub async fn interface_exists(handle: Handle, name: String) -> Result<bool, Error> {
    info!("Checking if interface {} exists", name);
    let mut links = handle.link().get().match_name(name.clone()).execute();
    let link = links.try_next().await;

    if let Err(Error::NetlinkError(_)) = &link {
        return Ok(false);
    }

    if link?.is_some() && links.try_next().await?.is_none() {
        Ok(true)
    } else {
        Ok(false)
    }
}

pub async fn create_interface(handle: Handle, name: String) -> Result<(), String> {
    let _ = handle
        .link()
        .add()
        .wireguard(name.clone())
        .execute()
        .await
        .map_err(|e| format!("Failed to create interface: {}", e))?;

    Ok(())
}

pub async fn create_wg_interface(wg_peer: &peer::Peer) -> Result<(), Box<dyn std::error::Error>> {
    let wg_api = WGApi::new(wg_peer.name.clone(), false)?;
    let _ = wg_api.create_interface()?;

    let endpoint = wg_peer.endpoint.to_socket_addrs().unwrap();
    let endpoint = endpoint
        .collect::<Vec<SocketAddr>>()
        .first()
        .unwrap()
        .clone();

    let key = PublicKey::from(string_to_array_32(wg_peer.public_key.clone().as_str()));
    let peer_key: Key = key.as_ref().try_into().unwrap();
    let mut peer = defguard_wireguard_rs::host::Peer::new(peer_key.clone());
    peer.endpoint = Some(SocketAddr::new(endpoint.ip(), endpoint.port()));
    peer.allowed_ips = vec![
        IpAddrMask::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0),
        IpAddrMask::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 0),
    ];

    let interface_config = InterfaceConfiguration {
        name: wg_peer.name.clone(),
        prvkey: "iE3WDdxWWs/uxTXZ5cX4g1hP+tqP30wxVjFripu5418=".to_string(),
        address: "172.20.196.64".to_string(),
        peers: vec![peer],
        port: 51820,
    };

    wg_api.configure_interface(&interface_config)?;

    Ok(())
}

fn string_to_array_32(s: &str) -> [u8; 32] {
    let mut array = [0u8; 32];
    let bytes = s.as_bytes();

    // Copy bytes into the array, up to 32 bytes
    let len = bytes.len().min(32);
    array[..len].copy_from_slice(&bytes[..len]);

    array
}
