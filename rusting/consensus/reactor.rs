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


async fn communication(ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, medium: String, 
    initial_port: u32, test_port: u32, value: Vec<String>)
{
    let mut output: Vec<String> = Vec::new();
    
    if medium=="prod_init"
    {
        output = communication::prod_communication(ip_address.clone(), level, port_count, _index, args.clone(), value.clone()).await;

        reaction(output.clone(), medium.clone()).await;

    }
    if medium=="dev_init"
    {
        output = communication::dev_communication(["127.0.0.1".to_string(), (initial_port + _index).to_string()].join(":"), 
            ["127.0.0.1".to_string(), (test_port + _index).to_string()].join(":"), value.clone()).await;

        reaction(output.clone(), medium.clone()).await;
    }
}


pub async fn reactor_init(ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, medium: String)
{   
    let accul_value = encoder::encoder(b"pvss_data", ip_address.len());
   
    // call pvss
    timer::wait(1);
    reactor(ip_address, level, _index, args, port_count, accul_value, "accum".to_string(), medium).await;
}


pub async fn reaction(output: Vec<String>, medium: String)
{
    if medium=="prod_init"
    {
        //println!("{:?}", output);
    }
    else 
    {
        println!("{:?}", output);           
    }
}

pub async fn reactor(ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, value: String, mode: String, medium: String) 
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
        

    if mode.contains("echo")
    {
        let echo = generic::Echo::create_echo("".to_string(), "".to_string());
        let echo_vec = echo.to_vec();

    }
    else if mode.contains("vote")
    {
        let vote: generic::Vote = generic::Vote::create_vote("".to_string(), "".to_string());
    }
    else if mode.contains("committee")
    {
        let committee = generic::Committee::create_committee("".to_string(), "".to_string());
    }
    else if mode.contains("codeword")
    {
        let codeword = generic::Codeword::create_codeword("".to_string(), "".to_string(), "".to_string(),
        "".to_string());
    }
    else 
    {
        let accum = generic::Accum::create_accum("".to_string(), value);
        let accum_vec = accum.to_vec();

        communication(ip_address, level, _index, args, port_count, medium, initial_port, test_port, accum_vec).await;
    }

    
     
}