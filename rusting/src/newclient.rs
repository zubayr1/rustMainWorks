use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use tokio::time::{ sleep, Duration};
use tokio::fs::{OpenOptions};


#[tokio::main]
pub async fn match_tcp_client(address: String, test_address: String, self_ip: String, types: String) -> Result<(), Box<dyn Error>> {

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    // Connect to a peer

    while TcpStream::connect(test_address.clone()).await.is_err() //waiting for server to be active, if not random wait and retry
    {
        sleep(Duration::from_millis(3)).await;
    }    
    
    let mut stream: TcpStream = TcpStream::connect(address.clone()).await?;

  loop{
    // Write some data.
    stream.write_all([self_ip.to_string(), address.to_string().to_string()].join(" ").as_bytes()).await?;
    
    
    if types=="first"
    {
        let result = stream.write_all(b"hello world!EOF").await;
        if  result.is_ok()
        {
            let text = ["client at: ".to_string(), self_ip.to_string()].join(": ");
            file.write_all(text.as_bytes()).await.unwrap();
            file.write_all(b"\n").await.unwrap();
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