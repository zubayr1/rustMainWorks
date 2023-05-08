use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::{ time};
use tokio::time::{ sleep, Duration};
use std::panic;
use std::format;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
pub async fn match_tcp_client(address: String, test_address: String, self_ip: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
   // println!("trying to connect from {} to address {}", self_ip, address);

    while TcpStream::connect(test_address.clone()).await.is_err() //waiting for server to be active, if not random wait and retry
    {
        sleep(Duration::from_millis(10)).await;
    }

    // loop 
    // {
    //     let stream = TcpStream::connect(test_address.clone()).await;

    //     if stream.is_err()
    //     {   
    //        // let result = stream.unwrap().shutdown().await;
    //         sleep(Duration::from_millis(10)).await;
    //     }
    //     else if stream.is_ok() 
    //     {
    //         let result = stream.unwrap().shutdown().await;

    //         break;
    //     }
    // }
    // const CONNECTION_TIME: u64 = 10000;


    // let mut stream = match tokio::time::timeout(
    //     Duration::from_secs(CONNECTION_TIME),
    //     TcpStream::connect(address.clone())
    // )
    // .await
    // {
    //     Ok(ok) => ok,
    //     Err(e) => panic!("timeout"),
    // }
    // .expect("Error while connecting to server");

    // let addr = std::net::SocketAddr::new(IpAddr::V4(Ipv4Addr::new(44, 204, 90, 157)), 7082);
    
    // let mut stream = std::net::TcpStream::connect_timeout(&addr, Duration::from_secs(10));

    let mut stream: TcpStream = TcpStream::connect(address.clone()).await?;
    // stream.set_linger(Some(Duration::from_secs(10))).expect("set_linger call failed");

  // println!("connected from {} to address {}", self_ip, address);


  loop{
    // Write some data.
    stream.write_all([self_ip.to_string(), address.to_string().to_string()].join(" ").as_bytes()).await?;
    let result = stream.write_all(b"hello world!EOF").await;
    
    
    if  result.is_ok()
    {
        println!("ok");
        break;
    }
    if result.is_err()
    {
        println!("some err");
    }

 }
    Ok(())
}