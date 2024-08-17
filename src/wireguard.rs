use crate::Peer;
use futures::TryStreamExt;
use rtnetlink::{new_connection, Error, Handle};

pub async fn create_peering(peer: Peer) -> Result<(), String> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    if interface_exists(handle.clone(), peer.name.clone())
        .await
        .map_err(|e| e.to_string())?
    {
        println!("Interface {} already exists", peer.name);
        Ok(())
    } else {
        println!("Creating peering with peer: {}", peer.name);
        create_interface(handle.clone(), peer.name.clone()).await?;
        Ok(())
    }
}

pub async fn interface_exists(handle: Handle, name: String) -> Result<bool, Error> {
    println!("Checking if interface {} exists", name);
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
