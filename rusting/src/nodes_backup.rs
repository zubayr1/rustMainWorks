
use std::{thread, time};
use futures::executor::block_on;
use std::collections::HashSet;

#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

#[path = "../probability/create_adv_prob.rs"]
mod create_adv_prob;

#[path = "./client.rs"]
mod client;

#[path = "./server.rs"]
mod server;

const INITIAL_PORT: u32 = 7081;

pub fn create_keys() // schnorr key generation
{

    schnorrkel::_create_keys_schnorrkel();

}


async fn handle_client(ip: String, self_ip: String, types: String, port: u32, epoch: i32, behavior: String) // clinet: initiating data sending.
{    
    client::match_tcp_client([ip.to_string(), port.to_string()].join(":"), self_ip, types, epoch, behavior);   
    
}




pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{  
    let mut blacklisted = HashSet::new(); // create blacklisted list (should change in recursion)

    let mut round_robin_count=0; // to have leader selected in round robin way

    let total = args[3].clone(); // get total number of nodes

    let ip_address_clone = ip_address.clone();


    let args_clone = args.clone();

    let self_ip = args[6].clone();

    let mut count:usize = 0;

    let mut port_count: u32 = 0;


    let mut behavior = args[8].clone();
  

    for _index in 1..(args[7].parse::<i32>().unwrap()+1) // iterate for all epoch
    {
        
        round_robin_count%=total.clone().parse::<i32>().unwrap();       
        round_robin_count+=1;

        count%=total.parse::<usize>().unwrap();       
        
        let mut leader = ip_address_clone[count].clone();

        count+=1;
        port_count+=1;

        
        if args[5]=="prod" // in prod mode
        {
            while blacklisted.contains(&leader) { // ignore blacklisted node from leader (should change in recursion)
                round_robin_count+=1;   
                
                leader = ip_address_clone[count].clone();
                count+=1;
                
            }


            
            if round_robin_count==args[2].parse::<i32>().unwrap()
            {
                if behavior=="1"
                {
                    if create_adv_prob::create_prob()
                    {
                        behavior="1".to_string();
                    }
                    else 
                    {
                        behavior="0".to_string();
                    }
                }

                for ip in ip_address_clone.clone() //LEADER SENDS TO EVERY IP (should change in recursion because in recursion, its distributed communication) 
                {
                    let self_ip_clone = self_ip.clone();
                    let behavior_clone =behavior.clone();

                    
                    if !blacklisted.clone().contains(&ip.clone())
                    {
                        if ip==self_ip.clone()
                        {
                            let ip_address_clone = ip_address.clone();
                            let args_clone1 = args_clone.clone();
                            let self_ip_clone1 = self_ip.clone();  

                           
                            thread::scope(|s| { // tokio thread, since leader is both client and server
                                s.spawn(|| {
                                    let blacklisted_child = server::handle_server("selfserver".to_string(), ip_address_clone.clone(), args_clone1.clone(), self_ip_clone1.clone(), INITIAL_PORT+port_count, _index, blacklisted.clone());
                                    
                                    blacklisted.extend(blacklisted_child);
                                });
                
                                s.spawn(|| {
                                    let three_millis = time::Duration::from_millis(3);
                                    thread::sleep(three_millis);
            
                                    let future = handle_client(ip.clone(), self_ip_clone.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior_clone.clone());
            
                                    block_on(future);
                                });
                            });


                        }
                        else 
                        {   
                            let three_millis = time::Duration::from_millis(3);
                            thread::sleep(three_millis);
                            handle_client(ip.clone(), self_ip_clone.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior_clone.clone()).await;
                        }

                        

                    }
                    else 
                    {   
                        let three_millis = time::Duration::from_millis(3);
                        thread::sleep(three_millis);
                        handle_client(ip.clone(), self_ip_clone.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior_clone.clone()).await;
                    }
                    
                    
                                    
                }
                
            }
            else // other nodes, acting only as server. (should change in recursion. In that this will also contain tokio thread)
            {
                let blacklisted_child = server::handle_server("otherserver".to_string(), ip_address.clone(), args_clone.clone(), self_ip.clone(), INITIAL_PORT+port_count, _index, blacklisted.clone());
                
                blacklisted.extend(blacklisted_child.into_iter());
            }

            
        }
        else 
        {                
            handle_client("127.0.0.1".to_string(), self_ip.clone(), "none".to_string(), INITIAL_PORT+port_count, _index, behavior.clone()).await;
        }


        println!("--------------------------------");

    }
    
    for i in blacklisted.iter()
    {
        println!("{}", i);
    }
    
    

    

}