use std::net::SocketAddr;

use rand::distributions::{Alphanumeric, DistString};
use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::time::Duration;

use crate::message::NetworkMessage;

pub struct Core {
    id: usize,                    // id of the node.
    name: SocketAddr,             // Note: a public key would make more sense as name.
    nodes: Vec<SocketAddr>,       // ip addresses of all nodes.
    tx: Sender<NetworkMessage>,   // Channel to send messages to the network.
    rx: Receiver<NetworkMessage>, // Channel to receive network messages.
    rx_tick: Receiver<bool>,      // Channel to receive ticks.
}

impl Core {
    pub fn spawn(
        id: usize,
        name: SocketAddr,
        nodes: Vec<SocketAddr>,
        tx: Sender<NetworkMessage>,
        rx: Receiver<NetworkMessage>,
    ) {
        let (tx_tick, rx_tick) = channel(10);

        // Spawn a ticker that sends a value to rx_tick at a random value between 20ms and 500ms.
        tokio::spawn(async move {
            loop {
                let duration = thread_rng().gen_range(20..500);
                tokio::time::sleep(Duration::from_millis(duration)).await;
                tx_tick.send(true).await.unwrap();
            }
        });

        tokio::spawn(async move {
            Self {
                id,
                name,
                nodes,
                tx,
                rx,
                rx_tick,
            }
            .run()
            .await;
        });
    }

    /// Broadcast a given message to every node in the network.
    async fn broadcast(&mut self, m: String) {
        let message = NetworkMessage {
            sender: self.name.clone(),
            addresses: self.nodes.clone(),
            message: m.clone(),
        };
        match self.tx.send(message).await {
            Ok(_) => (),
            Err(e) => println!("{}", e),
        }
    }

    pub async fn run(&mut self) {
        // Listen to incoming messages and process them. Note: self.rx is the channel where we can
        // retreive data from the message receiver.
        loop {
            tokio::select! {
                Some(message) = self.rx.recv() => {
                    println!("{} got message from {}: {}", self.id, message.sender, message.message);
                }
                Some(_) = self.rx_tick.recv() => {
                    // Create random string.
                    let content = Alphanumeric.sample_string(&mut thread_rng(), 32);
                    println!("{} broadcasting {}", self.id, content);
                    self.broadcast(content).await;
                }
            };
        }
    }
}
