use std::net::SocketAddr;

use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::{OpenOptions};



#[tokio::main]
pub async fn handle_server( _ip_address: Vec<&str>, port: u32, testport: u32) -> String{
   // loop{
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["0.0.0.0".to_string(), testport.to_string()].join(":")).await.unwrap();

    let (_, _) = test_listener.accept().await.unwrap();


    let (mut socket, socket_addr) = listener.accept().await.unwrap(); // accept listening

    let (reader, mut writer) = socket.split(); // tokio socket split to read and write concurrently
        
    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
    let mut line: String  = String :: new();

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let mut text;

    text = ["server at port".to_string(), port.to_string()].join(": ");

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    loop 
    { 
        
        let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();
    
        if line.contains("EOF")  
        {
          
            writer.write_all(line.as_bytes()).await.unwrap();

            text = line.clone();

            file.write_all(text.as_bytes()).await.unwrap();
            file.write_all(b"\n").await.unwrap();
            

            break;
        }
        
        
    }
    line = [line, socket_addr.to_string()].join(", ");    
    return line;
    
//}
}