
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::net::SocketAddr;
use chrono::Utc;
use tokio::sync::RwLock;
use std::env;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;


use crate::{node, socketing::{*, self}};

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



    let mut sockets: Vec<SocketAddr> = Vec::new();


    for ip in  node_ips.clone()
    {
        sockets.push([ip, "7000".to_string()].join(":").parse::<SocketAddr>().unwrap());
    }  
    
    println!("Before calling Node::new");
    
    // Use spawn to execute Node::new as an async task
    tokio::spawn(async move {
        node::Node::new(1, sockets).await;
    });
    
    println!("After calling Node::new");


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