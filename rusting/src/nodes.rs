
use std::io::Write;
use std::{thread, time};
use std::error::Error;
use std::fs::OpenOptions;
use std::collections::HashMap;
use chrono::Utc;
#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

// #[path = "../probability/create_adv_prob.rs"]
// mod create_adv_prob;

// #[path = "./client.rs"]
// mod client;

// #[path = "./server.rs"]
// mod server;

#[path = "../networking/newclient.rs"]
mod newclient;

#[path = "../networking/newserver.rs"]
mod newserver;

const INITIAL_PORT: u32 = 7821;

const TEST_PORT: u32 = 7921;

pub fn create_keys() // schnorr key generation
{

    schnorrkel::_create_keys_schnorrkel();

}



pub async fn initiate(filtered_committee: HashMap<u32, String>, args: Vec<String>)
{  
    let mut file: std::fs::File = OpenOptions::new().append(true).open("output.log").unwrap();

    let self_ip = args[6].clone();

    let mut port_count: u32 = 0;


    // let  behavior = args[8].clone();

    let mut sorted: Vec<(&u32, &String)> = filtered_committee.iter().collect();

    sorted.sort_by_key(|a| a.0);

    let start_time = Utc::now().time();

    for _index in 1..(args[7].parse::<u32>().unwrap()+1) // iterate for all epoch
    {   
        println!("epoch {}", _index);

        let mut text;

        text = ["epoch ".to_string(), _index.to_string()].join(": ");
        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();

        
        if args[5]=="prod" // in prod mode
        {
            let mut level=0;
            for (_i, ip_addresses_comb) in sorted.clone()
            {
                port_count+=1;

                let ip_address: Vec<&str> = ip_addresses_comb.split(" ").collect();

                let ip_address_clone = ip_address.clone();
                
                text = ["Level ".to_string(), level.to_string()].join(": ");
                file.write_all(text.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
                level+=1;
                       
                thread::scope(|s| { 

                    s.spawn(|| {
                        
                        let mut count=1;
                        for _ip in ip_address_clone.clone() 
                        {
                            count+=1;
                            let additional_port = (count + args[2].parse::<u32>().unwrap())*10;
                            
                            let _result = newserver::handle_server( ip_address_clone.clone(), INITIAL_PORT+port_count, TEST_PORT+port_count + additional_port );
                        }
                        
                       
                    });

                                 
                    s.spawn(|| {
                        let three_millis = time::Duration::from_millis(3);
                        thread::sleep(three_millis);

                        let mut count=1;

                        for ip in ip_address_clone.clone() 
                        {
                            count+=1;
                            let additional_port = (count + args[2].parse::<u32>().unwrap())*10;
                            let self_ip_clone = self_ip.clone();
                            let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client([ip.to_string(), (INITIAL_PORT+port_count).to_string()].join(":"),
                            [ip.to_string(), (TEST_PORT+port_count + additional_port).to_string()].join(":"), self_ip_clone);

                            
                        }

                    });

                });
            }
               
        }
        else 
        {                
            let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client(["127.0.0.1".to_string(), (INITIAL_PORT + _index).to_string()].join(":"),
                ["127.0.0.1".to_string(), (TEST_PORT + _index).to_string()].join(":"), "127.0.0.1".to_string());

        }

        text = "--------------------------------".to_string();

        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();


    }

    let end_time = Utc::now().time();

    let diff = end_time - start_time;
    
    println!("End by {}. time taken {} seconds", args[6], diff.num_seconds());
    
    

}