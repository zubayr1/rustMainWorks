//imports
use std::thread;
use std::fs::File;

use std::io::{ prelude::*, BufReader};
use futures::executor::block_on;

use std::env::{self};
use chrono::prelude::*;
use std::collections::HashMap;


//import own files
mod nodes;
// mod nodes_test;
mod nodes_test;

fn run_nodes(args: Vec<String>)
{

    let _file = File::create("output.log"); // to create output log file where all logs will be stored

    let mut ids: Vec<String> = Vec::new();
    let mut ip_address: Vec<String> = Vec::new();

    let mut committee: HashMap<u32, String> = HashMap::new();
    let mut filtered_committee: HashMap<u32, String> = HashMap::new();

    // get nodes information: with committees.
    
    let nodesfile = File::open("./updatednodeinfo.txt").expect("cant open the file"); // get all nodes information from nodes_information file
    let reader = BufReader::new(nodesfile);

        
    for line in reader.lines() 
    {
        let line_uw = line.unwrap();       
        
        let textsplit: Vec<&str> = line_uw.split("-").collect();      

        ids.push(textsplit[0].to_string());

        let committeesplit: Vec<&str> = textsplit[1].split(" ").collect();

        ip_address.push(committeesplit[0].to_string());

       
        for _i in 1..committeesplit.len() 
        {   
            let committee_id = committeesplit[_i];
            if !committee.contains_key(&committee_id.clone().parse::<u32>().unwrap())
            {
                committee.insert(committeesplit[_i].parse::<u32>().unwrap(), committeesplit[0].to_string());
            }   
            else 
            {   
                let participants = [committee[&committeesplit[_i].parse::<u32>().unwrap()].to_string(), committeesplit[0].to_string()].join(" ");
                committee.insert(committeesplit[_i].parse::<u32>().unwrap(),   participants);     
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
    
   
    if args[5]=="dev" // run in dev mode
    {
        let args_clone = args.clone();

        // since in dev mode, localhost runs as both client and server, need to use threading so that client and 
        // server runs concurrently

        let handle1 = thread::spawn(move || {  
        

            let future = nodes::initiate(filtered_committee.clone(), args_clone); //client

        
            block_on(future);
            
    
        });
        let args_clone_new = args.clone();

        let handle2 = thread::spawn(move || {
            
    
            let future1 = nodes_test::initiate(args_clone_new); //server

        
            block_on(future1);
            
    
        });
            
        
        handle1.join().unwrap();
            
        
        handle2.join().unwrap();
    } 
    else  // run in prod mode
    {
        let future = nodes::initiate(filtered_committee.clone(), args.clone()); 

        
        block_on(future);
    }
         


    
    
}


fn create_keys()
{
    nodes::create_keys();
    
}

fn main() 
{
    println!("Starting");    
    
    env::set_var("INITIAL_PORT", "7821");
    env::set_var("TEST_PORT", "7921");
    
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
