use std::env;
use async_recursion::async_recursion;

use serde_derive::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct CValueTuple {
    id_details: String,
    value: String,
    _committee_id: String,
}

#[path = "../networking/communication.rs"]
mod communication;

#[path = "../types/generic.rs"]
mod generic; 


#[path = "../types/accum.rs"]
mod accum;

#[path = "../algos/pvss_agreement.rs"]
mod encoder;

#[path = "./timer.rs"]
mod timer; 

#[path = "./deliver.rs"]
mod deliver;

#[path = "../merkle_tree/merkle_tree.rs"]
mod merkle_tree;

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


async fn communication(committee_id: u32, ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, medium: String, mode: String,
    initial_port: u32, test_port: u32, value: Vec<String>, committee_length: usize) -> Vec<String>
{
    let mut output: Vec<String>= Vec::new();
    
    if medium=="prod_init"
    {
        output = communication::prod_communication(committee_id, ip_address.clone(), level, port_count, _index, args.clone(), value.clone()).await;

       
    }
    if medium=="dev_init"
    {
        output = communication::dev_communication(committee_id, ["127.0.0.1".to_string(), (initial_port + _index).to_string()].join(":"), 
            ["127.0.0.1".to_string(), (test_port + _index).to_string()].join(":"), value.clone(), args.clone()).await;

    }

    return output;
}


pub async fn reactor_init(committee_id: u32, ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, medium: String)
{       
    let committee_length = ip_address.len();

    let leaves = encoder::encoder(b"pvss_data", committee_length.clone()/2);


    let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

    let acc_value = merkle_tree::get_root(merkle_tree.clone());

    let empty_vec: Vec<Vec<u8>> = Vec::new();
   
    timer::wait(1);
    reactor(committee_id, &ip_address, level, _index, args, port_count, acc_value, empty_vec, 
        "accum".to_string(), medium, committee_length).await;
}


pub async fn reaction(output: Vec<String>, medium: String, mode: String, committee_length: usize) -> bool
{
    let mut check: bool = false;

    if medium=="prod_init"
    {
        if mode=="accum"
        {
            timer::wait(1);

            check= accum::accum_check(output, medium, committee_length);
        }
        
    }
    else 
    {
        if mode=="accum"
        {
            timer::wait(1);

            check= accum::accum_check(output, medium, committee_length);
        }
    }
    return check;
}

#[async_recursion]
pub async fn reactor<'a>(committee_id: u32, ip_address: &'a Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    value: String, witnesses_vec: Vec<Vec<u8>>, mode: String, medium: String, committee_length: usize) 
{ 

    let mut c: Vec<(String, String, String)> = Vec::new();
    let mut v: (String, String, String) = ("".to_string(), "".to_string(), "".to_string());

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

        println!("{:?}", witnesses_vec);
    }
    else 
    {
        let witnesses_vec: Vec<Vec<u8>> = accum_reactor(committee_id, &ip_address, level, _index, args.clone(), port_count, 
            value.clone(), mode, medium.clone(), committee_length, initial_port, test_port).await;


        reactor(committee_id, ip_address, level, _index, args, port_count, 
            value, witnesses_vec, "codeword".to_string(), medium, committee_length).await;
    }

    
     
}


pub async fn accum_reactor(committee_id: u32, ip_address: &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    value: String, mode: String, medium: String, committee_length: usize, initial_port: u32, test_port: u32) -> Vec<Vec<u8>>
    {

        let mut c: Vec<(String, String, String)> = Vec::new();
        let mut v: (String, String, String) = ("".to_string(), "".to_string(), "".to_string());

        let accum = generic::Accum::create_accum("sign".to_string(), value);
        let accum_vec = accum.to_vec();

        let output = communication(committee_id.clone(), ip_address.clone(), level, _index, args.clone(), port_count, 
            medium.clone(), mode.clone(), initial_port, test_port, accum_vec, committee_length).await;

        let check = reaction(output.clone(), medium.clone(), mode.clone(), committee_length.clone()).await;

        if check==true
        {
            c = accum::accum_reaction(medium.clone(), output);
        }
       
        v = accum::call_byzar(c);

        timer::wait(1);

        let json_string = serde_json::to_string(&v).unwrap();

        let deserialized_tuple: CValueTuple = serde_json::from_str(&json_string.to_string()).unwrap();

        let CValueTuple {id_details, value, _committee_id} = deserialized_tuple;

        let mut witnesses_vec: Vec<Vec<u8>>= Vec::new();

        if value!="".to_string()
        {
            witnesses_vec = deliver::deliver_encode(b"pvss_data", value.clone(), committee_length.clone());
        }

        return witnesses_vec;
    }