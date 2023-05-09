// use std::error::Error;
use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::net::TcpStream;

const INITIAL_PORT: u32 = 7081;

const TEST_PORT: u32 = 7481;

#[tokio::main]
pub async fn handle_server( _: Vec<String>, port: u32, testport: u32) -> String{
   // loop{
    let listener = TcpListener::bind(["127.0.0.1".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["127.0.0.1".to_string(), testport.to_string()].join(":")).await.unwrap();
    

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



pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{
    let ip_address_clone = ip_address.clone();

    let mut port_count = 0;

    for _index in 1..(args[7].parse::<i32>().unwrap()+1)
    {
        port_count+=1;
        if args[2]<args[3]
        {            

            let _result = handle_server( ip_address_clone.clone(), INITIAL_PORT+port_count, TEST_PORT+port_count );
            
            println!("------------------{}-----------------------", _result);

            
        }
        
    }
    
    
    
}