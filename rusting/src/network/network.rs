use crate::message::NetworkMessage;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use std::{collections::HashMap, net::SocketAddr};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{channel, Receiver, Sender},
};
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tokio::time::{sleep, Duration};



pub struct NetworkSender {
    // Channel for communication between NetworkSender and other threads.
    transmit: Receiver<NetworkMessage>,
}

impl NetworkSender {
    pub fn new(transmit: Receiver<NetworkMessage>) -> Self {
        Self { transmit }
    }

    // Kepp one TCP connection per peer, handled by a seperate thread. Communication is done via
    // dedicated channels for every worker.
    pub async fn run(&mut self) {
        // Keep track of workers. Maps socket address to sender channel for worker.
        let mut senders = HashMap::<SocketAddr, Sender<NetworkMessage>>::new();

        // Receive messages from channel.
        while let Some(m) = self.transmit.recv().await {
            for address in &m.addresses {
                // Look up socket address of receiver in hash map.
                let spawn = match senders.get(&address) {
                    // If entry in hash map exists use the channel to send the message to the worker. If
                    // there is an error with the channel spawn a new worker for the receiver socket
                    // address.
                    Some(tx) => tx.send(m.clone()).await.is_err(),
                    // If there is no entry spawn a new worker for the receiver socket address.
                    None => true,
                };

                if spawn {
                    // Spawn a new worker for the receiver socket address.
                    let tx = Self::spawn_worker(*address).await;

                    // Send the new worker the message via a channel.
                    if let Ok(()) = tx.send(m.clone()).await {
                        // If sending was successful put the channel into the hash map.
                        senders.insert(*address, tx);
                    }
                }
            }
        }
    }

    async fn spawn_worker(address: SocketAddr) -> Sender<NetworkMessage> {
        // Create channel for communication with NetworkSender.
        let (tx, mut rx) = channel(10_000);

        tokio::spawn(async move {
            // Initialize a stream variable
            let stream: Option<TcpStream>;

            // Loop until the stream is successfully connected
            loop {
                // Try to connect to the socket address
                match TcpStream::connect(address).await {
                    Ok(s) => {
                        //println!("Outgoing connection established with {}", address);
                        // Assign the stream to the variable
                        stream = Some(s);
                        // Break the loop
                        break;
                    }
                    Err(_) => {
                        // println!("Failed to connect to {}: {}", address, e);
                        // Sleep for 1 second before retrying
                        sleep(Duration::from_millis(10)).await;
                    }
                }
            }

    // Unwrap the stream
    let stream = stream.unwrap();

            // Frame the TCP stream.
            let mut transport = Framed::new(stream, LengthDelimitedCodec::new());

            // Continuously listen to messages passed to the above created channel.
            while let Some(message) = rx.recv().await {
                // Serialize message
                let bytes = Bytes::from(bincode::serialize(&message).expect("Failed to serialize"));

                // Send the message to the nework
                match transport.send(bytes).await {
                    Ok(_) => {
                        //println!("Successfully sent message to {}", address)
                    },
                    Err(e) => {
                        println!("Failed to send message to {}: {}", address, e);
                        return;
                    }
                }
            }
        });
        tx
    }
}





pub struct NetworkReceiver {
    // Our own network address.
    address: SocketAddr,

    // Channel where received messages are put in.
    deliver: Sender<NetworkMessage>,
}

impl NetworkReceiver {
    pub fn new(address: SocketAddr, deliver: Sender<NetworkMessage>) -> Self {
        Self { address, deliver }
    }

    // Spawn a new worker for each incoming request. This worker is responsible for
    // receiving messages from exactly one connection and forwards those messages to
    // the deliver channel.
    pub async fn run(&self) {
        let split: Vec<String> = self.address.to_string().split(":").map(|s| s.to_string()).collect();
        let address_str = format!("{}:{}", "0.0.0.0".to_string(), split[1]);

        let address = address_str.parse::<SocketAddr>().unwrap();

        let listener = TcpListener::bind(address)
            .await
            .expect("Failed to bind TCP port");

        println!("Listening on {}", address);

        // Continuously accept new incoming connections.
        loop 
        {
            let (socket, peer) = match listener.accept().await {
                Ok(value) => value,
                // If there is an error with the connection just continue with the loop.
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
            //println!("incoming connection established with {}", peer);
            // Spawn a new worker that handles the just established connection.
            Self::spawn_worker(socket, peer, self.deliver.clone()).await;
        }
    }

    async fn spawn_worker(socket: TcpStream, peer: SocketAddr, deliver: Sender<NetworkMessage>) {
        tokio::spawn(async move {
            // Frame the TCP stream.
            let mut transport = Framed::new(socket, LengthDelimitedCodec::new());

            // Continuously receive incoming data from the framed TCP stream.
            while let Some(frame) = transport.next().await {
                match frame {
                    Ok(m) => {
                        // Deserialize received message.
                        let message = bincode::deserialize(&m.freeze()).unwrap();
                        match deliver.send(message).await {
                            Ok(_) => (),
                            Err(e) => println!("{}", e),
                        }
                    }
                    // If there is some error with the framed TCP stream return. This will
                    // kill the worker thread.
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                }
            }
            println!("Connection closed by peer {}", peer);
        });
    }
}
