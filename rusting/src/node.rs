use std::net::SocketAddr;

use tokio::sync::mpsc::channel;
use tokio::time::{sleep, Duration};

use crate::{
    core::Core,
    network::*,
};
use crate::message::NetworkMessage;

pub struct Node;

impl Node {
    pub async fn new(id: usize, nodes: Vec<SocketAddr>) 
    {
         // Create channels for the networking.
         let (tx_rec, rx_rec) = channel(10_000);
         let (tx_send, rx_send) = channel(10_000);
 
         // Create a network receiver and sender.
        //  let network_receiver = NetworkReceiver::new(nodes[id], tx_rec);
         let mut network_sender = NetworkSender::new(rx_send);
 
        //  tokio::spawn(async move {
        //      network_receiver.run().await;
        //  });
         tokio::spawn(async move {
             network_sender.run().await;
         });
 
         sleep(Duration::from_millis(50)).await;
 
         Core::spawn(id, nodes[id], nodes, tx_send, rx_rec);
    }

    pub async fn create_binding() -> tokio::sync::mpsc::Receiver<NetworkMessage>
    {
        let (tx_rec, rx_rec) = channel(10_000);
        let network_receiver: NetworkReceiver = NetworkReceiver::new(tx_rec);

         tokio::spawn(async move {
             network_receiver.run().await;
         });
        
        return rx_rec;
    }


}
