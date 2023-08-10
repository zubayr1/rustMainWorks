use std::fs::OpenOptions;
use std::io::Write;
use std::env;
use futures::executor::block_on;
use std::thread;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::net::TcpStream;
use tokio::spawn;
use std::time;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[path = "./newclient.rs"]
mod newclient;

#[path ="./newserver.rs"]
mod newserver;

#[path = "./nested_nodes_test.rs"]
mod nested_nodes_test;

#[path = "../types/codeword.rs"]
mod codeword;


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

pub async fn prod_communication<'a>(connections_server: Arc<RwLock<HashMap<String, TcpStream>>>,
    connections_client: Arc<RwLock<HashMap<String, TcpStream>>>,
    committee_id: u32, ip_address: Vec<&'a str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, value: Vec<String>, mode: String, types: String) -> Vec<String>
{



    let mut client_count = 1;

    // if mode.contains("codeword")
    // {
    //     let file_path = "./nodes_information.txt";
    //     let file = File::open(file_path).unwrap();
    
    //     let reader = BufReader::new(file);
    
    
    //     for line_result in reader.lines() {
    //         let line = line_result.unwrap();
            
    //         if line.contains(ip_address[0].clone())
    //         {
    //             break;
    //         }
    //         client_count+=1;
    //     }
    // }
    


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

    let mut file: std::fs::File = OpenOptions::new().append(true).open("output.log").unwrap();

    let mut text;

    let output: Vec<String> = Vec::new();


    let server_port_list = read_ports("./server_port_list.txt".to_string());
    let client_port_list = read_ports("./client_port_list.txt".to_string());
    

    text = ["epoch ".to_string(), _index.to_string()].join(": ");
    file.write_all(text.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
    
        

    let ip_address_clone: Vec<String> = ip_address.iter().map(|&s| s.to_string()).collect();
    let ip_address_clone1: Vec<String> = ip_address.iter().map(|&s| s.to_string()).collect();
    
    text = ["Level ".to_string(), level.to_string()].join(": ");
    file.write_all(text.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    let mut outputclone = output.clone();

    let handle_server_fut = async move {
        let mut count = 0;
        let mut additional_port;
       
        for ip in ip_address_clone.clone() 
            { 
                
                let connections_server_clone = connections_server.clone();
                
                additional_port = server_port_list[count];

                let connections_server_clone1 = connections_server_clone.clone();

                // Drop the original MutexGuard
                drop(connections_server_clone);

                let val = newserver::handle_server(connections_server_clone1.clone(), ip.to_string(), 
                initial_port.clone() + additional_port + 5000
                , test_port.clone() + additional_port + 5000).await;
                
                count+=1;
                outputclone.push(val);
                
            }
    };
    
    let handle_client_fut = async move {
        let mut count = 0;
        for ip in ip_address_clone1.clone() 
            { 
                let connections_client_clone = connections_client.clone();

                let additional_port = client_port_list[count];

                let connections_client_clone1 = connections_client_clone.clone();

                // Drop the original MutexGuard
                drop(connections_client_clone);

                 newclient::match_tcp_client(connections_client_clone1.clone(),
                    [ip.to_string(), (initial_port+ additional_port + 5000).to_string()].join(":"), 
                [ip.to_string(), (test_port+ additional_port + 5000).to_string()].join(":"), committee_id.clone(), value.clone(), 
                args.clone()).await;

                count+=1;
                
            }
    };

    
    
    let fut = async {
        let handle_server_task = spawn(handle_server_fut);
        let handle_client_task = spawn(handle_client_fut);
    
        let (_, _) = tokio::join!(handle_server_task, handle_client_task);
    };
    block_on(fut);

    
    // thread::scope(|s| { 

    //     s.spawn(|| 
    //     {
            
    //         let mut count=1;


    //         if types.contains("individual")
    //         {
                            
    //             let additional_port = (client_count)*10;

    //             let  _result = newserver::handle_server( connections_server.clone(), ip_address_clone[0].to_string(), initial_port+port_count, 
    //                     test_port+port_count + additional_port);
                
               
    //             let witness_verify =  codeword::verify_codeword(_result.clone());
    
    //             if witness_verify==true
    //             {
    //                 output.push(_result);
    //             }
                    

    //         }
    //         else
    //         {
    //             for _ip in ip_address_clone.clone() 
    //             {   
    //                 count+=1;
    //                 let mut additional_port = (count + args[2].parse::<u32>().unwrap())*5;

    //                 if mode=="codeword"
    //                 {
    //                     additional_port = (count + args[2].parse::<u32>().unwrap())*50;

    //                     let _result
    //                      = newserver::handle_server(connections_server.clone(), _ip.clone().to_string(), initial_port+port_count, 
    //                     test_port+port_count + additional_port);
                        
    //                     output.push(_result);

    //                 }
    //                 else if mode=="accum"
    //                 {
    //                     let  _result = newserver::handle_server( connections_server.clone(), _ip.clone().to_string(), initial_port+port_count, 
    //                     test_port+port_count + additional_port);

                        

    //                     let socket_vec: Vec<&str> = _result.split("/").collect();
    //                     let socket_ip = socket_vec[1];

    //                     let file_path = "./updatednodeinfo.txt";
    //                     let file = File::open(file_path).unwrap();
                    
    //                     let reader = BufReader::new(file);
                    
    //                     let mut result: String =  "".to_string();
    //                     for line_result in reader.lines() 
    //                     {
    //                         let line = line_result.unwrap();
                            
    //                         if line.contains(socket_ip)
    //                         {
    //                             let nodes_string: Vec<&str> = line.split(" ").collect();

    //                             let level_usize: usize = level as usize;

    //                             result = [_result.clone(), nodes_string.get(level_usize+1).unwrap().to_string()].join(", ");
    //                             break;
    //                         }
    //                     }

    //                     output.push(result);
    //                 }


    //             }
    //         }
            
            
            
    //     });

                        
    //     s.spawn(|| {
    //         let three_millis = time::Duration::from_millis(3);
    //         thread::sleep(three_millis);

    //         let mut count=1;

            
    //         if types.contains("individual")
    //         {
                
    //             let additional_port = (args[2].parse::<u32>().unwrap())*10;

                
    //             newclient::match_tcp_client( connections_client.clone(), [ip_address_clone[0].to_string(), (initial_port+port_count).to_string()].join(":"),
    //             [ip_address_clone[0].to_string(), (test_port+port_count + additional_port).to_string()].join(":"), 
    //             committee_id.clone(), value.clone(), args.clone());

    //             // for (key, value) in connections_client {
                            
    //             //     client_map.insert(key, value);
    //             // }
                
    //         }
    //         else 
    //         {
    //             for ip in ip_address_clone.clone() 
    //             {   
    //                 count+=1;
    //                 let mut additional_port = (count + args[2].parse::<u32>().unwrap())*5;

    //                 if mode=="codeword"
    //                 {   additional_port = (count + args[2].parse::<u32>().unwrap())*50;

    //                 }

    //                 newclient::match_tcp_client( connections_client.clone(), [ip.to_string(), (initial_port+port_count).to_string()].join(":"),
    //                 [ip.to_string(), (test_port+port_count + additional_port).to_string()].join(":"), 
    //                 committee_id.clone(), value.clone(), args.clone());

    //                 // for (key, value) in connections_client {
                            
    //                 //     // client_map.insert(key, value);
    //                 //     println!("{:?}, {:?}", key, value);
    //                 //     client_map.insert(key, value);
    //                 // }
                    
    //             }
    //         }

            

    //     });

    // });

    println!("{:?}", output);
    return output;

}


pub async fn dev_communication(connections_client: Arc<RwLock<HashMap<String, TcpStream>>>, committee_id: u32, working_port: String, test_port: String, mut value: Vec<String>, args: Vec<String>) -> Vec<String>
{    
    let _connections_client = newclient::match_tcp_client(connections_client.clone(), working_port, test_port, committee_id.clone(), value.clone(), args.clone());
    
        
    value.push(committee_id.to_string());

    let joined_string = value.join(", ");    

    let mut returnvec: Vec<String> = Vec::new();
    returnvec.push(joined_string);

    return returnvec;
}






pub async fn nested_dev_communication(connections_client: Arc<RwLock<HashMap<String, TcpStream>>>, committee_id: u32, working_port: String, test_port: String, mut value: Vec<String>, args: Vec<String>) -> Vec<String>
{    
    

    value.push(committee_id.to_string());


    let initial_port_server: u32 = working_port.parse().unwrap();
    let test_port_server: u32 = test_port.parse().unwrap();

    let initial_port_client: u32 = working_port.parse().unwrap();
    let test_port_client: u32 = test_port.parse().unwrap();



    thread::scope(|s| { 

        s.spawn(|| 
        {
            
            let future = nested_nodes_test::initiate( 
            initial_port_server + 500, test_port_server + 500);

            block_on(future);
            
            
        });

                        
        s.spawn(|| {
            let three_millis = time::Duration::from_millis(3);
            thread::sleep(three_millis);

            let _connections_client = newclient::match_tcp_client(connections_client.clone(), 
                ["127.0.0.1".to_string(), (initial_port_client + 500).to_string()].join(":"),
                ["127.0.0.1".to_string(), (test_port_client + 500).to_string()].join(":"),
                committee_id.clone(), value.clone(), args.clone());

        });

    });
    
    return value;
}
