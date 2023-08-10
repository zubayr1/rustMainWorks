use std::net::SocketAddr;

use tokio::sync::mpsc::channel;
use tokio::time::{sleep, Duration};

use crate::{
    core::Core,
    network::{MessageReceiver, SimpleSender},
};


pub struct Node;

impl Node {
    pub async fn new(id: usize, nodes: Vec<SocketAddr>) {
        // Create a channel for the message receiver. The receiver receives data from incoming
        // tcp connections and puts this data into the channel. The data is retreives via the rx
        // channel.
        let (tx, rx) = channel(1_000);
        MessageReceiver::spawn(nodes[id], tx);
        let sender = SimpleSender::new();

        sleep(Duration::from_millis(50)).await;

        Core::spawn(id, nodes, sender, rx);
    }
}
