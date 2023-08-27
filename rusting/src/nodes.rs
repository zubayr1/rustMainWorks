
use tokio::fs::File;
use tokio::io::{ AsyncBufReadExt, BufReader};
use tokio::io::AsyncWriteExt;
use tokio::fs::OpenOptions;

use std::collections::HashMap;
use chrono::Utc;



use crate::{node, socketing::*};

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


#[path = "../algos/GRand.rs"]
mod GRand;

pub fn create_keys() // schnorr key generation
{
    schnorrkel::_create_keys_schnorrkel();

}


pub async fn read_ports(file_name: String) -> Vec<u32>
{
    let file = File::open(file_name).await.unwrap();

    // Create a BufReader to efficiently read the file
    let reader = BufReader::new(file);

    // Initialize an empty vector to store the u32 port values
    let mut ports: Vec<u32> = Vec::new();

    // Read each line from the file and parse it into a u32, then push it into the vector
    let mut line_stream = reader.lines();
    while let Some(line_result) = line_stream.next_line().await.unwrap() {
        let line = line_result;

        if let Ok(num) = line.parse::<u32>() {
            ports.push(num);
        } else {
            println!("Invalid u32 string");
        }
        
    }


    return ports;
}



pub async fn initiate(filtered_committee: HashMap<u32, String>, args: Vec<String>)
{  
    let mut file = OpenOptions::new().write(true).create(true).open("output.log").await.unwrap();

    println!("adversary? {:?}", args[8]);
    let mut sorted: Vec<(&u32, &String)> = filtered_committee.iter().collect();

    sorted.sort_by_key(|a| a.0);



    let file_path = "./nodes_information.txt";
    let nodes_file = File::open(file_path).await.unwrap();

    let reader = BufReader::new(nodes_file);


    let mut node_ips: Vec<String> = Vec::new();

    let mut line_stream = reader.lines();

    while let Some(line_result) = line_stream.next_line().await.unwrap() {
        let line = line_result;

        let ip: Vec<&str> = line.split("-").collect();
        
        node_ips.push(ip[1].to_string());         
    }


   
    let server_port_list = read_ports("./server_port_list.txt".to_string());
    let client_port_list = read_ports("./client_port_list.txt".to_string());


    // Use spawn to execute Node::new as an async task

    // let self_ip = args[6].clone();
    
    // tokio::spawn(async move 
    // {

    //     let mut sockets: Vec<SocketAddr> = Vec::new();

    //     for ip in  node_ips.clone()
    //     {
    //         sockets.push([ip, "7000".to_string()].join(":").parse::<SocketAddr>().unwrap());
    //     } 

    //     node::Node::new(1, sockets, self_ip).await;
    // });
    
    // println!("start core");

    

    for _index in 1..(args[7].parse::<u32>().unwrap()+1) // iterate for all epoch
    {   
        let start_time = Utc::now().time(); 

        println!("epoch {}", _index);

        let mut text;

        text = ["epoch ".to_string(), _index.to_string()].join(": ");
        file.write_all(text.as_bytes()).await.unwrap();
        file.write_all(b"\n").await.unwrap();

        let mut port_count: u32 = 0;
        
        
        let mut level = 0;

        let pvss_data_str: String = "".to_string();

        let mut pvss_data: Vec<u8> = pvss_data_str.into_bytes();

       
        for (committee_id, ip_addresses_comb) in sorted.clone()
        {
            let ip_address: Vec<&str> = ip_addresses_comb.split(" ").collect();   
            
            if ip_address.len()==1
            {
                //GET PVSS DATA FROM DIMITRIS
                pvss_data = ["pvss_datapvss_data".to_string(), args[2].to_string()].join(" ").into_bytes();
                level+=1
            }
            else 
            {                               
                port_count+=1; 
               
               
                pvss_data = reactor::reactor_init( 
                    pvss_data.clone(),committee_id.clone(), ip_address.clone(), 
                level, _index, args.clone(), port_count.clone()).await;
                level+=1;
                
                println!("{:?}", String::from_utf8(pvss_data.clone()));

            }
            
        }    

        let end_time = Utc::now().time();
        let diff = end_time - start_time;
        
        println!("Setup End by {}. time taken {} seconds", args[6], diff.num_seconds());  



        text = "--------------------------------".to_string();

        file.write_all(text.as_bytes()).await.unwrap();
        file.write_all(b"\n").await.unwrap();


        
        text = "GRand Start".to_string();

        file.write_all(text.as_bytes()).await.unwrap();
        file.write_all(b"\n").await.unwrap();

        let start_time = Utc::now().time(); 

        // GRand::initiate(_pvss_data);

        let end_time = Utc::now().time();
        let diff = end_time - start_time;
        
        println!("GRand End by {}. time taken {} seconds", args[6], diff.num_seconds()); 
        


    }

    
    
}

