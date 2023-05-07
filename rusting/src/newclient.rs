use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::{ time};
use tokio::time::{ sleep, Duration};

#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    println!("trying to connect from {} to address {}", self_ip, address);

    while TcpStream::connect(address.clone()).await.is_err() //waiting for server to be active, if not random wait and retry
    {
        sleep(Duration::from_millis(10)).await;
    }
    
  //  loop{
            let mut stream: TcpStream = TcpStream::connect(address.clone()).await?;

   
    

   println!("connected from {} to address {}", self_ip, address);


  loop{
  
    // Write some data.
    stream.write([self_ip.to_string(), self_ip.to_string().to_string()].join(" ").as_bytes()).await.unwrap();
    let result = stream.write(b"hello world!EOF").await;
    
   
//    if  result.is_ok()
//    {
//     println!("ok");
//    }
//    if result.is_err()
//    {
//     println!("some err");
//    }

 }
    Ok(())
}