use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::{ time};
use tokio::time::{ sleep, Duration};
use std::panic;
use std::format;

#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    println!("trying to connect from {} to address {}", self_ip, address);

    // while TcpStream::connect(address.clone()).await.is_err() //waiting for server to be active, if not random wait and retry
    // {
    //     sleep(Duration::from_millis(10)).await;
    // }

    // loop 
    // {
    //     let stream = TcpStream::connect(address.clone()).await;

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
    const CONNECTION_TIME: u64 = 10000;


    let mut stream = match tokio::time::timeout(
        Duration::from_secs(CONNECTION_TIME),
        TcpStream::connect(address.clone())
    )
    .await
    {
        Ok(ok) => ok,
        Err(e) => panic!("timeout"),
    }
    .expect("Error while connecting to server");
    
    
    // let mut stream: TcpStream = TcpStream::connect(address.clone()).await?;
    stream.set_linger(Some(Duration::from_secs(10))).expect("set_linger call failed");

   println!("connected from {} to address {}", self_ip, address);


  loop{
  
    // Write some data.
    stream.write_all([self_ip.to_string(), self_ip.to_string().to_string()].join(" ").as_bytes()).await.unwrap();
    let result = stream.write_all(b"hello world!EOF").await;
    
   
    if  result.is_ok()
    {
        break;
    }
    if result.is_err()
    {
        println!("some err");
    }

 }
    Ok(())
}