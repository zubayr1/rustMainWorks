
use std::io::Write;
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
    let port_count: u32 = 0;

    let mut file: std::fs::File = OpenOptions::new().append(true).open("output.log").unwrap();


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
            prod_communication(sorted.clone(), port_count, _index, args.clone(), "echo".to_string()).await;
               
        }
        else 
        {                
            dev_communication(["127.0.0.1".to_string(), (INITIAL_PORT + _index).to_string()].join(":"), 
                ["127.0.0.1".to_string(), (TEST_PORT + _index).to_string()].join(":"), "echo".to_string()).await;

        }

        text = "--------------------------------".to_string();

        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();


    }

    let end_time = Utc::now().time();

    let diff = end_time - start_time;
    
    println!("End by {}. time taken {} seconds", args[6], diff.num_seconds());
    
    

}