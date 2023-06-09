// use std::error::Error;
use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::{OpenOptions};
use std::env;



#[tokio::main]
pub async fn handle_server(port: u32, testport: u32) -> String{

    
   // loop{
    let listener = TcpListener::bind(["127.0.0.1".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["127.0.0.1".to_string(), testport.to_string()].join(":")).await.unwrap();
    
    let (_, _) = test_listener.accept().await.unwrap();


    let (mut socket, _) = listener.accept().await.unwrap(); // accept listening

    println!("---continue---");


    let (reader, mut writer) = socket.split(); // tokio socket split to read and write concurrently
        
    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
    let mut line: String  = String :: new();

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let mut text;

    text = ["server at port".to_string(), port.to_string()].join(": ");

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    loop { 
        
        let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();

    
        if line.contains("EOF")  //REACTOR to be used here
        {
          

            writer.write_all(line.as_bytes()).await.unwrap();

            text = line.clone();

            file.write_all(text.as_bytes()).await.unwrap();
            file.write_all(b"\n").await.unwrap();
            

            break;
        }
        
        
    }

        
    return line;
       
    
//}
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