use std::{thread, time};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

#[path = "../networking/newclient.rs"]
mod newclient;

#[path ="../networking/newserver.rs"]
mod newserver;

#[path = "../types/generic.rs"]
mod generic; 

const INITIAL_PORT: u32 = 7821;
const TEST_PORT: u32 = 7921;

enum Phase 
{
    echo, vote, committee, codeword, accum
}

impl Phase 
{
    pub fn is_weekday(&self) -> bool
    {
        match self 
        {
            &Phase:: echo => return false,
            _=> return true
        }
    }
}


async fn prod_communication(sorted: Vec<(&u32, &String)>, mut port_count: u32, _index:u32, args: Vec<String>, message_type: String)
{
    let mut file: std::fs::File = OpenOptions::new().append(true).open("output.log").unwrap();

    let mut level=0;

    let mut text;

    text = ["epoch ".to_string(), _index.to_string()].join(": ");
    file.write_all(text.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();

    for (_i, ip_addresses_comb) in sorted.clone()
    {
        port_count+=1;

        let ip_address: Vec<&str> = ip_addresses_comb.split(" ").collect();

        let ip_address_clone = ip_address.clone();
        
        text = ["Level ".to_string(), level.to_string()].join(": ");
        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
        level+=1;
                
        thread::scope(|s| { 

            s.spawn(|| 
            {
                
                let mut count=1;
                for _ip in ip_address_clone.clone() 
                {
                    count+=1;
                    let additional_port = (count + args[2].parse::<u32>().unwrap())*10;
                    
                    let _result = newserver::handle_server( ip_address_clone.clone(), INITIAL_PORT+port_count, TEST_PORT+port_count + additional_port );
                
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

                    let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client([ip.to_string(), (INITIAL_PORT+port_count).to_string()].join(":"),
                    [ip.to_string(), (TEST_PORT+port_count + additional_port).to_string()].join(":"), message_type.clone());

                    
                }

            });

        });
    }


}


async fn dev_communication(working_port: String, test_port: String, message_type: String)
{
    let _result: Result<(), Box<dyn Error>> = newclient::match_tcp_client(working_port, test_port, message_type);

}


pub async fn reactor_init(sorted: Vec<(&u32, &String)>, _index: u32, args: Vec<String>, line: String, types: String)
{    
    reactor(sorted, _index, args, line, types).await;
}

pub async fn reactor(sorted: Vec<(&u32, &String)>, _index: u32, args: Vec<String>, line: String, types: String) 
{    
    let port_count: u32 = 0;

    if line.contains("echo")
    {
        let echo = generic::Echo::create_echo("".to_string(), "".to_string());
    }
    else if line.contains("vote")
    {
        let vote: generic::Vote = generic::Vote::create_vote("".to_string(), "".to_string());
    }
    if line.contains("committee")
    {
        let committee = generic::Committee::create_committee("".to_string(), "".to_string());
    }
    else if line.contains("codeword")
    {
        let codeword = generic::Codeword::create_codeword("".to_string(), "".to_string(), "".to_string(),
        "".to_string());
    }
    else 
    {
        let accum = generic::Accum::create_accum("".to_string(), "".to_string());
    }

    if types=="prod_init"
    {
        prod_communication(sorted.clone(), port_count, _index, args.clone(), line.clone()).await;

    }
    if types=="dev_init"
    {
        dev_communication(["127.0.0.1".to_string(), (INITIAL_PORT + _index).to_string()].join(":"), 
            ["127.0.0.1".to_string(), (TEST_PORT + _index).to_string()].join(":"), line.clone()).await;
    }
     
}