use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;

#[path = "./gba.rs"]
mod gba; 

#[allow(non_snake_case)]
pub async fn byzar(connections_server: Arc<Mutex<HashMap<String, TcpStream>>>, 
    connections_client: Arc<Mutex<HashMap<String, TcpStream>>>, 
    committee_id: u32, ip_address: &Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, V: String, medium: String, mode: String, types: String, committee_length: usize) -> String
{   
    let (V, _g) = gba::gba(connections_server, connections_client.clone(), committee_id, ip_address.to_vec(), level, port_count, _index, args.clone(),
    V.clone(), medium, mode.clone(), types, committee_length).await;

    //println!("{:?}, {:?}", V, _g);

    return V;
    
}