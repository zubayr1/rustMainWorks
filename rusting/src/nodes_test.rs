
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::ReadHalf;
use tokio::fs::{OpenOptions};

use schnorrkel::{Signature, PublicKey, signing_context};

const INITIAL_PORT: u32 = 7081;


#[tokio::main]
async fn handle_server(ip_address: Vec<String>, args: Vec<String>, port: u32) {

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let text = ["server at port".to_string(), port.to_string()].join(": ");

    println!("{}", text);

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    let listener = TcpListener::bind(["127.0.0.1".to_string(), port.to_string()].join(":")).await.unwrap();  

    let mut count =0;

    let mut messageperepochcount = 0;

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        println!("---continue---");

        file.write_all("---continue---".as_bytes()).await.unwrap();
        file.write_all(b"\n").await.unwrap();

        let arg_ip = args[6].clone();
        
        let (reader, mut writer) = socket.split();
        
        let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
        let mut line: String  = String :: new();
        

        let ip_address_clone;
        let line_clone;

        
        loop {
            
                let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();
                
                println!("{}", line);              
                if line.contains("EOF")
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
        
    
                let pubkeybytes: Vec<u8> = serde_json::from_str(pubkeystr).unwrap();
                let signstrbytes: Vec<u8> = serde_json::from_str(signstr).unwrap();

               
                let public_key: PublicKey = PublicKey::from_bytes(&pubkeybytes).unwrap();
    
                let signature:  Signature = Signature::from_bytes(&signstrbytes).unwrap();
    
                
                let context = signing_context(b"signature context");
                let message: &[u8] = "zake kal".as_bytes();
    
                if public_key.verify(context.bytes(message), &signature).is_ok()
                {
                    println!("Identity Verified");

                    file.write_all("Identity Verified".as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();


                    if count<=1
                    {
                        count+=1;

                        for ip in ip_address_clone.clone() // Broadcast to everyone
                        {   
                            if ip!=arg_ip.clone()
                            {
                                let address;
                                if args[5]=="dev"
                                {
                                    address = ["127.0.0.1".to_string(), port.to_string()].join(":");
                                }
                                else 
                                {
                                    address = [ip.to_string(), port.to_string()].join(":")
                                }
            
                                let mut stream = TcpStream::connect(address).await.unwrap();
                                
                                let message = ["text".to_string(), "EOF".to_string()].join(" ");
                                
                                stream.write_all(message.as_bytes()).await.unwrap();
            
                                    
                            }                                
                            
                        }
                    }
                }
                else 
                {
                    println!("Identity Verification Failed. Aborting Broadcasting...");

                    file.write_all("Identity Verification Failed. Aborting Broadcasting...".as_bytes()).await.unwrap();
                    file.write_all(b"\n").await.unwrap();


                    if count<=1
                    {
                        count+=1;

                        for ip in ip_address_clone.clone() // Broadcast to everyone
                        {   
                            if ip!=arg_ip.clone()
                            {
                                let address;
                                if args[5]=="dev"
                                {
                                    address = ["127.0.0.1".to_string(), port.to_string()].join(":");
                                }
                                else 
                                {
                                    address = [ip.to_string(), port.to_string()].join(":")
                                }
            
                                let mut stream = TcpStream::connect(address).await.unwrap();
                                
                                let message = ["Identity Verification Failed".to_string(), "EOF".to_string()].join(" ");
                                
                                stream.write_all(message.as_bytes()).await.unwrap();
            
                                    
                            }                                
                            
                        }
                    }
                }
            }

            messageperepochcount+=1;

            if messageperepochcount>=args[3].clone().parse::<i32>().unwrap()
            {
                break;
            }
            

    }
}




pub async fn initiate(ip_address: Vec<String>, args: Vec<String>)
{
    let ip_address_clone = ip_address.clone();

    let mut port_count = 0;
    for _index in 1..(args[7].parse::<i32>().unwrap()+1)
    {
        port_count+=1;
        if args[2]<args[3]
        {
            handle_server(ip_address_clone.clone(), args.clone(), INITIAL_PORT+port_count);
    
            
        }
        
    }
    
    
    
}