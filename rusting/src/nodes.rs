
use std::{thread, time};
// use std::collections::HashSet;
// use tokio::io::AsyncReadExt;
// use tokio::net::TcpStream;
// use tokio::net::TcpListener;
use std::error::Error;
use tokio::fs::{OpenOptions};
use tokio::io::AsyncWriteExt;

#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

#[path = "../probability/create_adv_prob.rs"]
mod create_adv_prob;

// #[path = "./client.rs"]
// mod client;

// #[path = "./server.rs"]
// mod server;


#[path = "./newclient.rs"]
mod newclient;

#[path = "./newserver.rs"]
mod newserver;

const INITIAL_PORT: u32 = 7081;

const TEST_PORT: u32 = 7481;

pub fn create_keys() // schnorr key generation
{

    schnorrkel::_create_keys_schnorrkel();

}

#[tokio::main]
pub async fn write_to_file(text: String, mut file: tokio::fs::File) -> tokio::fs::File
{
    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();
    return file;
}


pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{  
    // let  blacklisted = HashSet::new(); // create blacklisted list (should change in recursion)
    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let ip_address_clone = ip_address.clone();


    // let args_clone = args.clone();

    let self_ip = args[6].clone();


    let mut port_count: u32 = 0;


    // let  behavior = args[8].clone();


    for _index in 1..(args[7].parse::<i32>().unwrap()+1) // iterate for all epoch
    {   
        println!("epoch {}", _index);

        let mut text;

        text = ["epoch ".to_string(), _index.to_string()].join(": ");

        file = write_to_file(text, file);


        port_count+=1;
        if args[5]=="prod" // in prod mode
        {

           
                thread::scope(|s| { 


                    s.spawn(|| {

                        // let mut retry_ips = Vec::new();
                        let mut count=1;
                        for _ip in ip_address_clone.clone() 
                        {
                            count+=1;
                            let additional_port = (count + args[2].parse::<u32>().unwrap())*10;
                            
                            
                            let _result = newserver::handle_server( ip_address_clone.clone(), INITIAL_PORT+port_count, TEST_PORT+port_count + additional_port );

                            println!("------------------{}-----------------------", _result);

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

                            let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client([ip.to_string(), (INITIAL_PORT+port_count ).to_string()].join(":"),
                            [ip.to_string(), (TEST_PORT+port_count + additional_port).to_string()].join(":"), self_ip_clone, "first".to_string());

                            
                        }

                    });

                    
    
                    
                });

               
        }
        else 
        {                
        
            let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client(["127.0.0.1".to_string(), (INITIAL_PORT+port_count ).to_string()].join(":"),
                ["127.0.0.1".to_string(), (TEST_PORT+port_count ).to_string()].join(":"), "127.0.0.1".to_string(), "first".to_string());

        }

        text = "--------------------------------".to_string();

        file = write_to_file(text, file);

        println!("--------------------------------");

    }
    
    println!("End by {}", args[6]);
    
    

}