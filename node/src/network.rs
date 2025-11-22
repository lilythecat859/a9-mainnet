use libp2p::{identity, PeerId, gossipsub, Multiaddr, Swarm};
use libp2p::quic::{tokio::Transport as QuicTransport, Config as QuicConfig};
use std::collections::HashMap;

pub struct Network {
    local_key: identity::Keypair,
    swarm: Swarm<gossipsub::Behaviour>,
    peers: HashMap<PeerId, Multiaddr>,
}

impl Network {
    pub async fn new(port: u16) -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let transport = QuicTransport::new(QuicConfig::new(&local_key));
        let behaviour = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(local_key.clone()),
            gossipsub::Config::default(),
        ).expect("config");
        let swarm = Swarm::new(transport, behaviour, PeerId::from(local_key.public()));
        Network { local_key, swarm, peers: HashMap::new() }
    }

    pub async fn start(&mut self) {
        loop {
            match self.swarm.select_next_some().await {
                libp2p::swarm::SwarmEvent::NewListenAddr { address, .. } => {
                    tracing::info!("Listening on {}", address);
                }
                libp2p::swarm::SwarmEvent::Behaviour(gossipsub::Event::Message {
                    propagation_source,
                    message_id,
                    message,
                }) => {
                    tracing::info!("MSG from {}: {:?}", propagation_source, message);
                }
                _ => {}
            }
        }
    }
}
