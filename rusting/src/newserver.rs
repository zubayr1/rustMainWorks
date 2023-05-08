use std::error::Error;
use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::{ sleep, Duration};
use tokio::net::TcpStream;

#[tokio::main]
pub async fn handle_server( ip_address: Vec<String>, port: u32) -> Result<(), Box<dyn Error>>{

   // loop{
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    

    let (mut socket, addr) = listener.accept().await.unwrap(); // accept listening

    println!("---continue---{}", addr);


    let (reader, mut writer) = socket.split(); // tokio socket split to read and write concurrently
        
    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
    let mut line: String  = String :: new();

    loop { //loop to get all the data from client until EOF is reached

     
        let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();

    //    if _bytes_read==0
    //    {       
    //    // writer.write_all(b"test").await.unwrap();
    //     return addr.ip().to_string();
    //    }
        
        if line.contains("EOF")  //REACTOR to be used here
        {
            println!("EOF Reached");
          

            writer.write_all(line.as_bytes()).await.unwrap();
            println!("{}", line);
        

            line.clear();

           // return "NA".to_string();
        }
        
        
    }
   


    // for ip in ip_address.clone() // Broadcast to everyone. 
    // {   

    //     // while TcpStream::connect(ip.clone()).await.is_err() //waiting for server to be active, if not random wait and retry
    //     // {
    //     //     sleep(Duration::from_millis(10)).await;
    //     // }
    //     println!("aaa");
    //         let address=  [ip.to_string(), port.to_string()].join(":");
            
            
    //         let mut stream = TcpStream::connect(address.clone()).await?;             
          
            
    //         let broadcast_about_false_leader = [address.clone().to_string(), "EOF".to_string()].join(" ");
            

    //         let _result = stream.write(broadcast_about_false_leader.as_bytes()).await;

                
                                        
        
    // }

    
//}
     Ok(())
}