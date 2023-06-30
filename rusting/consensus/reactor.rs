use std::env;

#[path = "../networking/communication.rs"]
mod communication;

#[path = "../types/generic.rs"]
mod generic; 

#[path = "../algos/pvss_agreement.rs"]
mod encoder;

#[path = "./timer.rs"]
mod timer; 

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



pub async fn reactor_init(ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>,types: String)
{   
    //encoder::encoder(b"pvss_data", 4);
    // call pvss
    timer::wait(1);
    reactor(ip_address, level, _index, args, "accum".to_string(), types).await;
}


pub async fn reaction(output: Vec<String>, message_type: String, types: String)
{
    if types=="prod_init"
    {
        //println!("{:?}", output);
    }
    else 
    {
        println!("{}", message_type);            
    }
}

pub async fn reactor(ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, line: String, types: String) 
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

    let port_count: u32 = 0;

    let mut message_type ="".to_string();
    let mut output: Vec<String> = Vec::new();

    if line.contains("echo")
    {
        let echo = generic::Echo::create_echo("".to_string(), "".to_string());
    }
    else if line.contains("vote")
    {
        let vote: generic::Vote = generic::Vote::create_vote("".to_string(), "".to_string());
    }
    else if line.contains("committee")
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
        output = communication::prod_communication(ip_address.clone(), level, port_count, _index, args.clone(), line.clone()).await;

        reaction(output.clone(), message_type, types.clone()).await;

    }
    if types=="dev_init"
    {
        message_type = communication::dev_communication(["127.0.0.1".to_string(), (initial_port + _index).to_string()].join(":"), 
            ["127.0.0.1".to_string(), (test_port + _index).to_string()].join(":"), line.clone()).await;

        reaction(output.clone(), message_type, types.clone()).await;
    }
     
}