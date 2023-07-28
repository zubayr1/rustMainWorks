// use std::error::Error;
use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::OpenOptions;


#[tokio::main]
pub async fn handle_server(port: u32, testport: u32) -> String{

    
   // loop{
    let listener = TcpListener::bind(["127.0.0.1".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["127.0.0.1".to_string(), testport.to_string()].join(":")).await.unwrap();
    
    let (_, _) = test_listener.accept().await.unwrap();


    let (mut socket, _) = listener.accept().await.unwrap(); // accept listening

    socket.set_nodelay(true).expect("Failed to enable TCP_NODELAY");

    let (reader, _) = socket.split(); // tokio socket split to read and write concurrently
        
    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
    let mut line: String  = String :: new();

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let text;

    text = ["server at port".to_string(), port.to_string()].join(": ");

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();


    loop 
    {         
        let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();

        // if _bytes_read == 0 {
        //     // End of stream, the client has closed the connection.
        //     break;
        // }
    
        if line.contains("EOF")  
        {
            line = line.replace("EOF", "");                           
            
            break;
        }

        line.clear();
                
    }

    return line;
       
    
//}
}



pub async fn initiate( initial_port: u32, test_port: u32)
{

    let _result = handle_server(initial_port, test_port );

    
}