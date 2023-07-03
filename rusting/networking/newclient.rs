use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use tokio::time::{ sleep, Duration};
use tokio::fs::{OpenOptions};


#[tokio::main]
pub async fn match_tcp_client(address: String, test_address: String, value: Vec<String>, args: Vec<String>) -> Result<(), Box<dyn Error>> {

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    // Connect to a peer    
    
    while TcpStream::connect(test_address.clone()).await.is_err() //waiting for server to be active, if not random wait and retry
    {
        sleep(Duration::from_millis(3)).await;
    }    
    let mut stream: TcpStream = TcpStream::connect(address.clone()).await?;
    

    let mut self_ip="0.0.0.0".to_string();

    if let Ok(peer_addr) = stream.peer_addr() {
        self_ip = peer_addr.ip().to_string();
    } 

    let value_string = value.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", ");

    loop
    {
        // Write data.
        stream.write_all(self_ip.as_bytes()).await?;
        stream.write_all(args[6].to_string().as_bytes()).await?;
        
        let result = stream.write_all([value_string.clone(), "EOF".to_string().to_string()].join(" ").as_bytes()).await;
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

    Ok(())
}