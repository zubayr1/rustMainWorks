use std::{thread, time};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;

#[path = "./newclient.rs"]
mod newclient;

#[path ="./newserver.rs"]
mod newserver;



pub async fn prod_communication(ip_address: Vec<&str>, level: u32, port_count: u32, _index:u32, args: Vec<String>, message_type: String) -> Vec<String>
{

    let initial_port_str = env::var("INITIAL_PORT").unwrap_or_else(|_| {
        println!("INITIAL_PORT_STR is not set.");
        String::new()
    });
    let test_port_str = env::var("TEST_PORT").unwrap_or_else(|_| {
        println!("TEST_PORT_STR is not set.");
        String::new()
    });
   
    let initial_port: u32 = initial_port_str.parse().unwrap();
    let test_port: u32 = test_port_str.parse().unwrap();

    let mut file: std::fs::File = OpenOptions::new().append(true).open("output.log").unwrap();

    let mut text;

    let mut output: Vec<String> = Vec::new();

    text = ["epoch ".to_string(), _index.to_string()].join(": ");
    file.write_all(text.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
    
        

    let ip_address_clone = ip_address.clone();
    
    text = ["Level ".to_string(), level.to_string()].join(": ");
    file.write_all(text.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
    
    thread::scope(|s| { 

        s.spawn(|| 
        {
            
            let mut count=1;
            for _ip in ip_address_clone.clone() 
            {
                count+=1;
                let additional_port = (count + args[2].parse::<u32>().unwrap())*10;

                let _result = newserver::handle_server( ip_address_clone.clone(), initial_port+port_count, test_port+port_count + additional_port );
                //println!("{:?}", _result);
                output.push(_result);
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

                let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client([ip.to_string(), (initial_port+port_count).to_string()].join(":"),
                [ip.to_string(), (test_port+port_count + additional_port).to_string()].join(":"), message_type.clone());

                
            }

        });

    });


    return output;

}


pub async fn dev_communication(working_port: String, test_port: String, message_type: String) -> String
{
    let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client(working_port, test_port, message_type.clone());

    return message_type;
}
