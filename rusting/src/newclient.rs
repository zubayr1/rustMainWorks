use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::{ time};
use tokio::time::{ sleep, Duration};
use std::panic;
use std::format;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
pub async fn match_tcp_client(address: String, test_address: String, self_ip: String, types: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
//    println!("trying to connect from {} to address {}", self_ip, address);

    while TcpStream::connect(test_address.clone()).await.is_err() //waiting for server to be active, if not random wait and retry
    {
        sleep(Duration::from_millis(3)).await;
    }

    
    
    let mut stream: TcpStream = TcpStream::connect(address.clone()).await?;
    // stream.set_linger(Some(Duration::from_secs(10))).expect("set_linger call failed");



  loop{
    // Write some data.
    stream.write_all([self_ip.to_string(), address.to_string().to_string()].join(" ").as_bytes()).await?;
    
    
    if types=="first"
    {
        let result = stream.write_all(b"hello world!EOF").await;
        if  result.is_ok()
        {
            break;
        }
        if result.is_err()
        {
            continue;
        }
    }
    else {
        let result = stream.write_all(b"hello world!EOFEOF").await;
        if  result.is_ok()
        {
            break;
        }
        if result.is_err()
        {
            continue;
        }
    }
    
    
    

 }

    Ok(())
}