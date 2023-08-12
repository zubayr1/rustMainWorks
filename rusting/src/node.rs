use std::net::SocketAddr;

use tokio::sync::mpsc::channel;
use tokio::time::{sleep, Duration};

use crate::{core::Core, network::*};

pub struct Node;

impl Node {
    pub async fn new(id: usize, nodes: Vec<SocketAddr>, self_ip: String) {
        // Create channels for the networking.
        let (tx_rec, rx_rec) = channel(10_000);
        let (tx_send, rx_send) = channel(10_000);

        // Create a network receiver and sender.
        let network_receiver = NetworkReceiver::new(tx_rec);
        let mut network_sender = NetworkSender::new(rx_send);

        tokio::spawn(async move {
            network_receiver.run().await;
        });
        tokio::spawn(async move {
            
            network_sender.run().await;
        });
        // sleep(Duration::from_millis(50)).await;

        println!("{:?}", self_ip);

        let self_socket = self_ip.parse::<SocketAddr>().unwrap();

        Core::spawn(id, self_socket, nodes, tx_send, rx_rec);

        println!("check");

        sleep(Duration::from_millis(1000)).await;
        println!("checked");
    }
}
