use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::error::Error;
use std::{ time};
use tokio::time::{ sleep, Duration};

#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    println!("trying to connect from {} to address {}", self_ip, address);

    // while TcpStream::connect(address.clone()).await.is_err() //waiting for server to be active, if not random wait and retry
    // {
    //     sleep(Duration::from_millis(10)).await;
    // }

    loop 
    {

        let std_stream = std::net::TcpStream::connect(address.clone())?;
        std_stream.set_nonblocking(true)?;
        let stream = TcpStream::from_std(std_stream);

        if stream.is_err()
        {   
           // let result = stream.unwrap().shutdown().await;
            sleep(Duration::from_millis(10)).await;
        }
        else if stream.is_ok() 
        {
            println!("b");
            let result = stream.unwrap().shutdown().await;
            break;
        }
    }
    



    let std_stream = std::net::TcpStream::connect(address.clone())?;
    std_stream.set_nonblocking(true)?;
    let mut stream = TcpStream::from_std(std_stream)?;
      

   println!("connected from {} to address {}", self_ip, address);


  loop{
  
    // Write some data.
    stream.write_all([self_ip.to_string(), self_ip.to_string().to_string()].join(" ").as_bytes()).await.unwrap();
    let result = stream.write_all(b"hello world!EOF").await;
    
    let mut data = [0u8; 12];
    let res = stream.read_exact(&mut data);

    println!("{:?}", res);
   
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