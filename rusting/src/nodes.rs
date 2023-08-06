

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::Write;
use std::fs::OpenOptions;
use std::collections::HashMap;
use chrono::Utc;
use std::rc::Rc;
use tokio::net::TcpStream;

use std::thread;

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


pub async fn portifying(node_ips: Vec<String>, server_port_list: Vec<u32>, client_port_list: Vec<u32>, 
    initial_port: u32, test_port: u32) -> (Vec<TcpStream>, Vec<TcpStream>)
{
    let nodes_ip_clone = node_ips.clone();

    let mut server_stream_vec:Vec<TcpStream> = Vec::new();
    let mut client_stream_vec:Vec<TcpStream> = Vec::new();


    thread::scope(|s| { 

        s.spawn(|| {

            let mut count = 0;
            for ip in nodes_ip_clone {
            let additional_port = server_port_list[count];
            count+=1;
            let result = newserver::create_server(ip.clone(), initial_port
            + additional_port, test_port+ additional_port);
            server_stream_vec.push(result);
            
        }

        });

        s.spawn(|| {
            let mut count = 0;
            for ip in node_ips {
                let additional_port = client_port_list[count];
                count+=1;
                let result = newclient::create_client(ip.clone(), initial_port
                + additional_port, test_port+ additional_port);
                client_stream_vec.push(result);
            }
        });

    });

    
    
    return (server_stream_vec, client_stream_vec);
}



async fn port_testing(server_stream_vec_rc: &Vec<Rc<TcpStream>>, client_stream_vec_rc: &Vec<Rc<TcpStream>>, initial_port: u32) -> bool
{   

    let server_stream_slice = server_stream_vec_rc.as_slice();
    let server_stream_vec_rc = server_stream_slice.to_vec();

    let client_stream_slice = client_stream_vec_rc.as_slice();
    let client_stream_vec_rc = client_stream_slice.to_vec();

    for rc in &server_stream_vec_rc {
        println!("Strong count: {}", Rc::strong_count(rc));
    }

    let mut server_stream_vec: Vec<TcpStream> = server_stream_vec_rc
    .into_iter()
    .filter_map(|rc| Rc::try_unwrap(rc).ok())
    .collect();


    let mut client_stream_vec: Vec<TcpStream> = client_stream_vec_rc
    .into_iter()
    .filter_map(|rc| Rc::try_unwrap(rc).ok())
    .collect();
    println!("{:?}",client_stream_vec);


    // Split the server_stream_vec into individual streams
    let mut server_streams = Vec::new();
    while let Some(stream) = server_stream_vec.pop() {
        server_streams.push(stream);
    }
    // Split the client_stream_vec into individual streams
    let mut client_streams = Vec::new();
    while let Some(stream) = client_stream_vec.pop() {
        client_streams.push(stream);
    }

    let mut check = true;

   

    thread::scope(|s| { 

        s.spawn(|| {

            for server_stream in server_stream_vec {
                let line = newserver::test_server(server_stream, initial_port);
                if line=="".to_string()
                {
                    check = false;
                    
                }
            }

        });

        s.spawn(|| {
            for client_stream in client_stream_vec {
                newclient::test_client(client_stream, initial_port);
                
            }
        });

    });

    check
}


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

    for line_result in reader.lines() 
    {
        let line = line_result.unwrap();

        let ip: Vec<&str> = line.split("-").collect();
        
        node_ips.push(ip[1].to_string());         
        
    }

   
    let server_port_list = read_ports("./server_port_list.txt".to_string());
    let client_port_list = read_ports("./client_port_list.txt".to_string());
    
    let future = portifying(node_ips.clone(), server_port_list, client_port_list, initial_port, test_port);
    let (server_stream_vec, client_stream_vec) = future.await;

    let server_stream_vec_rc: Vec<Rc<TcpStream>> = server_stream_vec.into_iter()
    .map(Rc::new)
    .collect();

    let client_stream_vec_rc: Vec<Rc<TcpStream>> = client_stream_vec.into_iter()
    .map(Rc::new)
    .collect();

    // PORT TESTING START
  
    let future1 = port_testing(&server_stream_vec_rc, &client_stream_vec_rc, initial_port);
    let check = future1.await;
    println!("port testing: {}", check);

    // PORT TESTING DONE


    let start_time = Utc::now().time();   

    for _index in 1..(args[7].parse::<u32>().unwrap()+1) // iterate for all epoch
    {   
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
                let mut server_stream_vec_final_rc: Vec<Rc<TcpStream>> = Vec::new();
                let mut client_stream_vec_final_rc: Vec<Rc<TcpStream>> = Vec::new();
                
                port_count+=1;
                
                for element in &ip_address 
                {                    
                    match node_ips.clone().iter().position(|x| x == *element) {
                        Some(index) => 
                        {        
                            server_stream_vec_final_rc.push(server_stream_vec_rc[index].clone());
                            client_stream_vec_final_rc.push(client_stream_vec_rc[index].clone());
                        },
                        None => 
                        {
                            println!("Element {} not found in B", element)
                        },
                    }
                }
                // println!("{:?}", server_stream_vec_final_rc);
                let server_stream_vec_final: Vec<TcpStream> = server_stream_vec_final_rc
                .into_iter()
                    .filter_map(|rc| Rc::try_unwrap(rc).ok())
                    .collect();
                
                let client_stream_vec_final: Vec<TcpStream> = client_stream_vec_final_rc
                .into_iter()
                    .filter_map(|rc| Rc::try_unwrap(rc).ok())
                    .collect();



                reactor::reactor_init(server_stream_vec_final, client_stream_vec_final, 
                    _pvss_data.clone(),committee_id.clone(), ip_address.clone(), 
                level, _index, args.clone(), port_count.clone(), "prod_init".to_string()).await;
                level+=1;
            }

            
        }                          
        
        

        text = "--------------------------------".to_string();

        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();


    }

    let end_time = Utc::now().time();

    let diff = end_time - start_time;
    
    println!("End by {}. time taken {} seconds", args[6], diff.num_seconds());
    
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

        let server_stream_vec: Vec<TcpStream> = Vec::new();
        let client_stream_vec: Vec<TcpStream> = Vec::new();
        reactor::reactor_init(server_stream_vec, client_stream_vec,
            pvss_data.clone(), 999, ip_address.clone(), level, 
        _index, args.clone(), port_count.clone(), "dev_init".to_string()).await;
    }

    let end_time = Utc::now().time();

    let diff = end_time - start_time;
    
    println!("End by {}. time taken {} seconds", args[6], diff.num_seconds());
}