use std::fs;
use tokio::fs::{OpenOptions};

use tokio::net::TcpStream;
use tokio::io::{ AsyncWriteExt};

use std::{thread, time};

#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String, types: String, epoch: i32, behavior: String)
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


    // let stream = TcpStream::connect(address).await.unwrap(); //tokio TCPStream to connect to listening server


    let addressclone = address.clone();

    
    while TcpStream::connect(addressclone.clone()).await.is_err() {
        let three_millis = time::Duration::from_millis(3);
        thread::sleep(three_millis);
    }

    if TcpStream::connect(addressclone.clone()).await.is_err()
    {
        println!("not");
    }

   // if TcpStream::connect(addressclone.clone()).await.is_ok(){

    let stream = TcpStream::connect(address).await.unwrap(); 

    let (_, mut write) = tokio::io::split(stream); 

    println!("connection done");

    file.write_all("connection done".as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();
    
    println!("aaa{}", addressclone);


    if types == "none" // types == "none": first time communication
    {   
        if behavior=="1" // if 1: can act as adversary and send false signature
        {
            let false_key = schnorrkel::_create_adversarial_key();
            println!("bbb");
            write.write_all(false_key.as_bytes()).await.unwrap();
            
        }
        else // acts as honest node
        {
            write.write_all(pubkey.as_bytes()).await.unwrap();
        }
        
        write.write_all(sign.as_bytes()).await.unwrap(); // write signature to server.
        let id = [self_ip.to_string(), "messageEOF".to_string()].join(" ");
        write.write_all(id.as_bytes()).await.unwrap();

        println!("ccc");
    } 
    else // next communication. REACTOR to be used here
    {
        write.write_all(types.as_bytes()).await.unwrap();
        write.write_all(types.as_bytes()).await.unwrap();
        write.write_all(b"EOF").await.unwrap();
    }

//}
        

}
