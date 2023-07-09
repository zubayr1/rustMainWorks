use std::{thread, time};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::env;
use futures::executor::block_on;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[path = "./newclient.rs"]
mod newclient;

#[path ="./newserver.rs"]
mod newserver;

#[path = "./nested_nodes_test.rs"]
mod nested_nodes_test;

#[path = "../types/codeword.rs"]
mod codeword;

pub async fn prod_communication(committee_id: u32, ip_address: Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, value: Vec<String>, types: String) -> Vec<String>
{
    let file_path = "./nodes_information.txt";
    let file = File::open(file_path).unwrap();

    let reader = BufReader::new(file);


    let mut client_count = 1;

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        
        if line.contains(ip_address[0])
        {
            break;
        }
        client_count+=1;
    }


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

            let length = i32::pow(2, level);

            if types.contains("individual")
            {
                count=0;
                // for i in 0..length 
                // {
                    count+=1;

                    println!("{:?},   {:?}", count, _index);

                    let additional_port = (args[2].parse::<u32>().unwrap())*10;

                    println!("server {:?}, {:?}, {:?}", (initial_port+port_count), (test_port+port_count + additional_port), additional_port);

                    let _result = newserver::handle_server( ip_address_clone.clone(), initial_port+port_count, 
                    test_port+port_count+ additional_port );

                    println!("{:?}", _result);
                    println!("\n");

                    let witness_verify =  codeword::verify_codeword(_result.clone());
        
                    if witness_verify==true
                    {
                        output.push(_result);
                    }
                    
             //   }

            }
            else
            {
                for _ip in ip_address_clone.clone() 
                {   
                    count+=1;
                    let additional_port = (count + args[2].parse::<u32>().unwrap())*10;
    
                    let _result = newserver::handle_server( ip_address_clone.clone(), initial_port+port_count, 
                    test_port+port_count + additional_port);

                    output.push(_result);
                    
                                        
                    
                }
            }
            
            
            
        });

                        
        s.spawn(|| {
            let three_millis = time::Duration::from_millis(3);
            thread::sleep(three_millis);

            let mut count=1;

            
            if types.contains("individual")
            {
                
                let additional_port = (client_count)*10;

                println!("client {:?}, {:?}, {:?}", (initial_port+port_count), (test_port+port_count + additional_port), additional_port);
                println!("client count: {:?},   {:?}", count, _index);
                println!("{:?}", ip_address_clone);

                let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client([ip_address_clone[0].to_string(), (initial_port+port_count).to_string()].join(":"),
                [ip_address_clone[0].to_string(), (test_port+port_count + additional_port).to_string()].join(":"), 
                committee_id.clone(), value.clone(), args.clone());
            }
            else 
            {
                for ip in ip_address_clone.clone() 
                {
                    count+=1;
                    let additional_port = (count + args[2].parse::<u32>().unwrap())*10;

                    let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client([ip.to_string(), (initial_port+port_count).to_string()].join(":"),
                    [ip.to_string(), (test_port+port_count + additional_port).to_string()].join(":"), 
                    committee_id.clone(), value.clone(), args.clone());
                    
                }
            }

            

        });

    });


    return output;

}


pub async fn dev_communication(committee_id: u32, working_port: String, test_port: String, mut value: Vec<String>, args: Vec<String>) -> Vec<String>
{    
    let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client(working_port, test_port, committee_id.clone(), value.clone(), args.clone());
    
    value.push(committee_id.to_string());
    
    return value;
}






pub async fn codeword_dev_communication(committee_id: u32, working_port: String, test_port: String, mut value: Vec<String>, args: Vec<String>) -> Vec<String>
{    
    

    value.push(committee_id.to_string());


    let initial_port_server: u32 = working_port.parse().unwrap();
    let test_port_server: u32 = test_port.parse().unwrap();

    let initial_port_client: u32 = working_port.parse().unwrap();
    let test_port_client: u32 = test_port.parse().unwrap();


    thread::scope(|s| { 

        s.spawn(|| 
        {
            
            let future = nested_nodes_test::initiate( 
            initial_port_server + 500, test_port_server + 500);

            block_on(future);
            
            
        });

                        
        s.spawn(|| {
            let three_millis = time::Duration::from_millis(3);
            thread::sleep(three_millis);

            let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client(
                ["127.0.0.1".to_string(), (initial_port_client + 500).to_string()].join(":"),
                ["127.0.0.1".to_string(), (test_port_client + 500).to_string()].join(":"),
                committee_id.clone(), value.clone(), args.clone());


        });

    });
    
    return value;
}
