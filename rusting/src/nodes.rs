
use tokio::fs::File;
use tokio::io::{ AsyncBufReadExt, BufReader};
use tokio::fs::OpenOptions;

use std::collections::HashMap;

use std::net::SocketAddr;

use crate::network::NetworkReceiver;
use crate::network::NetworkSender;

use tokio::sync::mpsc::channel;

use tokio::time::sleep;
use tokio::time::Duration;

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


pub async fn _read_ports(file_name: String) -> Vec<u32>
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
    let (tx_receiver, mut rx_receiver) = channel(10_000);
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



    
    reactor::reactor(tx_sender, rx_receiver, sorted, args).await;
   
    
    

    // for _index in 1..(args[7].parse::<u32>().unwrap()+1) // iterate for all epoch
    // {   
    //     let start_time = Utc::now().time(); 

    //     println!("epoch {}", _index);

    //     let mut text;

    //     text = ["epoch ".to_string(), _index.to_string()].join(": ");
    //     file.write_all(text.as_bytes()).await.unwrap();
    //     file.write_all(b"\n").await.unwrap();

      
        
    //     let mut level = 0;

    //     let pvss_data_str: String = "".to_string();

    //     let mut pvss_data: Vec<u8> = pvss_data_str.into_bytes();

    //     for (committee_id, ip_addresses_comb) in sorted.clone()
    //     {`
    //         let ip_address: Vec<&str> = ip_addresses_comb.split(" ").collect(); 

            
            
    //         if ip_address.len()==1
    //         {
    //             //GET PVSS DATA FROM DIMITRIS
    //             pvss_data = ["pvss_datapvss_data".to_string(), args[2].to_string()].join(" ").into_bytes();
    //             level+=1
    //         }
    //         else 
    //         {                               
              
    //             pvss_data = reactor::reactor_init( 
    //                 pvss_data.clone(),committee_id.clone(), ip_address.clone(), 
    //             level, _index, args.clone()).await;
    //             level+=1;
                
    //             println!("{:?}", String::from_utf8(pvss_data.clone()));

    //         }
            
    //     }    

    //     let end_time = Utc::now().time();
    //     let diff = end_time - start_time;
        
    //     println!("Setup End by {}. time taken {} seconds", args[6], diff.num_seconds());  



    //     text = "--------------------------------".to_string();

    //     file.write_all(text.as_bytes()).await.unwrap();
    //     file.write_all(b"\n").await.unwrap();


        
    //     text = "GRand Start".to_string();

    //     file.write_all(text.as_bytes()).await.unwrap();
    //     file.write_all(b"\n").await.unwrap();

    //     let start_time = Utc::now().time(); 

    //     GRand::initiate(pvss_data);

    //     let end_time = Utc::now().time();
    //     let diff = end_time - start_time;
        
    //     println!("GRand End by {}. time taken {} seconds", args[6], diff.num_seconds()); 
        


    // }

    
    
}

