use std::net::SocketAddr;

use bytes::Bytes;
use rand::distributions::{Alphanumeric, DistString};
use rand::{thread_rng, Rng};
use tokio::sync::mpsc::{channel, Receiver};
use tokio::time::Duration;

use crate::{message::BroadcastMessage, network::SimpleSender};

pub struct Core {
    id: usize,                          // id of the node.
    nodes: Vec<SocketAddr>,             // ip addresses of all nodes.
    sender: SimpleSender,               // Network sender.
    rx: Receiver<BroadcastMessage>,     // Channel to receive network messages.
    rx_tick: Receiver<bool>,            // Channel to receive ticks.
}

impl Core {
    pub fn spawn(
        id: usize,
        nodes: Vec<SocketAddr>,
        sender: SimpleSender,
        rx: Receiver<BroadcastMessage>,
    ) {
        let (tx_tick, rx_tick) = channel(10);

        // Spawn a ticker that sends a value to rx_tick at a random value between 500ms and 1000ms.
        tokio::spawn(async move {
            loop {
                let duration = thread_rng().gen_range(500..1000);
                tokio::time::sleep(Duration::from_millis(duration)).await;
                tx_tick.send(true).await.unwrap();
            }
        });

        tokio::spawn(async move {
            Self {
                id,
                nodes,
                sender,
                rx,
                rx_tick,
            }
            .run()
            .await;
        });
    }

    /// Broadcast a given message to every node in the network.
    async fn broadcast(&mut self, m: BroadcastMessage) {
        let bytes = Bytes::from(bincode::serialize(&m).unwrap());
        self.sender.broadcast(self.nodes.clone(), bytes).await;
    }

    pub async fn run(&mut self) {
        // Listen to incoming messages and process them. Note: self.rx is the channel where we can
        // retreive data from the message receiver.
        loop {
            tokio::select! {
                Some(message) = self.rx.recv() => {
                    println!("{} got message from {}: {}", self.id, message.sender, message.content);
                }
                Some(_) = self.rx_tick.recv() => {
                    // Create random string.
                    let content = Alphanumeric.sample_string(&mut thread_rng(), 32);
                    println!("{} broadcasting {}", self.id, content);
                    self.broadcast(
                        BroadcastMessage { sender: self.id, content: content, round: 0 }
                    ).await;
                }
            };
        }
    }
}
