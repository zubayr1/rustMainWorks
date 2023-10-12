
use crate::message::{NetworkMessage, ConsensusMessage, *};
use tokio::sync::mpsc::Sender;
use std::net::SocketAddr;


#[allow(unused)]
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

            pi.push(split_output[0].to_string());
        }
    }

    return (count, pi);
    
}

#[allow(unused)]
#[allow(non_snake_case)]
pub async fn forward_phase(tx_sender: Sender<NetworkMessage>, count: usize, pi: Vec<String>, ip_address: Vec<&str>, args: Vec<String>, level: usize) 
    -> bool
{
    let b = ip_address.clone().len()/2;

    if count >= b // forward phase
    {
        let v_split: Vec<String> = pi[0]
            .clone()
            .split(" ")
            .map(|s| s.to_string())
            .collect();

        let forward = Forward::create_forward("".to_string(), v_split[0].clone());
    
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

        
        let forward_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
            addresses: sockets, message: forward_consensus_message, level: level
        };

        let _ = tx_sender.send(forward_network_message).await;
            return true;
    }
    return false;
    
}




#[allow(unused)]
#[allow(non_snake_case)]
pub async fn gba_setup(tx_sender: Sender<NetworkMessage>, ip_address: Vec<&str>, 
    args: Vec<String>, V: String, level: usize) 
    
{    println!("       LEVEL {}", level);
    let echo = Echo::create_echo("".to_string(), V.to_string());
    
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

   
    let echo_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
        addresses: sockets, message: echo_consensus_message, level: level
    };


    let _ = tx_sender.send(echo_network_message).await;

}





