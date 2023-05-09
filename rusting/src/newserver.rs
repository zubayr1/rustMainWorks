use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
pub async fn handle_server( _ip_address: Vec<String>, port: u32, testport: u32) -> String{
   // loop{
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["0.0.0.0".to_string(), testport.to_string()].join(":")).await.unwrap();
    

    let (_, _) = test_listener.accept().await.unwrap();


    let (mut socket, _) = listener.accept().await.unwrap(); // accept listening

   
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
    
    
    return line;
    
//}
}