use tokio::task::spawn;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::fs::OpenOptions;
use std::collections::HashMap;
use chrono::Utc;

use tokio::net::TcpStream;

use tokio::sync::mpsc;

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

#[tokio::main]
pub async fn initiate(filtered_committee: HashMap<u32, String>, args: Vec<String>)
{  
    let mut file: std::fs::File = OpenOptions::new().append(true).open("output.log").unwrap();

    let mut sorted: Vec<(&u32, &String)> = filtered_committee.iter().collect();

    sorted.sort_by_key(|a| a.0);


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


    let file_path = "./nodes_information.txt";
    let nodes_file = File::open(file_path).unwrap();

    let reader = BufReader::new(nodes_file);

    let mut node_ips: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();

        let ip: Vec<&str> = line.split("-").collect();
        
        node_ips.push(ip[1].to_string()); 
        
        
    }

    let server_stream_vec: Vec<TcpStream> = Vec::new();

    let client_stream_vec: Vec<TcpStream> = Vec::new();

    if args[5]=="prod"
    {
        let nodes_ip_clone = node_ips.clone();

        let (server_tx, mut server_rx): (mpsc::Sender<TcpStream>, mpsc::Receiver<TcpStream>) =
            mpsc::channel(32);
        let (client_tx, mut client_rx): (mpsc::Sender<TcpStream>, mpsc::Receiver<TcpStream>) =
        mpsc::channel(32);

        // Spawning the server and client tasks
        let server_task = spawn(async move {
            for ip in nodes_ip_clone {
                let future = newserver::create_server(ip.clone(), initial_port, test_port);
                let result = future.await;
                let _ = server_tx.send(result).await;
                
            }
        });

        let client_task = spawn(async move {
            for ip in node_ips {
                let future = newclient::create_client(
                    [ip.clone(), initial_port.to_string()].join(":"),
                    [ip, test_port.to_string()].join(":"),
                );
                let result = future.await;
                let _ = client_tx.send(result).await;
            }
        });

        // Wait for the tasks to complete
        server_task.await.unwrap();
        client_task.await.unwrap();

        // Collect the results
        let mut server_stream_vec = Vec::new();
        while let Some(result) = server_rx.recv().await {
            server_stream_vec.push(result);
        }

        let mut client_stream_vec = Vec::new();
        while let Some(result) = client_rx.recv().await {
            client_stream_vec.push(result);
        }
    }
    

     println!("{:?}", server_stream_vec);
     println!("{:?}", client_stream_vec);

    let start_time = Utc::now().time();

    for _index in 1..(args[7].parse::<u32>().unwrap()+1) // iterate for all epoch
    {   
        println!("epoch {}", _index);

        let mut text;

        text = ["epoch ".to_string(), _index.to_string()].join(": ");
        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();

        let mut port_count: u32 = 0;
        
        if args[5]=="prod" // in prod mode
        {
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
                    println!("{:?}", ip_address);
                    reactor::reactor_init(_pvss_data.clone(),committee_id.clone(), ip_address.clone(), level, _index, args.clone(), port_count.clone(), "prod_init".to_string()).await;
                    level+=1;
                }

                
            }
                           
        }
        else 
        {           
            let pvss_data = ["pvss_data".to_string(), 999.to_string()].join(" ");     
            let mut ip_address: Vec<&str> = Vec::new();
            let address:&str = "127.0.0.1";
            ip_address.push(address);
            let level = 0;
            reactor::reactor_init(pvss_data.clone(), 999, ip_address.clone(), level, _index, args.clone(), port_count.clone(), "dev_init".to_string()).await;

        }

        text = "--------------------------------".to_string();

        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();


    }

    let end_time = Utc::now().time();

    let diff = end_time - start_time;
    
    println!("End by {}. time taken {} seconds", args[6], diff.num_seconds());
    
    

}