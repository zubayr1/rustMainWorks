
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tokio::spawn;
use futures::executor::block_on;


#[path ="../networking/newserver.rs"]
mod newserver;

#[path ="../networking/newclient.rs"]
mod newclient;


pub fn socket(server_map: HashMap<String, TcpStream>, client_map: HashMap<String, TcpStream>,
    server_port_list: Vec<u32>, client_port_list: Vec<u32>, initial_port: u32, test_port: u32,
    node_ips: Vec<String>

)
{
    let connections_server: Arc<RwLock<HashMap<String, TcpStream>>> = Arc::new(RwLock::new(server_map));
    let connections_client: Arc<RwLock<HashMap<String, TcpStream>>> = Arc::new(RwLock::new(client_map));


    let connections_server_clone = Arc::clone(&connections_server);
    let connections_client_clone = Arc::clone(&connections_client);

    let nodes_ip_clone = node_ips.clone();

    let handle_server_fut = async move {
        let mut count = 0;
        let mut additional_port;
        for ip in nodes_ip_clone.clone() {
            additional_port = server_port_list[count];
            let val = newserver::create_server(
                ip.to_string(),
                initial_port.clone() + additional_port + 5000,
                test_port.clone() + additional_port + 5000,
            ).await;
            count += 1;

            let mut write_lock = connections_server.write().await;

            for (key, value) in val {
                write_lock.insert(key, value);
            }
            drop(write_lock);
        }
    };
    
    let handle_client_fut = async move {
        let mut count = 0;
        for ip in node_ips.clone() {
            let additional_port = client_port_list[count];
            let val = newclient::create_client(
                [ip.to_string(), (initial_port + additional_port + 5000).to_string()].join(":"),
                [ip.to_string(), (test_port + additional_port + 5000).to_string()].join(":"),
            ).await;
            count += 1;

            let mut write_lock = connections_client.write().await;
            for (key, value) in val {
                write_lock.insert(key, value);
            }
            drop(write_lock);
        }
    };

    
    
    let fut = async {
        let handle_server_task = spawn(handle_server_fut);
        let handle_client_task = spawn(handle_client_fut);
    
        let (_, _) = tokio::join!(handle_server_task, handle_client_task);
    };
    block_on(fut);
    
    println!("{:?}", connections_client_clone);


}