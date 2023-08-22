//imports
use tokio::fs::File;

use tokio::io::{ AsyncBufReadExt, BufReader};

use std::env::{self};
use chrono::prelude::*;
use std::collections::HashMap;

mod socketing;

mod core;
mod message;
mod network;
mod node;

mod nodes;

#[tokio::main]
async fn run_nodes(args: Vec<String>)
{

    let _file = File::create("output.log"); // to create output log file where all logs will be stored

    let mut ids: Vec<String> = Vec::new();
    let mut ip_address: Vec<String> = Vec::new();

    let mut committee: HashMap<u32, String> = HashMap::new();
    let mut filtered_committee: HashMap<u32, String> = HashMap::new();

    // get nodes information: with committees.
    
    let nodesfile = File::open("./updatednodeinfo.txt").await.unwrap(); // get all nodes information from nodes_information file

    let reader = BufReader::new(nodesfile);

    let mut line_stream = reader.lines();
    while let Some(line_result) = line_stream.next_line().await.unwrap() {
        let line = line_result;

        let line_uw = line;      
        
        let textsplit: Vec<&str> = line_uw.split("-").collect();      

        ids.push(textsplit[0].to_string());

        let committeesplit: Vec<&str> = textsplit[1].split(" ").collect();

        ip_address.push(committeesplit[0].to_string());

       
        for _i in 1..committeesplit.len() 
        {   
            let committee_id = committeesplit[_i].clone();
            let modified_committee_id = committee_id.replace("l", "").replace("r", "");

            if !committee.contains_key(&modified_committee_id.clone().parse::<u32>().unwrap())
            {
                committee.insert(committeesplit[_i].replace("l", "").replace("r", "").parse::<u32>().unwrap(), committeesplit[0].to_string());
            }   
            else 
            {   
                let participants = [committee[&committeesplit[_i].replace("l", "").replace("r", "").parse::<u32>().unwrap()].to_string(), committeesplit[0].to_string()].join(" ");
                committee.insert(committeesplit[_i].replace("l", "").replace("r", "").parse::<u32>().unwrap(),   participants);     
            }     
            
        }         
    }
        
   

    // filter committees: based on self ip being exist
    let self_ip = args[6].to_string();

    
    
    for (i, j) in committee
    {
        if j.contains(&self_ip.to_string())
        {
            filtered_committee.insert(i, j.to_string());
        }
    }
    
       
    nodes::initiate(filtered_committee.clone(), args.clone()).await; 


    
    
}


fn create_keys()
{
    nodes::create_keys();
    
}

fn main() 
{
    println!("Starting");    
    
    env::set_var("INITIAL_PORT", "7221");
    env::set_var("TEST_PORT", "7421");
    
    let args: Vec<String> = env::args().collect(); // get user argument
    /*
        user argument format:
        cargo run -- keys 1 4 03282129 dev 18.117.92.19 10 1
        cargo run -- nok 1 4 03282129 dev 18.117.92.19 10 1

        1. keys/ nok: for keys -> schorr signature generation. For nok -> run the nodes (considering signature is already generated).

        2. 1: nodes ids (1, ..., 128)

        3. 4: number of nodes (EC2 instances running). Finally it will be 128.

        4. 03282129: A future time in UTC. Nodes will be in waiting state until this time is reached. Then they all will start running. 
        We will not need it when we have a bash script that concurrently runs all the nodes.

        5. dev/ prod: Whether we are running nodes in localhost or with EC2 instances.

        6. 18.117.92.19: ip address of current node. 

        7. 10: number of epochs to be run

        8. 1/0: 1-> the node can act as adversary. 0-> node is honest.
    */

    let keys: &str = "keys";

    println!("execution type");

    println!("{}", args[1]);
        

    loop // implement waiting time for nodes
    {
        let utc: DateTime<Utc>  = Utc::now(); 
        // make arg time
        let month = &args[4][0..2].to_string();
        let date = &args[4][2..4].to_string();
        let hour = &args[4][4..6].to_string();
        let min = &args[4][6..8].to_string();
        
        if utc >= Utc.with_ymd_and_hms(2023, month.parse::<u32>().unwrap(), 
        date.parse::<u32>().unwrap(), hour.parse::<u32>().unwrap(), min.parse::<u32>().unwrap(), 00).unwrap()
        {
            break;
        }
    }

    println!("launched");
    
    if args[1].trim() == keys
    {
        create_keys(); // to create schnorr keys
    }
    else 
    {
        run_nodes(args.clone()); // to run the nodes
    }



    
    
}
