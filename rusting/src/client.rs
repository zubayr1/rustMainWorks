use std::{fs, result};
use tokio::fs::{OpenOptions};
use std::error::Error;

use tokio::net::TcpStream;
use tokio::io::{ AsyncWriteExt};

#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

type SomeResult<T> = Result<T, Box<dyn std::error::Error>>;


#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String, types: String, epoch: i32, behavior: String) -> SomeResult<()>
{   
    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let mut text = ["epoch".to_string(), epoch.to_string()].join(": ");

    println!("{}", text);

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    println!("client");

    file.write_all("client".as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    
    text = ["server address is".to_string(), address.to_string()].join(": ");

    println!("{}", text);

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    //reading pubkey and signature of schnorr
    let pubkey = fs::read_to_string("../pubkey.txt").expect("Unable to read file");
    let sign = fs::read_to_string("../sign.txt").expect("Unable to read file");

   
    
    // let std_stream = std::net::TcpStream::connect(address)?; 
    // std_stream.set_nonblocking(true)?;

    let mut stream = TcpStream::connect(address).await?;
    // let (_, mut write) = tokio::io::split(stream); 

    println!("connection done");

    file.write_all("connection done".as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();
    

    
    // if types == "none" // types == "none": first time communication
    // {   
    //     if behavior=="1" // if 1: can act as adversary and send false signature
    //     {
    //         let false_key = schnorrkel::_create_adversarial_key();
    //         write.write_all(false_key.as_bytes()).await.unwrap();
            
    //     }
    //     else // acts as honest node
    //     {
    //         write.write_all(pubkey.as_bytes()).await.unwrap();
    //     }
        
    //     write.write_all(sign.as_bytes()).await.unwrap(); // write signature to server.
    //     let id = [self_ip.to_string(), "message".to_string()].join(" ");
    //     write.write_all(id.as_bytes()).await.unwrap();

    // } 
    // else // next communication. REACTOR to be used here
    // {
    //     write.write_all(types.as_bytes()).await.unwrap();
    //     write.write_all(types.as_bytes()).await.unwrap();
        
    // }
    // let _result = stream.write([self_ip.to_string(), "EOF".to_string()].join(" ").as_bytes()).await;
    // // write.shutdown().await?;
    
    // println!("{:?}", _result.is_ok());

    while stream.write([self_ip.to_string(), "EOF".to_string()].join(" ").as_bytes()).await.is_err()
    {

    }

    Ok(())

}
