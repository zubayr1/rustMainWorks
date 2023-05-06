
use std::{thread, time};
use std::collections::HashSet;
use tokio::net::TcpStream;

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

const INITIAL_PORT: u32 = 7281;

pub fn create_keys() // schnorr key generation
{

    schnorrkel::_create_keys_schnorrkel();

}


async fn handle_client(ip: String, self_ip: String, types: String, port: u32, epoch: i32, behavior: String) // clinet: initiating data sending.
{    
    let _result = newclient::match_tcp_client([ip.to_string(), port.to_string()].join(":"), self_ip);   
    
}
#[tokio::main]
pub async fn handle_wait(ip_address: Vec<String>) {
    // First wait for all nodes to be online.
       let _result =  tokio::spawn(async move {
            for address in ip_address{
            while TcpStream::connect(address.clone()).await.is_err() {
                let three_millis = time::Duration::from_millis(10);
                                    thread::sleep(three_millis);
                                    

                if TcpStream::connect(address.clone()).await.is_ok()
                {
                    println!("a");
                    break;
                }
            }

        }
        })
   
    .await;

    
}


pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{  
    // let  blacklisted = HashSet::new(); // create blacklisted list (should change in recursion)



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
                                      println!("ddd");     
                            
                            let _result = newserver::handle_server( INITIAL_PORT+port_count  );
                        
                        }
                    });
                    


                    s.spawn(|| {
                        let three_millis = time::Duration::from_millis(3);
                        thread::sleep(three_millis);

                       // handle_wait(ip_address_clone.clone());

                        for ip in ip_address_clone.clone() 
                        {
                            let self_ip_clone = self_ip.clone();
    
    
                            let _result = newclient::match_tcp_client([ip.to_string(), (INITIAL_PORT+port_count ).to_string()].join(":"), self_ip_clone);
                        
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