
use std::{thread, time};
use std::collections::HashSet;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use std::error::Error;
use tokio::time::{ sleep, Duration};

#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

#[path = "../probability/create_adv_prob.rs"]
mod create_adv_prob;

#[path = "./client.rs"]
mod client;

#[path = "./server.rs"]
mod server;


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


async fn handle_client(ip: String, self_ip: String, types: String, port: u32, epoch: i32, behavior: String) // clinet: initiating data sending.
{    
    let _result = newclient::match_tcp_client([ip.to_string(), port.to_string()].join(":"), 
    [ip.to_string(), port.to_string()].join(":"), self_ip, "first".to_string());   
    
}




pub fn removeelement(mut all_ips: Vec<String>, ip: String) -> Vec<String>
{
    let index = all_ips.iter().position(|x| *x == ip).unwrap();
    all_ips.remove(index);

    return all_ips;
}

pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{  
    // let  blacklisted = HashSet::new(); // create blacklisted list (should change in recursion)

    // initiate_server(INITIAL_PORT.clone());

    let ip_address_clone = ip_address.clone();


    let args_clone = args.clone();

    let self_ip = args[6].clone();


    let mut port_count: u32 = 0;


    let  behavior = args[8].clone();


    for _index in 1..(args[7].parse::<i32>().unwrap()+1) // iterate for all epoch
    {   
         println!("epoch {}", _index);
        port_count+=1;
        if args[5]=="prod" // in prod mode
        {
            let mut all_ips = ip_address_clone.clone();

            loop{

                if all_ips.len()==0
                {
                    break;
                }

                thread::scope(|s| { 


                    s.spawn(|| {

                        // let mut retry_ips = Vec::new();

                        for _ip in ip_address_clone.clone() 
                        {
                            let _result = newserver::handle_server( ip_address_clone.clone(), INITIAL_PORT+port_count, TEST_PORT+port_count  );

                            println!("------------------{}-----------------------", _result);

                            let successful_ip: Vec<&str> = _result.split(" ").collect();

                            // let index = getindex(all_ips.clone(), successful_ip[0].clone().to_string());

                            // println!("{}", index);

                            // all_ips.remove(index);

                            all_ips = removeelement(all_ips.clone(), successful_ip[0].clone().to_string());
                        }
                        
                       
                    });

                                 
                    s.spawn(|| {
                        let three_millis = time::Duration::from_millis(10);
                        thread::sleep(three_millis);

                        

                        for ip in ip_address_clone.clone() 
                        {
                            
                            let self_ip_clone = self_ip.clone();

                            let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client([ip.to_string(), (INITIAL_PORT+port_count ).to_string()].join(":"),
                            [ip.to_string(), (TEST_PORT+port_count ).to_string()].join(":"), self_ip_clone, "first".to_string());


                        }

                    });

                    
    
                    
                });

            }    
                                
            
               
        }
        else 
        {                
            handle_client("127.0.0.1".to_string(), self_ip.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior.clone()).await;
        }


        println!("--------------------------------");

    }
    
    // for i in blacklisted.iter()
    // {
    //     println!("{}", i);
    // }
    
    

    

}