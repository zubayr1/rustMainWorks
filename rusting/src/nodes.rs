
use std::{thread, time};
use std::collections::HashSet;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use std::error::Error;

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

pub fn create_keys() // schnorr key generation
{

    schnorrkel::_create_keys_schnorrkel();

}


async fn handle_client(ip: String, self_ip: String, types: String, port: u32, epoch: i32, behavior: String) // clinet: initiating data sending.
{    
    let _result = newclient::match_tcp_client([ip.to_string(), port.to_string()].join(":"), self_ip);   
    
}


#[tokio::main]
pub async fn check_connect(address: String) -> Result<(), Box<dyn Error>> {

    let _result = TcpStream::connect(address.clone()).await?;
    Ok(())
}


pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{  
    // let  blacklisted = HashSet::new(); // create blacklisted list (should change in recursion)

    // initiate_server(INITIAL_PORT.clone());

    let three_millis = time::Duration::from_millis(5);
                        thread::sleep(three_millis);

    let ip_address_clone = ip_address.clone();


    let args_clone = args.clone();

    let self_ip = args[6].clone();


    let mut port_count: u32 = 0;


    let  behavior = args[8].clone();


    for _index in 1..(args[7].parse::<i32>().unwrap()+1) // iterate for all epoch
    {   
         
        port_count+=1;
        if args[5]=="prod" // in prod mode
        {


        
                thread::scope(|s| { 


                    s.spawn(|| {
                        for _ip in ip_address_clone.clone() 
                        {
                            
                            let _result = newserver::handle_server( ip_address_clone.clone(), INITIAL_PORT+port_count  );
                            
                            println!("{:?}", _result);
                        }
                        println!("aaa");
                    });

                                      


                    s.spawn(|| {
                        let three_millis = time::Duration::from_millis(10);
                        thread::sleep(three_millis);

                       // handle_wait(ip_address_clone.clone());
                       let mut count = 0;
                       let mut accepted_ips = Vec::new(); 
                       let mut check = 0;
                        loop{

                        for ip in ip_address_clone.clone() 
                        {
                            let ip_clone = ip.clone();
                            // if !accepted_ips.contains(&ip.clone())
                            // {
                                let self_ip_clone = self_ip.clone();
        
                                
                                let _result = newclient::match_tcp_client([ip.to_string(), (INITIAL_PORT+port_count ).to_string()].join(":"), self_ip_clone);
                                
                                if _result.is_ok()
                                {
                                    println!("{:?}", accepted_ips);
                                    accepted_ips.push(ip);
                                    count+=1;
                                }
                           // }

                            if  count>=400
                            {
                                check =1;
                            }

                        }

                        if check==1
                        {
                            println!("bbb");
                            break;
                        }

                        
                        
                    }
                    });

                    
    
                    
                });

            //}    
                                
            
               
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