use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;

#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect(address.clone()).await?;
    // Write some data.
    stream.write_all(self_ip.as_bytes()).await?;
    stream.write_all(b"hello world!EOF").await?;
    // stream.shutdown().await?;
    Ok(())
}