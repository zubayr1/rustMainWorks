

use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::OpenOptions;
use chrono::Utc;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};



#[allow(unused)]
#[tokio::main]
pub async fn handle_server( connections_server: Arc<Mutex<HashMap<String, TcpStream>>>, _ip_address: Vec<&str>, port: u32, testport: u32) 
    -> (Arc<Mutex<HashMap<String, TcpStream>>>, String){

    let start_time = Utc::now().time();
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["0.0.0.0".to_string(), testport.to_string()].join(":")).await.unwrap();

    let (_, _) = test_listener.accept().await.unwrap();

    // let connections: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));

    let (mut socket, socket_addr) = listener.accept().await.unwrap(); // accept listening

    let connections_server_clone = Arc::clone(&connections_server);
    
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


    file.write_all(line.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    let socket_addr_string = socket_addr.to_string();
    let socket_ip: Vec<&str> = socket_addr_string.split(":").collect();

    let connection_key = socket_addr.to_string();

    line = [line.clone(), socket_ip[0].to_string()].join("/"); 

    let serialized_data = serde_json::to_string(&line).unwrap();   

    let _message_size = serialized_data.len();

    let end_time = Utc::now().time();

    let _diff = end_time - start_time;
    
    // println!("time taken {} miliseconds",diff.num_milliseconds());

    connections_server_clone.lock().unwrap().insert("client_id".to_string(), socket);

    println!("{:?}", connections_server_clone);

    return (connections_server_clone, line);
    
}