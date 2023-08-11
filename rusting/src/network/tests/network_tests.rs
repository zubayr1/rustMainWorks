use futures::future::try_join_all;
use tokio::task::JoinHandle;
use tokio::time::sleep;
use tokio::time::Duration;

use super::*;

#[tokio::test]
async fn send() {
    // Create a network sender and run it.
    let (tx, rx) = channel(10);
    let mut sender = NetworkSender::new(rx);
    tokio::spawn(async move {
        sender.run().await;
    });

    // Run a dummy TCP server.
    let address = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let handle = listener(address);

    // Send a message via the network sender.
    let message = NetworkMessage {
        sender: address,
        addresses: vec![address],
        message: "Hello, World!".to_string(),
    };
    let _ = tx.send(message).await;

    // Use the handle to check if the sender successfully transmitted the data over the TCP
    // connection.
    assert!(handle.await.is_ok());
}

#[tokio::test]
async fn broadcast() {
    // Create a network sender and run it.
    let (tx, rx) = channel(10);
    let mut sender = NetworkSender::new(rx);
    tokio::spawn(async move {
        sender.run().await;
    });

    // Run 5 dummy TCP servers.
    let (handles, addresses): (Vec<_>, Vec<_>) = (0..5)
        .map(|x| {
            let address = format!("127.0.0.1:{}", 8000 + x)
                .parse::<SocketAddr>()
                .unwrap();
            (listener(address), address)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .unzip();

    // Broadcast a message via the network sender.
    let message = NetworkMessage {
        sender: addresses[0],
        addresses,
        message: "Hello, World!".to_string(),
    };
    let _ = tx.send(message).await;

    // Use the handle to check if the sender successfully transmitted the data over the TCP
    // connections.
    assert!(try_join_all(handles).await.is_ok());
}

#[tokio::test]
async fn receive() {
    // Create a network receiver and run it.
    let address = "127.0.0.1:8070".parse::<SocketAddr>().unwrap();
    let (tx, mut rx) = channel(10);
    let receiver = NetworkReceiver::new(address.clone(), tx);
    tokio::spawn(async move {
        receiver.run().await;
    });

    // Sleep to make sure the receiver is ready.
    sleep(Duration::from_millis(50)).await;

    // Create a message and serialize it.
    let message = NetworkMessage{sender: address, addresses: vec![address], message: "Hello, World!".to_string()};
    let bytes = Bytes::from(bincode::serialize(&message).unwrap());

    // Connect to the address of the receiver.
    let stream = TcpStream::connect(address).await.unwrap();
    let mut transport = Framed::new(stream, LengthDelimitedCodec::new());

    // Send the message to the receiverr via the TCP connection.
    transport.send(bytes.clone()).await.unwrap();

    // Make sure the message receives the receiver and gets passed into the channel.
    match rx.recv().await {
        Some(val) => assert_eq!(val, message),
        _ => assert!(false),
    }
}

// Helper funtion that creates a TCP connection and checks if there is data sent over this
// connection.
pub fn listener(address: SocketAddr) -> JoinHandle<()> {
    tokio::spawn(async move {
        // Bind to the given address.
        let listener = TcpListener::bind(&address).await.unwrap();
        // Accept an incoming connection.
        let (socket, _) = listener.accept().await.unwrap();
        // Frame the TCP connection
        let mut transport = Framed::new(socket, LengthDelimitedCodec::new());

        // Check if there is something sent over the connection.
        match transport.next().await {
            Some(Ok(_)) => assert!(true),
            _ => assert!(false),
        }
    })
}
