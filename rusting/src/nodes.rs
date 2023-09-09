
use tokio::fs::File;
use tokio::io::{ AsyncBufReadExt, BufReader};
// use tokio::fs::OpenOptions;

use std::collections::HashMap;

use std::net::SocketAddr;

use crate::network::NetworkReceiver;
use crate::network::NetworkSender;

use tokio::sync::mpsc::channel;

use tokio::time::sleep;
use tokio::time::Duration;

use chrono::Utc;

#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

// #[path = "../probability/create_adv_prob.rs"]
// mod create_adv_prob;


#[path = "../consensus/reactor.rs"]
mod reactor;


#[path = "../algos/GRand.rs"]
mod GRand;

pub fn create_keys() // schnorr key generation
{
    schnorrkel::_create_keys_schnorrkel();

}



pub async fn initiate(filtered_committee: HashMap<u32, String>, args: Vec<String>)
{  
    // let mut file = OpenOptions::new().write(true).create(true).open("output.log").await.unwrap();

    println!("adversary? {:?}", args[8]);
    let mut sorted: Vec<(&u32, &String)> = filtered_committee.iter().collect();

    sorted.sort_by_key(|a| a.0);


    let file_path = "./nodes_information.txt";
    let nodes_file = File::open(file_path).await.unwrap();

    let reader = BufReader::new(nodes_file);


    let mut node_ips: Vec<String> = Vec::new();

    let mut line_stream = reader.lines();

    while let Some(line_result) = line_stream.next_line().await.unwrap() 
    {
        let line = line_result;

        let ip: Vec<&str> = line.split("-").collect();
        
        node_ips.push(ip[1].to_string()); 
              
    }

    

    let mut port: u32 = 7000;

    let mut sockets: Vec<SocketAddr> = Vec::new();

    for ip in &node_ips 
    {

        port+=args.clone()[2].parse::<u32>().unwrap();

        let ip_with_port = format!("{}:{}", ip, port.to_string()); 

        sockets.push(ip_with_port.parse::<SocketAddr>().unwrap());

        port = 7000;
    }

    // Get own node id from command line arguments.
    let node_id: usize = args[2].parse().unwrap();
    // Get own node ip from ip file. Note: not from command line, this is redundant and leads to errors.
    let node_ip = &sockets[node_id - 1];

    
    // Create channels for communication for receiver. Use rx_receiver to receive network messages.
    let (tx_receiver, rx_receiver) = channel(10_000);
    // Create channels for communication with sender. Use tx_sender to send messages to the network.
    let (tx_sender, rx_sender) = channel(10_000);

    

    // Create and start the sender and receiver.
    let network_receiver = NetworkReceiver::new(*node_ip, tx_receiver.clone());
    let mut network_sender = NetworkSender::new(rx_sender);


    tokio::spawn(async move {
        network_receiver.run().await;
    });
    tokio::spawn(async move {
        network_sender.run().await;
    });


    // Sleep to make sure sender and receiver are ready.
    sleep(Duration::from_millis(50)).await;


    let start_time = Utc::now().time(); 
    
    reactor::reactor(tx_sender, rx_receiver, sorted, args.clone()).await;
   
    
    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    
    println!("Setup End by {}. time taken {} miliseconds", args[6], diff.num_milliseconds());
    
    
    
}

