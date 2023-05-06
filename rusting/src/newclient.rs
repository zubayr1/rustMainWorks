use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use socket2;
use std::{ time};
use tokio::time::{ sleep, Duration};

#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    println!("trying to connect from {} to address {}", self_ip, address);

    
      loop{
            let mut stream: TcpStream = TcpStream::connect(address.clone()).await.expect(
                "except reached"
            );

   
    

    println!("connected from {} to address {}", self_ip, address);




    // Write some data.
    stream.write_all(self_ip.as_bytes()).await?;
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