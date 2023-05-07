use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use socket2;
use std::{ time};
use tokio::time::{ sleep, Duration};


pub async fn wait(address: String) {
    // First wait for all nodes to be online.
      let result =  tokio::spawn(async move {
            while TcpStream::connect(address.clone()).await.is_err() {
                sleep(Duration::from_millis(10)).await;
            }
        })
    .await;

   
}


#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String) -> Result<(), Box<dyn Error>> {
    // Connect to a peer
    println!("trying to connect from {} to address {}", self_ip, address);

 
   // wait(address.clone()).await;
  //  loop{
            let mut stream: TcpStream = TcpStream::connect(address.clone()).await?;

   
    

   println!("connected from {} to address {}", self_ip, address);


//    sleep(Duration::from_millis(300)).await;
  
    // Write some data.
    stream.write([self_ip.to_string(), self_ip.to_string().to_string()].join(" ").as_bytes()).await.unwrap();
    let result = stream.write(b"hello world!EOF").await;
    
   
//    if  result.is_ok()
//    {
//     println!("ok");
//     break;
//    }
//    if result.is_err()
//    {
//     println!("some err");
//    }

// }
    Ok(())
}