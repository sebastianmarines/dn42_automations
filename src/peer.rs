pub struct Peer {
    pub name: String,
    pub public_key: String,
    pub endpoint: String,
}

pub type Peers = Vec<Peer>;
