// use std::error::Error;
use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::OpenOptions;
use std::env;

use data_encoding::BASE64;


#[tokio::main]
pub async fn handle_server(port: u32, testport: u32) -> String{

    let listener = TcpListener::bind(["127.0.0.1".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["127.0.0.1".to_string(), testport.to_string()].join(":")).await.unwrap();
    
    let (_, _) = test_listener.accept().await.unwrap();


    let (mut socket, _) = listener.accept().await.unwrap(); // accept listening

    println!("---continue---");
    socket.set_nodelay(true).expect("Failed to enable TCP_NODELAY");

    let (reader, _) = socket.split(); // tokio socket split to read and write concurrently
        
    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
    let mut line: String  = String :: new();

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let text;

    text = ["server at port".to_string(), port.to_string()].join(": ");

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    let mut _decoded_data = Vec::new(); // Declare decoded_data outside the loop

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

            // Decode the Base64 data back to binary format.
            _decoded_data = BASE64.decode(line.trim().as_bytes()).unwrap();                    
            
            break;
        }

        line.clear();
                
    }
    let decoded_string: String = String::from_utf8_lossy(&_decoded_data).to_string();

    return decoded_string;
       

}



pub async fn initiate(args: Vec<String>)
{
    let mut port_count = 0;

    let initial_port_str = env::var("INITIAL_PORT").unwrap_or_else(|_| {
        println!("INITIAL_PORT_STR is not set.");
        String::new()
    });
    let test_port_str = env::var("TEST_PORT").unwrap_or_else(|_| {
        println!("TEST_PORT_STR is not set.");
        String::new()
    });
   
    let initial_port: u32 = initial_port_str.parse().unwrap();
    let test_port: u32 = test_port_str.parse().unwrap();

    for _index in 1..(args[7].parse::<i32>().unwrap()+1)
    {
        port_count+=1;
        if args[2]<args[3]
        {            

            let _result = handle_server(initial_port+port_count, test_port+port_count );
            
           println!("------------------{}-----------------------", _result);

            
        }
        
    }
    
    
    
}