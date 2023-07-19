use crate::nodes::reactor::communication;


#[path = "../types/generic.rs"]
mod generic; 

use std::env;


async fn gba_communication(committee_id: u32, ip_address: Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, value: Vec<String>, medium: String, mode: String, types: String, committee_length: usize) -> Vec<String>
{
    

    if medium=="prod_init"
    {
        let output = communication::prod_communication(committee_id, ip_address.clone(), level, port_count, _index, 
        args.clone(), value.clone(), mode.clone(), types.clone()).await;

        return output;
    }
    else 
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

        let output = communication::nested_dev_communication(committee_id, (initial_port + _index).to_string(), 
        (test_port + _index).to_string(), value.clone(), args.clone()).await;

        return output;
    }

    

}

#[allow(non_snake_case)]
fn check_echo_major_v(echo_phase_output: Vec<String>, V: String) -> usize
{
    let mut count: usize = 0;

    let val: &str = V.as_str();

    for output in echo_phase_output
    {
        let split_output: Vec<&str> = output.split(", ").collect();

        if split_output.contains(&val.clone())
        {
            count+=1;
        }
    }

    return count;

}

#[allow(non_snake_case)]
pub async fn gba(committee_id: u32, ip_address: Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, V: String, medium: String, mode: String, types: String, committee_length: usize)
{
    let mut W: String = "".to_string();
    let mut C1: String = "".to_string();
    let mut C2: String = "".to_string();

    let mut g: usize = 0;

    let mut sent: bool = false;

    let b = committee_length/2;

    let echo = generic::Echo::create_echo("".to_string(), V.to_string());
    let echo_vec = echo.to_vec();

    let echo_phase_output = gba_communication(committee_id, ip_address.clone(), level, port_count, _index, 
    args.clone(), echo_vec, medium.clone(), mode.clone(), types.clone(), committee_length).await;
    
    let count = check_echo_major_v(echo_phase_output.clone(), V.clone());

    println!("{:?}, {:?}", echo_phase_output, count);
    
    // if output.len() > b
    // {
    //     W =V.clone();
    // }


    // let mut W_vec: Vec<String> = Vec::new();

    // W_vec.push(W.clone());

    // let output = gba_communication(committee_id, ip_address.clone(), level, port_count, _index, 
    // args.clone(), W_vec, medium.clone(), mode.clone(), types.clone(), committee_length).await;

    // sent = true;

    // // println!("{:?}", output);




    // let vote = generic::Vote::create_vote("".to_string(), V.to_string());
    // let vote_vec = vote.to_vec();

    
    // let output = gba_communication(committee_id, ip_address.clone(), level, port_count, _index, 
    // args.clone(), vote_vec.clone(), medium.clone(), mode.clone(), types.clone(), committee_length).await;


    // if output.len() >= b
    // {
    //     C1 = W.clone();
    // }

    // let output = gba_communication(committee_id, ip_address.clone(), level, port_count, _index, 
    // args.clone(), vote_vec, medium, mode.clone(), types.clone(), committee_length).await;

    // if output.len() >= b
    // {
    //     C2 = W;
    // }

    // println!("{:?}", output);

}