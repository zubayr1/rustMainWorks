use tokio::net::{TcpListener, TcpStream};
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::OpenOptions;
use chrono::Utc;
use tokio::sync::RwLock;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};


#[allow(unused)]
pub async fn create_server( _ip_address: String, port: u32, testport: u32) 
    -> HashMap<String, TcpStream>
{
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["0.0.0.0".to_string(), testport.to_string()].join(":")).await.unwrap();

    let (_, _) = test_listener.accept().await.unwrap();

    let mut connections: HashMap<String, TcpStream> = HashMap::new();

    let (mut socket, socket_addr) = listener.accept().await.unwrap(); // accept listening


    let socket_addr_string = socket_addr.to_string();
    let socket_ip: Vec<&str> = socket_addr_string.split(":").collect();

    let connection_key = socket_addr.to_string();


    connections.insert(socket_ip[0].clone().to_string(), socket);

    println!("client created");

    connections

}

#[allow(unused)]
pub async fn handle_server(connections_server: Arc<RwLock<HashMap<String, TcpStream>>>, _ip_address: String, port: u32, testport: u32) 
    -> String
{

    let start_time = Utc::now().time();

    // let mut connection_server_lock = connections_server.lock().unwrap();

    let read_lock = connections_server.read().await;

    let key_to_check = _ip_address.clone();
    
    
    if read_lock.contains_key(&key_to_check) 
    {
        println!("SERVER TcpStream exists for key: {}, {:?}", key_to_check, read_lock.get(&key_to_check));

        drop(read_lock);

        let mut write_lock = connections_server.write().await;

        // Bind the temporary value to a variable
        let socket = write_lock.get_mut(&key_to_check).unwrap();        // Use the as_mut method on the variable        
        
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

        
        if line.contains("EOF")  
        {
            line = line.replace("EOF", "");

          
            break;
        }

        line.clear();
                        
        }


        file.write_all(line.as_bytes()).await.unwrap();
        file.write_all(b"\n").await.unwrap();

        let socket_addr_string = _ip_address.to_string();

        line = [line.clone(), socket_addr_string.to_string()].join("/"); 

        

        let end_time = Utc::now().time();

        let _diff = end_time - start_time;   

        println!("{:?}", line);    

        return line;
    } 
    else 
    {
       println!("SERVER TcpStream does not exist for key: {}", key_to_check);
       return "na".to_string();
    }

    
}