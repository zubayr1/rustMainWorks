use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use socket2::{Socket, Domain, Type};
use std::{thread, time};

#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    let mut stream = TcpStream::connect(address.clone()).await?;

    let sock_ref = socket2::SockRef::from(&stream);

    let mut ka = socket2::TcpKeepalive::new();
    ka = ka.with_time(time::Duration::from_secs(20));
    ka = ka.with_interval(time::Duration::from_secs(20));

    sock_ref.set_tcp_keepalive(&ka)?;


    // Write some data.
    stream.write_all(self_ip.as_bytes()).await?;
    stream.write_all(b"hello world!EOF").await?;
    
    
    Ok(())
}