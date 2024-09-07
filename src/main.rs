use std::sync::Arc;
use std::sync::RwLock;
use libp2p::{identity, PeerId, Swarm};
use libp2p::kad::{Kademlia, KademliaConfig};
use libp2p::tcp::TcpConfig;
use libp2p::core::transport::Transport;
use libp2p::noise::{Keypair, NoiseConfig, X25519Spec};
use libp2p::yamux::YamuxConfig;
use libp2p::swarm::SwarmBuilder;
use sled::Db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the database
    let db = Arc::new(RwLock::new(sled::open("prustdb_data")?));

    // Create a random PeerId
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    // Set up an encrypted DNS-enabled TCP Transport
    let noise_keys = Keypair::<X25519Spec>::new().into_authentic(&local_key).unwrap();
    let transport = TcpConfig::new()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(YamuxConfig::default())
        .boxed();

    // Create a Kademlia behavior
    let mut kademlia = Kademlia::new(local_peer_id, KademliaConfig::default());

    // Create a Swarm to manage peers and events
    let mut swarm = SwarmBuilder::new(transport, kademlia, local_peer_id)
        .executor(Box::new(|fut| {
            tokio::spawn(fut);
        }))
        .build();

    // Listen on all interfaces and random OS-assigned port
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // Start the main event loop
    loop {
        match swarm.next_event().await {
            // Handle events here
            _ => {}
        }
    }
}

// Helper functions for database operations
fn set(db: Arc<RwLock<Db>>, key: &[u8], value: &[u8]) -> Result<(), sled::Error> {
    db.write().unwrap().insert(key, value)?;
    Ok(())
}

fn get(db: Arc<RwLock<Db>>, key: &[u8]) -> Result<Option<Vec<u8>>, sled::Error> {
    Ok(db.read().unwrap().get(key)?.map(|ivec| ivec.to_vec()))
}
