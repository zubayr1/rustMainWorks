use std::error::Error;
use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use std::time::{Duration, Instant};

#[tokio::main]
pub async fn handle_server( ip_address: Vec<String>, port: u32, testport: u32) -> String{
   // loop{
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["0.0.0.0".to_string(), testport.to_string()].join(":")).await.unwrap();

    
    let start = Instant::now();

    let (_, _) = test_listener.accept().await.unwrap();


    let (mut socket, addr) = listener.accept().await.unwrap(); // accept listening

    let duration = start.elapsed(); 

    


    println!("---continue---");


    let (reader, mut writer) = socket.split(); // tokio socket split to read and write concurrently
        
    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
    let mut line: String  = String :: new();

    loop { 
        
        let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();

    
        if line.contains("EOF")  //REACTOR to be used here
        {
          

            writer.write_all(line.as_bytes()).await.unwrap();
            

            break;
        }
        
        
    }
    
    


    // for ip in ip_address.clone() // Broadcast to everyone. 
    // {   

    //     // while TcpStream::connect(ip.clone()).await.is_err() //waiting for server to be active, if not random wait and retry
    //     // {
    //     //     sleep(Duration::from_millis(10)).await;
    //     // }
    //         let address=  [ip.to_string(), port.to_string()].join(":");
            
            
    //         let mut stream = TcpStream::connect(address.clone()).await.unwrap();             
          
            
    //         let broadcast_about_false_leader = [address.clone().to_string(), "EOF".to_string()].join(" ");
            

    //         let _result = stream.write(broadcast_about_false_leader.as_bytes()).await;

                
                                        
        
    // }
    return line;
    
//}
}