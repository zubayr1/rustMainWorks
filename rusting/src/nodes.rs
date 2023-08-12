
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::net::SocketAddr;
use chrono::Utc;
use futures::executor::block_on;
use tokio::sync::RwLock;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::spawn;
use tokio::sync::mpsc::channel;
use tokio::task;

use crate::{node, socketing::{*, self}};
use crate::message::NetworkMessage;
#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

// #[path = "../probability/create_adv_prob.rs"]
// mod create_adv_prob;

#[path ="../networking/newserver.rs"]
mod newserver;

#[path ="../networking/newclient.rs"]
mod newclient;

#[path = "../consensus/reactor.rs"]
mod reactor;

pub fn create_keys() // schnorr key generation
{
    schnorrkel::_create_keys_schnorrkel();

}


pub fn read_ports(file_name: String) -> Vec<u32>
{
    let file = File::open(file_name).expect("Failed to open the file");

    // Create a BufReader to efficiently read the file
    let reader = BufReader::new(file);

    // Initialize an empty vector to store the u32 port values
    let mut ports: Vec<u32> = Vec::new();

    // Read each line from the file and parse it into a u32, then push it into the vector
    for line in reader.lines() {
        if let Ok(num_str) = line {
            if let Ok(num) = num_str.trim().parse::<u32>() {
                ports.push(num);
            } else {
                println!("Invalid number: {}", num_str);
            }
        }
    }

    return ports;
}



pub async fn initiate(filtered_committee: HashMap<u32, String>, args: Vec<String>)
{  
    let mut file: std::fs::File = OpenOptions::new().append(true).open("output.log").unwrap();

    let mut sorted: Vec<(&u32, &String)> = filtered_committee.iter().collect();

    sorted.sort_by_key(|a| a.0);



    let file_path = "./nodes_information.txt";
    let nodes_file = File::open(file_path).unwrap();

    let reader = BufReader::new(nodes_file);

    let mut node_ips: Vec<String> = Vec::new();

    for line_result in reader.lines() 
    {
        let line = line_result.unwrap();

        let ip: Vec<&str> = line.split("-").collect();
        
        node_ips.push(ip[1].to_string());         
        
    }

   
    let server_port_list = read_ports("./server_port_list.txt".to_string());
    let client_port_list = read_ports("./client_port_list.txt".to_string());


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


    // create persistant connections
    let server_map: HashMap<String, tokio::net::TcpStream> = HashMap::new();
    let client_map: HashMap<String, tokio::net::TcpStream> = HashMap::new();


    // socketing::socket(server_map, client_map, server_port_list.clone(), client_port_list.clone(), initial_port, test_port, node_ips.clone());

    let mut server_initial_port: Vec<u32> = Vec::new();
    let mut server_test_port: Vec<u32> = Vec::new();


    let mut client_initial_port: Vec<u32> = Vec::new();
    let mut client_test_port: Vec<u32> = Vec::new();


    for i in server_port_list.clone()
    {
        server_initial_port.push(initial_port.clone() + i);
        server_test_port.push(test_port.clone() + i);
    }


    for i in client_port_list.clone()
    {
        client_initial_port.push(initial_port.clone() + i);
        client_test_port.push(test_port.clone() + i);
    }


    // let mut nodes: Vec<Node> = node_ips.into_iter().map(Node::new).collect();
   
    
    // let mut futures: Vec<_> = Vec::new();

    
    
    // for (count, node) in nodes.iter_mut().enumerate() 
    // {     
        
    //     let server_initial_port = server_initial_port.get(count).copied().unwrap_or_default();
    //     let server_test_port = server_test_port.get(count).copied().unwrap_or_default();
    //     let client_initial_port = client_initial_port.get(count).copied().unwrap_or_default();
    //     let client_test_port = client_test_port.get(count).copied().unwrap_or_default();
        

        
    //     let future = async move {
    //         node.create_sockets(server_initial_port, server_test_port,
    //             client_initial_port, client_test_port
    //         ).await;
            
    //     };
    //     futures.push(future);
    // }
    
    

    // // Wait for all the futures to complete
    // futures::future::join_all(futures).await;

    // // For each node, print the number of server and client sockets it has
    // for node in &nodes {
    //     println!("Node {} has {} server sockets and {} client sockets. socket: {:?}", 
    //         node.ip,
    //         node.get_server_sockets().read().await.len(),
    //         node.get_client_sockets().read().await.len(),
    //         node.get_server_sockets()
    //     );
    // }

    let mut sockets: Vec<SocketAddr> = Vec::new();


    for ip in  node_ips.clone()
    {
        sockets.push([ip, "7000".to_string()].join(":").parse::<SocketAddr>().unwrap());
    }  
    
    let handle = tokio::spawn(async move {
        node::Node::new(1, sockets).await;
    });
    let result = handle.await.unwrap();
    println!("fuck");

    for _index in 1..(args[7].parse::<u32>().unwrap()+1) // iterate for all epoch
    {   
        let start_time = Utc::now().time(); 

        println!("epoch {}", _index);

        let mut text;

        text = ["epoch ".to_string(), _index.to_string()].join(": ");
        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();

        let mut port_count: u32 = 0;
        
        
        let mut level = 0;

        let mut _pvss_data: String = "".to_string();

       
        for (committee_id, ip_addresses_comb) in sorted.clone()
        {
            let ip_address: Vec<&str> = ip_addresses_comb.split(" ").collect();   
            
            
            if ip_address.len()==1
            {
                //GET PVSS DATA FROM DIMITRIS
                _pvss_data = ["pvss_data".to_string(), args[2].to_string()].join(" ");
                level+=1
            }
            else 
            {                               
                port_count+=1; 

                
               
                reactor::reactor_init( 
                    _pvss_data.clone(),committee_id.clone(), ip_address.clone(), 
                level, _index, args.clone(), port_count.clone(), "prod_init".to_string()).await;
                level+=1;
            }

            
        }                          
        
        

        text = "--------------------------------".to_string();

        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();


        let end_time = Utc::now().time();

        let diff = end_time - start_time;
        
        println!("End by {}. time taken {} seconds", args[6], diff.num_seconds());


    }

    
    
}


pub async fn dev_initiate(filtered_committee: HashMap<u32, String>, args: Vec<String>)
{

    let mut file: std::fs::File = OpenOptions::new().append(true).open("output.log").unwrap();

    let mut sorted: Vec<(&u32, &String)> = filtered_committee.iter().collect();

    sorted.sort_by_key(|a| a.0);


    let start_time = Utc::now().time();

    for _index in 1..(args[7].parse::<u32>().unwrap()+1) // iterate for all epoch
    { 
        println!("epoch {}", _index);

        let text;

        text = ["epoch ".to_string(), _index.to_string()].join(": ");
        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();

        let port_count: u32 = 0;        

        let mut _pvss_data: String = "".to_string();

        let pvss_data = ["pvss_data".to_string(), 999.to_string()].join(" ");     
        let mut ip_address: Vec<&str> = Vec::new();
        let address:&str = "127.0.0.1";
        ip_address.push(address);
        let level = 0;

        let server_map: HashMap<String, tokio::net::TcpStream> = HashMap::new();
        let client_map: HashMap<String, tokio::net::TcpStream> = HashMap::new();

        let connections_server: Arc<RwLock<HashMap<String, TcpStream>>> = Arc::new(RwLock::new(server_map));
        let connections_client: Arc<RwLock<HashMap<String, TcpStream>>> = Arc::new(RwLock::new(client_map));
        
        // reactor::reactor_init(connections_server, connections_client, 
        //     pvss_data.clone(), 999, ip_address.clone(), level, 
        // _index, args.clone(), port_count.clone(), "dev_init".to_string()).await;
    }

    let end_time = Utc::now().time();

    let diff = end_time - start_time;
    
    println!("End by {}. time taken {} seconds", args[6], diff.num_seconds());
}