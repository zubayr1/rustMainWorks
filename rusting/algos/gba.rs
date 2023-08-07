use crate::nodes::reactor::communication;


#[path = "../types/generic.rs"]
mod generic; 

use std::env;


async fn gba_communication(committee_id: u32, ip_address: Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, value: Vec<String>, medium: String, mode: String, types: String) -> Vec<String>
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
fn check_echo_major_v(echo_phase_output: Vec<String>, V: String, medium: String) -> (usize, Vec<String>)
{
    let mut count: usize = 0;

    let val: &str = V.as_str();

    let mut pi: Vec<String> = Vec::new();

    if medium.clone()=="prod_init"
    {
        for output in echo_phase_output
        {
            let split_output: Vec<&str> = output.split(", ").collect();
    
            if split_output[1].contains(&val.clone())
            {
                count+=1;
    
                pi.push(split_output[0].to_string());
            }
        }
    
        return (count, pi);
    }
    else 
    {
        let val = echo_phase_output[1].clone();
        pi.push(val);
        return (1, pi);
    }

    

}

#[allow(non_snake_case)]
fn check_other_major(mut forward_output: Vec<String>, V: String, medium: String) -> bool
{
    if medium.clone()=="prod_init"
    {
        for output in forward_output
        {
            let split_output: Vec<&str> = output.split(", ").collect();

            if !split_output[0].contains(&V)
            {
                return false;
            }
        }
        return true;
    }
    else 
    {
        forward_output.pop().expect("Vector is empty.");
        
        let split_output: Vec<&str> = forward_output[0].split(" ").collect();

        for output in split_output.clone()
        {
            if output!=V
            {
                return false;
            }
        }
        return true;
    }
}

#[allow(non_snake_case)]
pub async fn gba(committee_id: u32, ip_address: Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, mut V: String, medium: String, mode: String, types: String, committee_length: usize) -> (String, usize)
{
    println!("{}", committee_id);
    let own_signature = args[6].clone().to_string();

    let mut W: Vec<(String, String)> = Vec::new();
    let mut C1: Vec<(String, String)> = Vec::new();
    let mut C2: Vec<(String, String)> = Vec::new();

    let mut g: usize = 0;

    let mut sent: bool = false;

    let b = committee_length/2;

    let echo = generic::Echo::create_echo("".to_string(), V.to_string());
    let echo_vec = echo.to_vec();

    // let echo_phase_output = gba_communication(committee_id, ip_address.clone(), level, port_count, _index, 
    // args.clone(), echo_vec, medium.clone(), mode.clone(), types.clone()).await;

    
    // let (count, pi) = check_echo_major_v(echo_phase_output.clone(), V.clone(), medium.clone());
   
    
    // if count > b
    // {
    //     let tuples: Vec<(String, String)> = pi
    //     .iter()
    //     .map(|ip| (ip.clone(), V.clone()))
    //     .collect();
    
    //     W = tuples;
    // }

    // let mut forward_output: Vec<String> = Vec::new();
    // if W.len()>0
    // {
    //     let (pi_val, v): (String, String) = W[0].clone();

    //     let mut W_vec: Vec<String> = Vec::new();

    //     W_vec.push([pi_val, v].join(" "));

    //     forward_output = gba_communication(committee_id, ip_address.clone(), level, port_count+250, _index, 
    //         args.clone(), W_vec, medium.clone(), mode.clone(), types.clone()).await;
        
    //     sent = true;
    // }

    let mut first_vote_output: Vec<String> = Vec::new();

    // if sent==true
    // {        
    //     let check = check_other_major(forward_output.clone(), V.clone(), medium.clone());

    //     if check==true
    //     {
    //         let vote1 = generic::Vote::create_vote("".to_string(), V.to_string());
    //         let vote1_vec = vote1.to_vec();

    //         first_vote_output = gba_communication(committee_id, ip_address.clone(), level, port_count+300, _index, 
    //             args.clone(), vote1_vec.clone(), medium.clone(), mode.clone(), types.clone()).await;
    //     }
    // }

    if first_vote_output.len() >=b
    {

        if medium.clone()=="prod_init"
        {
            for output in first_vote_output
            {
                let split_output: Vec<&str> = output.split(", ").collect();
                C1.push((split_output[0].to_string(), split_output[1].to_string()));

            }
        }
        else 
        {
            C1.push((first_vote_output[0].to_string(), first_vote_output[1].to_string()));
        }
    }

    let mut second_vote_output: Vec<String> = Vec::new();

    // if C1.len() >0
    // {
    //     let (_, val): (String, String) = C1[0].clone();

    //     let value = [own_signature, val].join(", ");

    //     let vote2 = generic::Vote::create_vote("".to_string(), value.to_string());
    //     let vote2_vec = vote2.to_vec();


    //     second_vote_output = gba_communication(committee_id, ip_address.clone(), level, port_count+350, _index, 
    //         args.clone(), vote2_vec.clone(), medium.clone(), mode.clone(), types.clone()).await;

    // }

    // if second_vote_output.len()>=b
    // {
    //     if medium.clone()=="prod_init"
    //     {
    //         for output in second_vote_output
    //         {
    //             let split_output: Vec<&str> = output.split(", ").collect();
    //             C2.push((split_output[1].to_string(), split_output[2].to_string()));

    //         }
    //     }
    //     else 
    //     {
    //         let split_output: Vec<&str> = second_vote_output[1].split(", ").collect();

    //         C2.push((split_output[0].to_string(), split_output[1].to_string()));
    //     }
    // }
    
    
    // if C1.len()>0
    // {
    //     let (_, v1_prime) =  C1[0].clone();

    //     let (_, v2_prime) =  C2[0].clone();


    //     if v1_prime==v2_prime
    //     {
    //         g =1;
    //         V = v1_prime;
    //     }
    // }

    return (V, g);

}