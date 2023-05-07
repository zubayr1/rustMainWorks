use std::error::Error;
use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{ sleep, Duration};

#[tokio::main]
pub async fn handle_server( ip_address: Vec<String>, port: u32) -> Result<(), Box<dyn Error>>{

   // loop{
    let std_listener = std::net::TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":"))?; // open connection
    
    println!("server listening at port {}", port);

    std_listener.set_nonblocking(true)?;
    let listener = TcpListener::from_std(std_listener)?;

    match listener.accept().await
    {
        Ok((mut socket, addr)) =>
        {
            println!("---continue---{}, {:?}", addr, ip_address);


            let (reader, mut writer) = socket.split(); // tokio socket split to read and write concurrently
                
            let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
            let mut line: String  = String :: new();
        
            loop { //loop to get all the data from client until EOF is reached
        
             
                let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();
        
                if _bytes_read == 0
                {
                    println!("0 bytes from {}", addr.clone());
                    writer.write_all(b"NAK").await.unwrap();
                    break;
                }
                
                
                if line.contains("EOF")  //REACTOR to be used here
                {
                    println!("EOF Reached");
                  
        
                    writer.write_all(line.as_bytes()).await.unwrap();
                    println!("{}", line);
                
        
                    line.clear();
        
                    break;
                }
                
                
            }
        },
        Err(e) => println!("couldn't get client: {:?}", e),

    }


    

//}
    Ok(())
}