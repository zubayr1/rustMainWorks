use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::ReadHalf;
// use std::{thread, time};
use std::{thread, time};

use tokio::fs::{OpenOptions};

use std::collections::HashSet;

#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

type SomeResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main] //3 instances
pub async fn handle_server(ip: String, server_type: String, ip_address: Vec<String>, args: Vec<String>, self_ip: String, port: u32, epoch: i32, mut blacklisted: HashSet<String>) -> SomeResult<()>{
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();


    let mut text = ["epoch".to_string(), epoch.to_string()].join(": ");

    println!("{}", text);

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    
    text = ["server at port".to_string(), port.to_string()].join(": ");

    println!("{}", text);

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();
    
    let mut count =0;

    let mut messageperepochcount = 0;

    
  // loop {
        let (mut socket, _) = listener.accept().await.unwrap(); // starts listening
        println!("---continue---");
        file.write_all("---continue---".as_bytes()).await.unwrap();
        file.write_all(b"\n").await.unwrap();


        let (reader, mut writer) = socket.split(); // tokio socket split to read and write concurrently
        
        let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
        let mut line: String  = String :: new();
        

        let ip_address_clone;
        let line_clone;


        loop { //loop to get all the data from client until EOF is reached

     
                let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();
                               
                if line.contains("EOF") //REACTOR to be used here
                {
                    println!("EOF Reached");
                    
                    file.write_all("EOF Reached".as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();


                    writer.write_all(line.as_bytes()).await.unwrap();
                    println!("{}", line);
                    
                    file.write_all(line.as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();
                    
                    ip_address_clone = ip_address.clone();

                    line_clone = line.clone();
                    

                    line.clear();

                    break;
                }
                
                
            }

           
            let line_collection: Vec<&str> = line_clone.split("//").collect();
            
            if line_collection.len()>=3
            {
                let pubkeystr = line_collection[0];
                let signstr = line_collection[1];
         
                   
                if schnorrkel::_verify_schnorrkel_sign(pubkeystr, signstr) // verify schnorr signature 
                {
                    println!("Identity Verified");
                    
                    file.write_all("Identity Verified".as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();

                    let id_info: Vec<&str> = line_collection[2].split(" ").collect();

                    if count<=1
                    {
                        count+=1;
                        // for ip in ip_address_clone.clone() // Broadcast to everyone. deliver to be used here.
                        // {   
                        //     if ip!=self_ip.clone() 
                        //     {   messageperepochcount+=1;
                        //         let address;
                        //         if args[5]=="dev"
                        //         {
                        //             address = ["127.0.0.1".to_string(), port.to_string()].join(":");
                        //         }
                        //         else 
                        //         {
                        //             address = [ip.to_string(), port.to_string()].join(":")
                        //         }
                              
                        //         let mut stream = TcpStream::connect(address).await?; 
                                
                        //         let message1 = ["Re: Text".to_string(), self_ip.to_string().to_string()].join(" ");

                        //         let message2 = [message1.to_string(), id_info[0].to_string().to_string()].join(" ");
                                
                        //         let broadcast_about_false_leader = [message2.to_string(), "EOF".to_string()].join(" ");
                                
                        //         let finalmessage = [broadcast_about_false_leader, epoch.to_string()].join(" ");

                        //         let _result = stream.write(finalmessage.as_bytes()).await;

                                    
                        //     }                                
                            
                        // }
                    }
                }
                else 
                {
                    println!("Identity Verification Failed. Aborting Broadcasting...");

                    
                    file.write_all("Identity Verification Failed. Aborting Broadcasting...".as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();

                    let id_info: Vec<&str> = line_collection[2].split(" ").collect();

                    blacklisted.insert(id_info[0].to_string());

                    if count<=1
                    {
                        count+=1;
                        
                        // for ip in ip_address_clone.clone() // Broadcast to everyone. deliver to be used here.
                        // {   
                        //     if ip!=self_ip.clone() 
                        //     {   messageperepochcount+=1;
                        //         let address;
                        //         if args[5]=="dev"
                        //         {
                        //             address = ["127.0.0.1".to_string(), port.to_string()].join(":");
                        //         }
                        //         else 
                        //         {
                        //             address = [ip.to_string(), port.to_string()].join(":")
                        //         }
                              
            
                        //         let mut stream = TcpStream::connect(address).await?; 

                        //         let message1 = ["Re: Identity Verification Failed".to_string(), self_ip.to_string().to_string()].join(" ");

                        //         let message2 = [message1.to_string(), id_info[0].to_string().to_string()].join(" ");
                                
                        //         let broadcast_about_false_leader = [message2.to_string(), "EOF".to_string()].join(" ");
                                
                        //         let finalmessage = [broadcast_about_false_leader, epoch.to_string()].join(" ");

                        //         let _result = stream.write(finalmessage.as_bytes()).await;
                                            
                        //     }                                
                            
                        // }
                    }
                }
            }
        //     println!("{}", messageperepochcount);
        //     if messageperepochcount==3
        //     {
        //         break;
        //     }

        // }
            
        Ok(())
            
            
            

   // }
}
