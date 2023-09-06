use crate::nodes::reactor::communication;


#[path = "../types/generic.rs"]
mod generic; 

use crate::message::{NetworkMessage, ConsensusMessage, *};
use tokio::sync::mpsc::Sender;
use std::net::SocketAddr;


async fn gba_communication1(committee_id: u32, ip_address: Vec<&str>, level: u32, _index:u32, 
    args: Vec<String>, value: Vec<String>, mode: String) -> Vec<String>
{    
    
    let output  = communication::prod_communication(committee_id, ip_address.clone(), level, _index, 
    args.clone(), value.clone(), mode.clone()).await;

    return output;


}

#[allow(non_snake_case)]
pub fn check_echo_major_v(echo_phase_output: Vec<String>, V: String) -> (usize, Vec<String>)
{
    let mut count: usize = 0;

    let val: &str = V.as_str();

    let mut pi: Vec<String> = Vec::new();
    
    for output in echo_phase_output
    {
        let split_output: Vec<&str> = output.split(" ").collect();

        if split_output[0].contains(&val.clone())
        {
            count+=1;

            pi.push(format!("{} {}", split_output[0].to_string(), split_output[2].to_string()));
        }
    }

    return (count, pi);
    
}

#[allow(non_snake_case)]
pub async fn forward_phase(tx_sender: Sender<NetworkMessage>, count: usize, pi: Vec<String>, ip_address: Vec<&str>, args: Vec<String>, part: usize) 
    -> bool
{
    let b = ip_address.clone().len()/2;

    if count >= b // forward phase
    {
        let v = pi[0].clone();

        let forward = Forward::create_forward("".to_string(), v.to_string(), part);
    
        let forward_consensus_message: ConsensusMessage = ConsensusMessage::ForwardMessage(forward);


        let mut port = 7000;

        let mut sockets: Vec<SocketAddr> = Vec::new();

        for ip_str in ip_address.clone()
        {
            let splitted_ip: Vec<&str> = ip_str.split("-").collect();

            port+=splitted_ip.clone()[0].parse::<u32>().unwrap();

            let ip_with_port = format!("{}:{}", splitted_ip[1], port.to_string()); 

            sockets.push(ip_with_port.parse::<SocketAddr>().unwrap());

            port = 7000;
        }


        let senderport = 7000 + args[2].parse::<u32>().unwrap();
        let sender_str = format!("{}:{}", args[6], senderport.to_string());

        let length = ip_address.len();

        let level_f = (length as f64).sqrt();

        let level = level_f.round() as usize;

        let echo_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
            addresses: sockets, message: forward_consensus_message, level: level
        };
        

        // let _ = tx_sender.send(echo_network_message).await;
            return true;
    }
    return false;
    
}


#[allow(non_snake_case)]
fn check_other_major(forward_output: Vec<String>, V: String) -> bool
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


#[allow(non_snake_case)]
pub async fn gba_setup(tx_sender: Sender<NetworkMessage>, ip_address: Vec<&str>, 
    args: Vec<String>, V: String, part: usize) 
    
{    
    let echo = Echo::create_echo("".to_string(), V.to_string(), part);
    
    let echo_consensus_message: ConsensusMessage = ConsensusMessage::EchoMessage(echo);


    let mut port = 7000;

    let mut sockets: Vec<SocketAddr> = Vec::new();

    for ip_str in ip_address.clone()
    {
        let splitted_ip: Vec<&str> = ip_str.split("-").collect();

        port+=splitted_ip.clone()[0].parse::<u32>().unwrap();

        let ip_with_port = format!("{}:{}", splitted_ip[1], port.to_string()); 

        sockets.push(ip_with_port.parse::<SocketAddr>().unwrap());

        port = 7000;
    }


    let senderport = 7000 + args[2].parse::<u32>().unwrap();
    let sender_str = format!("{}:{}", args[6], senderport.to_string());

    let length = ip_address.len();

    let level_f = (length as f64).sqrt();

    let level = level_f.round() as usize;

    let echo_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
        addresses: sockets, message: echo_consensus_message, level: level
    };


    let _ = tx_sender.send(echo_network_message).await;

}






#[allow(non_snake_case)]
pub async fn gba1(committee_id: u32, ip_address: Vec<&str>, level: u32, _index:u32, 
    args: Vec<String>, mut V: String, mode: String, committee_length: usize) -> (String, usize)
{

    let own_signature = args[6].clone().to_string();

    let mut W: Vec<(String, String)> = Vec::new();
    let mut C1: Vec<(String, String)> = Vec::new();
    let mut C2: Vec<(String, String)> = Vec::new();

    let mut g: usize = 0;

    let mut sent: bool = false;

    let b = committee_length/2;

    let echo = generic::Echo::create_echo("".to_string(), V.to_string());
    let echo_vec = echo.to_vec();

    let echo_phase_output = gba_communication1(committee_id, ip_address.clone(), level, _index, 
    args.clone(), echo_vec, mode.clone()).await;

    
    let (count, pi) = check_echo_major_v(echo_phase_output.clone(), V.clone());
   
    
    if count > b // forward phase
    {
        let tuples: Vec<(String, String)> = pi
        .iter()
        .map(|ip| (ip.clone(), V.clone()))
        .collect();
    
        W = tuples;
    }

    let mut forward_output: Vec<String> = Vec::new();
    if W.len()>0
    {
        let (pi_val, v): (String, String) = W[0].clone();

        let mut W_vec: Vec<String> = Vec::new();

        W_vec.push([pi_val, v].join(" "));

        forward_output = gba_communication1(committee_id, ip_address.clone(), level, _index, 
            args.clone(), W_vec, mode.clone()).await;
        
        sent = true;
    }

    let mut first_vote_output: Vec<String> = Vec::new();

    if sent==true //first vote phase
    {        
        let check = check_other_major(forward_output.clone(), V.clone());

        if check==true
        {
            let vote1 = generic::Vote::create_vote("".to_string(), V.to_string());
            let vote1_vec = vote1.to_vec();

            first_vote_output = gba_communication1(committee_id, ip_address.clone(), level, _index, 
                args.clone(), vote1_vec.clone(), mode.clone()).await;
        }
    }

    if first_vote_output.len() >=b //update C1
    {
        
        for output in first_vote_output
        {
            let split_output: Vec<&str> = output.split(", ").collect();
            C1.push((split_output[0].to_string(), split_output[1].to_string()));

        }
    
        
    }

    let mut second_vote_output: Vec<String> = Vec::new();

    if C1.len() >0 //second vote phase
    {
        let (_, val): (String, String) = C1[0].clone();

        let value = [own_signature, val].join(", ");

        let vote2 = generic::Vote::create_vote("".to_string(), value.to_string());
        let vote2_vec = vote2.to_vec();


        second_vote_output = gba_communication1(committee_id, ip_address.clone(), level, _index, 
            args.clone(), vote2_vec.clone(), mode.clone()).await;

    }

    if second_vote_output.len()>=b //update C2
    {        
        for output in second_vote_output
        {
            let split_output: Vec<&str> = output.split(", ").collect();
            C2.push((split_output[1].to_string(), split_output[2].to_string()));

        }
        
        
    }
    
    
    if C1.len()>0 // output generation
    {
        let (_, v1_prime) =  C1[0].clone();

        let (_, v2_prime) =  C2[0].clone();


        if v1_prime==v2_prime
        {
            g =1;
            V = v1_prime;
        }
    }

    return (V, g);

}