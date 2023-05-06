use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String, types: String, epoch: i32, behavior: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    println!("client {}", address);
    let mut stream = TcpStream::connect(address.clone()).await?;
    println!("client done");
    // Write some data.
    stream.write_all(address.as_bytes()).await?;
    stream.write_all(b"hello world!EOF").await?;
    // stream.shutdown().await?;
    Ok(())
}