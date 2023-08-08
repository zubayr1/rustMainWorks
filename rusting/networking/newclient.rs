use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

use tokio::time::{ sleep, Duration};
use tokio::fs::OpenOptions;
use std::sync::{Arc, Mutex};

use std::collections::HashMap;




#[allow(unused)]
#[tokio::main]
pub async fn match_tcp_client(connections_client: Arc<Mutex<HashMap<String, TcpStream>>>, address: String, test_address: String, committee_id:u32, value: Vec<String>, args: Vec<String>) 
-> Arc<Mutex<HashMap<String, TcpStream>>>
 {

    // let mut connections_client: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut connections: HashMap<String, TcpStream> = HashMap::new();
  
    let address_clone = address.clone();

    let parts: Vec<&str> = address_clone.split(':').collect();

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();
    // Connect to a peer    

    
    let connections_client_clone = Arc::clone(&connections_client);

    let mut connection_client_lock = connections_client_clone.lock().unwrap();

    let key_to_check = parts[0].clone().to_string();
    let is_present = {
        
        connection_client_lock.contains_key(&key_to_check)
    };

    
    if is_present {
      println!("TcpStream exists for key: {}, {:?}", key_to_check, connection_client_lock.get(&key_to_check));
    } else {
      println!("TcpStream does not exist for key: {}", key_to_check);
    }

    
    while TcpStream::connect(test_address.clone()).await.is_err() //waiting for client to be active, if not random wait and retry
    {               
        sleep(Duration::from_millis(1)).await;        
    }    
       
    let mut stream: TcpStream = TcpStream::connect(address.clone()).await.unwrap();  

    
    connections.insert(parts[0].clone().to_string(), stream);  

    let value_string = value.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", ");

    let temp_string = [value_string.to_string(), committee_id.to_string().clone()].join(", ");

    let final_string = [temp_string.to_string(), args[2].to_string().clone()].join(", ");

    
    loop
    {
        // Write data.           

        connections.get_mut(parts[0].clone()).unwrap().write_all(final_string.as_bytes()).await.unwrap();
         let result = connections.get_mut(parts[0].clone()).unwrap().write_all(b"EOF").await;

        if  result.is_ok()
        {
            break;
        }             

    }

    let text = ["client at: ".to_string(), args[6].to_string()].join(": ");
    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    // println!("{:?}", connections_client_lock.get_mut("client").unwrap());

    let connections_client: Arc<Mutex<HashMap<String, TcpStream>>> = Arc::new(Mutex::new(connections));

    println!("client {:?}", connections_client);

    return connections_client;
   
    
}


