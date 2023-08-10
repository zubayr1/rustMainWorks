use std::{collections::HashMap, net::SocketAddr};

use bytes::Bytes;
use futures::stream::StreamExt as _;
use futures::SinkExt;
use tokio::{
    net::TcpStream,
    sync::mpsc::{channel, Receiver, Sender},
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub struct SimpleSender {
    // Keep track of exisitng connections.
    connections: HashMap<SocketAddr, Sender<Bytes>>,
}

/// Keep alive one TCP connection per peer, each connection is handled by a separate thread.
impl SimpleSender {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    fn spawn_connection(address: SocketAddr) -> Sender<Bytes> {
        let (tx, rx) = channel(1_000);
        Connection::spawn(address, rx);
        tx
    }

    /// Sends given data to a given address.
    pub async fn send(&mut self, address: SocketAddr, data: Bytes) {
        // If we already have a connection established to the given address we use this connection.
        if let Some(tx) = self.connections.get(&address) {
            // We put the given data to a channel, where it can be retrieved and send via the
            // existing tcp connection.
            if tx.send(data.clone()).await.is_ok() {
                return;
            }
        }

        // Otherwise make a new connection and store it in the hashmap.
        let tx = Self::spawn_connection(address);
        if tx.send(data).await.is_ok() {
            self.connections.insert(address, tx);
        }
    }

    /// Sends given data to all given address.
    pub async fn broadcast(&mut self, addresses: Vec<SocketAddr>, data: Bytes) {
        for address in addresses {
            self.send(address, data.clone()).await;
        }
    }
}

/// A Connection to a single peer.
struct Connection {
    /// Destination address.
    address: SocketAddr,
    /// Channel to receive data from.
    receiver: Receiver<Bytes>,
}

impl Connection {
    fn spawn(address: SocketAddr, receiver: Receiver<Bytes>) {
        tokio::spawn(async move {
            Self { address, receiver }.run().await;
        });
    }

    /// Main loop for connecting and transmitting.
    async fn run(&mut self) {
        // Try to connect to the peer.
        let (mut writer, _) = match TcpStream::connect(self.address).await {
            Ok(stream) => Framed::new(stream, LengthDelimitedCodec::new()).split(),
            Err(_e) => {
                // TODO: log
                return;
            }
        };

        // Transmit messages
        loop {
            // If there is data in the channel retreive it and send it via the tcp connection.
            if let Some(data) = self.receiver.recv().await {
                if let Err(_e) = writer.send(data.into()).await {
                    // TODO: log
                    return;
                }
            }
        }
    }
}
