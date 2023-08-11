
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tokio::spawn;
use futures::executor::block_on;


#[path ="../networking/newserver.rs"]
mod newserver;

#[path ="../networking/newclient.rs"]
mod newclient;



// A struct that represents a node in your network
pub struct Node {
    // The IP address of the node
    pub ip: String,
    // A map of server sockets for the node
    connections_server: Arc<RwLock<HashMap<String, TcpStream>>>,
    // A map of client sockets for the node
    connections_client: Arc<RwLock<HashMap<String, TcpStream>>>,
}

impl Node {
    // A method that creates a new node with the given IP address
    pub fn new(ip: String) -> Self {
        Node {
            ip,
            connections_server: Arc::new(RwLock::new(HashMap::new())),
            connections_client: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_sockets(&mut self, initial_port_server: u32, test_port_server: u32,
        initial_port_client: u32, test_port_client: u32
    )
    {
        println!("{:?}, {:?}, {:?}, {:?}",initial_port_server, test_port_server, initial_port_client, test_port_client);
        let connections_server_clone = Arc::clone(&self.connections_server);
        let nodes_ip_clone = self.ip.clone();

        let handle_server_fut = async move {
            let val = newserver::create_server(
                nodes_ip_clone.to_string(),
                initial_port_server.clone() + 5000,
                test_port_server.clone() + 5000,
            ).await;
            let mut write_lock = connections_server_clone.write().await;

            for (key, value) in val {
                println!("{:?}", value);
                write_lock.insert(key, value);
                
            }
            drop(write_lock);
        };
        println!("fut");

        let connections_client_clone = Arc::clone(&self.connections_client);
        let nodes_ip_clone = self.ip.clone();

        let handle_client_fut = async move {
            let val = newclient::create_client(
                [nodes_ip_clone.to_string(), (initial_port_client + 5000).to_string()].join(":"),
                [nodes_ip_clone.to_string(), (test_port_client + 5000).to_string()].join(":"),
            ).await;

            let mut write_lock = connections_client_clone.write().await;
            for (key, value) in val {
                write_lock.insert(key, value);
            }
            drop(write_lock);
        };


        let handle_server_task = spawn(handle_server_fut);

        handle_server_task.await.unwrap();

        let handle_client_task = spawn(handle_client_fut);

        handle_client_task.await.unwrap();

    }

    // A method that creates and stores the server sockets for the node
    pub async fn create_server_sockets(&mut self, initial_port: u32, test_port: u32) {
        println!("nodes server{:?}, {:?}", initial_port, test_port);        

        let connections_server_clone = Arc::clone(&self.connections_server);
        let nodes_ip_clone = self.ip.clone();

        let handle_server_fut = async move {
            let val = newserver::create_server(
                nodes_ip_clone.to_string(),
                initial_port.clone() + 5000,
                test_port.clone() + 5000,
            ).await;
            let mut write_lock = connections_server_clone.write().await;

            for (key, value) in val {
                println!("{:?}", value);
                write_lock.insert(key, value);
                
            }
            drop(write_lock);
        };

        let handle_server_task = spawn(handle_server_fut);

        handle_server_task.await.unwrap();
    }

    // A method that creates and stores the client sockets for the node
    pub async fn create_client_sockets(&mut self, initial_port: u32, test_port: u32) {
        println!("nodes client{:?}, {:?}", initial_port, test_port);
        let connections_client_clone = Arc::clone(&self.connections_client);
        let nodes_ip_clone = self.ip.clone();

        let handle_client_fut = async move {
            let val = newclient::create_client(
                [nodes_ip_clone.to_string(), (initial_port + 5000).to_string()].join(":"),
                [nodes_ip_clone.to_string(), (test_port + 5000).to_string()].join(":"),
            ).await;

            let mut write_lock = connections_client_clone.write().await;
            for (key, value) in val {
                write_lock.insert(key, value);
            }
            drop(write_lock);
        };

        let handle_client_task = spawn(handle_client_fut);

        handle_client_task.await.unwrap();
    }

    // A method that returns a reference to the server sockets for the node
    pub fn get_server_sockets(&self) -> &Arc<RwLock<HashMap<String, TcpStream>>> {
        &self.connections_server
    }

    // A method that returns a reference to the client sockets for the node
    pub fn get_client_sockets(&self) -> &Arc<RwLock<HashMap<String, TcpStream>>> {
        &self.connections_client
    }
}




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
            println!("under socketing {:?}, {:?}", initial_port.clone() + additional_port, test_port.clone() + additional_port);
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