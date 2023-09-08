
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::OpenOptions;

use tokio::sync::mpsc::{Receiver, Sender};

use crate::message::{NetworkMessage, ConsensusMessage, *};

use std::net::SocketAddr;

use std::collections::HashMap;

use tokio::time::sleep;
use tokio::time::Duration;

#[path = "../networking/communication.rs"]
mod communication;

#[path = "../types/generic.rs"]
mod generic; 


#[path = "../types/accum.rs"]
mod accum;


#[path = "../algos/byzar.rs"]
mod byzar;
#[path = "../algos/gba.rs"]
mod gba;

#[path = "./timer.rs"]
mod timer; 

#[path = "./deliver.rs"]
mod deliver;

#[path = "../merkle_tree/merkle_tree.rs"]
mod merkle_tree;

#[path = "../algos/pvss_agreement.rs"]
mod pvss_agreement;

#[path = "../types/codeword.rs"]
mod codeword;

#[path = "../networking/newclient.rs"]
mod newclient;

#[path ="../networking/newserver.rs"]
mod newserver;


fn set_state(ip_address: Vec<&str>) -> InternalState
{
    let mut sockets: Vec<SocketAddr> = Vec::new();

    let mut port = 7000;

    
    for ip_str in ip_address.clone()
    {
        let splitted_ip: Vec<&str> = ip_str.split("-").collect();
        port+=splitted_ip.clone()[0].parse::<u32>().unwrap();

        let ip_with_port = format!("{}:{}", splitted_ip[1], port.to_string()); 

        sockets.push(ip_with_port.parse::<SocketAddr>().unwrap());

        port = 7000;
    }
    

    let length = ip_address.len();

    let level_f = (length as f64).sqrt();

    let level = level_f.round() as usize;

    let state = InternalState
    {
        level: level, 
        addresses: sockets
    };

    state
}


fn split_vec_recursively<'a>(input: &[&'a str], left_half: &mut Vec<Vec<&'a str>>, right_half: &mut Vec<Vec<&'a str>>)
{
    let length = input.len();

    if length == 2 {
        return;
    }

    let mid = input.len() / 2;
    let left_slice = &input[..mid];
    let right_slice = &input[mid..];

    left_half.push(left_slice.to_vec());
    right_half.push(right_slice.to_vec());

    split_vec_recursively(left_slice, left_half, right_half);
    split_vec_recursively(right_slice, left_half, right_half);
}

fn reactor_init(pvss_data: Vec<u8>, ip_address: Vec<&str>) -> (String, InternalState)
{
    let committee_length = ip_address.len();    

    let leaves = pvss_agreement::encoder(pvss_data.clone(), committee_length.clone());
    // create accum value
    let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

    let acc_value_zl = merkle_tree::get_root(merkle_tree.clone());

    let state = set_state(ip_address) ;

    (acc_value_zl, state)

}



fn accum_init(acc_value_zl: String, ip_address: Vec<&str>, args: Vec<String>) -> NetworkMessage
{
    let accum: Accum = Accum::create_accum("sign".to_string(), acc_value_zl.clone());

    let accum_consensus_message: ConsensusMessage = ConsensusMessage::AccumMessage(accum);


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

    let accum_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
        addresses: sockets, message: accum_consensus_message, level: level
    };

    accum_network_message

}

#[allow(non_snake_case)]
async fn accum_helper(accum_value: Vec<String>, level: usize, committee_length: usize) -> (String, String)
{
    let mut V1_vec: Vec<String> = Vec::new();
    let mut V2_vec: Vec<String> = Vec::new();
    

    let file_path = "./updatednodeinfo.txt";


    for val in accum_value.clone() 
    {
        let file = OpenOptions::new().read(true).open(file_path).await.unwrap();
        let reader = BufReader::new(file);
        let mut line_stream = reader.lines();
        let val_clone = val.clone();
        let data_stream: Vec<&str> = val.split(" ").collect();

        let ipdetails = data_stream[1].clone();        

        let ip_port_split: Vec<&str> = ipdetails.split(":").collect();


        let default_port: u32 = 7000;

        let mut count = ip_port_split[1].parse::<u32>().unwrap() - default_port;

        
        while let Some(line_result) = line_stream.next_line().await.unwrap() 
        {
            let line1 = line_result;

            count-=1;


            if count==0 
            {
                let substrings: Vec<&str> = line1.split(" ").collect();
                let level_usize = level as usize;

                if substrings[level_usize + 1].contains("l")
                {
                    V1_vec.push(val_clone.clone());
                }
                else 
                {
                    V2_vec.push(val_clone.clone());
                }

                break;
            }
        }

    }


    // Get majority accum value
    let V1 = accum::accum_check(V1_vec.clone(), committee_length.clone());

    let V2 = accum::accum_check(V2_vec.clone(), committee_length.clone());

    

    (V1, V2)


}


fn codeword_init( 
    ip_address: Vec<&str>, _level: usize, args: Vec<String>, 
    value: String, merkle_len: usize, codeword_vec: Vec<String>, witnesses_vec: Vec<Vec<u8>>, part: usize) -> Vec<NetworkMessage>
{

    let mut index = 0;
    let mut network_vec: Vec<NetworkMessage> =  Vec::new();

    let mut count=0;
    
    for witness in witnesses_vec
    {
        let subset_ip: &str;
        if ip_address.clone().len()==1
        {
            subset_ip = ip_address.clone()[0];
        }
        else {
            subset_ip = ip_address.clone()[index];

        }
        let mut subset_vec: Vec<&str> = Vec::new();
        subset_vec.push(subset_ip);
        let mut leaf_values_to_prove = codeword_vec[index].to_string();

        
        let indices_to_prove = index.clone().to_string();
        leaf_values_to_prove = leaf_values_to_prove.replace(",", ";");

        let codeword = Codeword::create_codeword("".to_string(), leaf_values_to_prove.clone(), witness.clone(), 
        value.to_string(), indices_to_prove.clone(), merkle_len, part);
        index+=1;

        
        let codeword_consensus_message: ConsensusMessage = ConsensusMessage::CodewordMessage(codeword);


        let mut port = 7000;

        let mut sockets: Vec<SocketAddr> = Vec::new();

        let ip_str = ip_address.clone()[count];

        count+=1;

        
        let splitted_ip: Vec<&str> = ip_str.split("-").collect();

        port+=splitted_ip.clone()[0].parse::<u32>().unwrap();

        let ip_with_port = format!("{}:{}", splitted_ip[1], port.to_string()); 

        sockets.push(ip_with_port.parse::<SocketAddr>().unwrap());
    
        

        let senderport = 7000 + args[2].parse::<u32>().unwrap();
        let sender_str = format!("{}:{}", args[6], senderport.to_string());


        let length = ip_address.len();

        let level_f = (length as f64).sqrt();

        let level = level_f.round() as usize;


        let codeword_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
            addresses: sockets, message: codeword_consensus_message, level: level
        };

        network_vec.push(codeword_network_message)

           
    }

    network_vec
    
}

#[allow(non_snake_case)]
async fn codeword_helper(tx_sender: Sender<NetworkMessage>, ip_address: Vec<&str>, codewords: String, witness: Vec<u8>, 
    value: String, index: String, leaves_len: usize, part: usize, args: Vec<String>, mut check_first_codeword_list: Vec<String>)
    -> (String, Vec<String>)
{
    let mut data: String = "pvss".to_string();
   

    if ip_address.len()==2
    {
        let bytes = codewords.trim_matches('[').trim_matches(']').split("; ");

        // Parse each substring as u8 and collect into a vector
        let bytes: Vec<u8> = bytes.map(|s| s.parse().unwrap()).collect();

        // Decode the vector as UTF-8 and handle errors
        let output = match std::str::from_utf8(&bytes) {
            Ok(s) => s,
            Err(_) => {
                // Handle invalid UTF-8 error
                return (data, check_first_codeword_list);
            }
        };

        data  = output.to_string();


        return (data, check_first_codeword_list);

    }
    

    if !check_first_codeword_list.contains(&value)
    {
        let (proof, codeword) = codeword::verify_codeword(codewords.clone(), witness, value.clone(), index, leaves_len);

        if proof==true
        {
            check_first_codeword_list.push(value.clone());

            // send witness to nodes if have received the first valid code word

            let codeword_retrieve = CodewordRetrieve::create_codeword_retrieve("sign".to_string(), codeword, part); 

            let codeword_retrieve_message: ConsensusMessage = ConsensusMessage::CodewordRetrieveMessage(codeword_retrieve);


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
    
    
            let codewordretrieve_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
                addresses: sockets, message: codeword_retrieve_message, level: level
            };
    
            let _ = tx_sender.send(codewordretrieve_network_message).await;

        
        }
    }
    (data, check_first_codeword_list)

}

fn codeword_retrieve(retrieved_hashmap: HashMap<usize, HashMap<SocketAddr, String>>, committee_length: usize) -> HashMap<usize, Vec<u8>>
{  
    let mut pvss_hashmap: HashMap<usize, Vec<u8>> = HashMap::new();
    
    
    for (i, map) in retrieved_hashmap
    {        
        let mut values: HashMap<usize, String> = HashMap::new();

        for (j, value) in map
        {
            let splitted: Vec<String> = j.to_string().split(":").map(|s| s.to_owned()).collect(); 

            let ordered_str = &splitted.clone()[1]; 

            let order = ordered_str.parse::<u32>().unwrap() - 7000;

            let usize_order: usize = order as usize;

            values.insert(usize_order, value);
        } 

        let mut sorted_pairs: Vec<(&usize, &String)> = values.iter().collect();

        // Sort the vector based on keys (usize in this case).
        sorted_pairs.sort_by(|(a_key, _), (b_key, _)| a_key.cmp(b_key));


        let mut codeword_vec: Vec<Vec<u8>> = Vec::new();

        for (_, val) in sorted_pairs
        {
            let codeword_str = val.trim_start_matches('[').trim_end_matches(']');
    
            // Split the string by commas, and then parse each part into u8.
            let codeword: Vec<u8> = codeword_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            codeword_vec.push(codeword);
        }
                    
        let pvss = pvss_agreement::decode(codeword_vec, committee_length);

        pvss_hashmap.insert(i, pvss.into_bytes());    
    }

    pvss_hashmap
}


fn committee_init( 
    ip_address: Vec<&str>, args: Vec<String>, 
    value: String, merkle_len: usize, codeword_vec: Vec<String>, witnesses_vec: Vec<Vec<u8>>, part: usize) -> Vec<NetworkMessage>
{

    let mut index = 0;
    let mut network_vec: Vec<NetworkMessage> =  Vec::new();

    let mut count=0;
    
    for witness in witnesses_vec
    {
        let subset_ip: &str;
        if ip_address.clone().len()==1
        {
            subset_ip = ip_address.clone()[0];
        }
        else {
            subset_ip = ip_address.clone()[index];

        }
        let mut subset_vec: Vec<&str> = Vec::new();
        subset_vec.push(subset_ip);
        let mut leaf_values_to_prove = codeword_vec[index].to_string();

        
        let indices_to_prove = index.clone().to_string();
        leaf_values_to_prove = leaf_values_to_prove.replace(",", ";");

        let committee = Committee::create_committee("".to_string(), leaf_values_to_prove.clone(), witness.clone(), 
        value.to_string(), indices_to_prove.clone(), merkle_len, part);
        index+=1;

        
        let committee_consensus_message: ConsensusMessage = ConsensusMessage::CommitteeMessage(committee);


        let mut port = 7000;

        let mut sockets: Vec<SocketAddr> = Vec::new();

        let ip_str = ip_address.clone()[count];

        count+=1;

        let splitted_ip: Vec<&str> = ip_str.split("-").collect();

        port+=splitted_ip.clone()[0].parse::<u32>().unwrap();

        let ip_with_port = format!("{}:{}", splitted_ip[1], port.to_string()); 

        sockets.push(ip_with_port.parse::<SocketAddr>().unwrap());

        

        let senderport = 7000 + args[2].parse::<u32>().unwrap();
        let sender_str = format!("{}:{}", args[6], senderport.to_string());


        let length = ip_address.len();

        let level_f = (length as f64).sqrt();

        let level = level_f.round() as usize;


        let codeword_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
            addresses: sockets, message: committee_consensus_message, level: level
        };

        network_vec.push(codeword_network_message)

           
    }

    network_vec
    
}



#[allow(non_snake_case)]
async fn committee_selection(tx_sender: Sender<NetworkMessage>, mut qual: Vec<u32>, 
    pvss_data: HashMap<usize, Vec<u8>>, ip_address: Vec<&str>, args: Vec<String>)
{   
    let mut b: Vec<u32> = Vec::new();

    b.push(1);
    b.push(2);

    let W1 = pvss_data.get(&1).unwrap();
    let W2 = pvss_data.get(&2).unwrap();
    
    if qual.contains(&1)
    {
        // //2BA for W1
        // let v1 = byzar::BA(committee_id, ip_address, level, _index, args.clone(),
        //     W1.clone(), mode.clone(), committee_length.clone()).await;
        // // update b
        // if byzar::twoBA(v1).await
        // {
        //     b.push(1);
        // }

    }
    if qual.contains(&2)
    {
        // //2BA for W2
        // let v2 = byzar::BA( committee_id, ip_address, level, _index, args.clone(), 
        // W2.clone(), mode.clone(), committee_length.clone()).await;
        // // update b
        // if byzar::twoBA(v2).await
        // {
        //     b.push(2);
        // }
    }
    qual.retain(|&x| b.contains(&x));



    for val in qual.clone()
    {   
        if val==1 && W1.len()!=0
        {   
            let (codeword_vec, witnesses_vec, merkle_len) = 
                deliver::deliver_encode(W1.clone(), "_".to_string(), 
            ip_address.clone().len());          

            let leaves = pvss_agreement::encoder(W1.clone(), ip_address.clone().len());
            // create accum value
            let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

            let acc_value_zl_W1 = merkle_tree::get_root(merkle_tree.clone());

            
            let network_vec = committee_init( 
                ip_address.clone(), args.clone(), 
                acc_value_zl_W1.clone(), merkle_len, codeword_vec, witnesses_vec, 1);


                for network_msg in network_vec
                {   
                    let _  = tx_sender.send(network_msg).await;
                }

        }

        if val==2 && W2.len()!=0
        {                                  
            let (codeword_vec, witnesses_vec, merkle_len) = 
                deliver::deliver_encode(W2.clone(), "_".to_string(), 
            ip_address.clone().len());
            
            
            let leaves = pvss_agreement::encoder(W2.clone(), ip_address.clone().len());
            // create accum value
            let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

            let acc_value_zl_W2 = merkle_tree::get_root(merkle_tree.clone());


            let network_vec = committee_init( 
                ip_address.clone(), args.clone(), 
                acc_value_zl_W2.clone(), merkle_len, codeword_vec, witnesses_vec, 2);

            for network_msg in network_vec
            {   
                let _  = tx_sender.send(network_msg).await;
            }

        }
    }


}


async fn forward_helper(tx_sender: Sender<NetworkMessage>, ip_address: Vec<&str>, args: Vec<String>, v: String, no: usize)
{
    let vote = Vote::create_vote("".to_string(), no, v);
    
    let vote_consensus_message: ConsensusMessage = ConsensusMessage::VoteMessage(vote);


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

    let vote_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
        addresses: sockets, message: vote_consensus_message, level: level
    };


    let _ = tx_sender.send(vote_network_message).await;
}


async fn propose_helper(tx_sender: Sender<NetworkMessage>, ip_address: Vec<&str>, args: Vec<String>, v: String)
{
    let propose = Propose::create_propose("".to_string(), v);
    
    let propose_consensus_message: ConsensusMessage = ConsensusMessage::ProposeMessage(propose);


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

    let propose_network_message = NetworkMessage{sender: sender_str.parse::<SocketAddr>().unwrap(),
        addresses: sockets, message: propose_consensus_message, level: level
    };


    let _ = tx_sender.send(propose_network_message).await;
}

fn aggregate(mut updated_pvss: Vec<String>) -> Vec<u8>
{
    updated_pvss.sort();

    let pvss = updated_pvss.join("");

    return pvss.into_bytes();
}

#[allow(non_snake_case)]
pub async fn reactor(tx_sender: Sender<NetworkMessage>, mut rx: Receiver<NetworkMessage>, sorted: Vec<(&u32, &String)>, args: Vec<String>)
{  
    let mut level = 0;

    let (_, mut ip_addresses_comb) = sorted[level];
    let mut ip_address: Vec<&str> = ip_addresses_comb.split(" ").collect(); 

    let mut pvss_data: Vec<u8> = "".to_string().into_bytes();

    let mut qual: Vec<u32> = Vec::new();

    let mut state: InternalState;

    let mut storage: HashMap<usize, HashMap<SocketAddr, String>> = HashMap::new();

    let mut retrieved_hashmap: HashMap<usize, HashMap<SocketAddr, String>> = HashMap::new();

    if ip_address.len()==1
    {
        //GET PVSS DATA FROM DIMITRIS
        pvss_data = ["pvss".to_string(), args[2].to_string()].join(" ").into_bytes();
        level+=1;
        
    }


    (_, ip_addresses_comb) = sorted[level];


    ip_address = ip_addresses_comb.split(" ").collect();

    let mut acc_value_zl: String;
            
    (acc_value_zl, state) = reactor_init(pvss_data.clone(), ip_address.clone());

    let accum_network_message = accum_init(acc_value_zl.clone(), ip_address.clone(), args.clone());

    let _ = tx_sender.send(accum_network_message).await;

    let (mut V1, mut V2): (String, String) = ("".to_string(), "".to_string());


    let mut accum_value: Vec<String> = Vec::new();

    let mut echo_value: Vec<String> = Vec::new();

    let mut updated_pvss: Vec<String> = Vec::new();
  

    let mut forward_value: Vec<String> = Vec::new();

    let mut vote1_value: Vec<String> = Vec::new();
    let mut vote2_value: Vec<String> = Vec::new();

    let mut propose_value: Vec<String> = Vec::new();

    let mut flag = 0;

    let mut forward_check = false;

    let mut C1: Vec<(String, String)> = Vec::new();
    let mut C2: Vec<(String, String)> = Vec::new();

    let mut BA_V: String = "bot".to_string();
    let mut g: usize = 0;

    let mut ip_address_left: Vec<Vec<&str>>  = Vec::new();
    let mut ip_address_right: Vec<Vec<&str>>  = Vec::new();

    let mut ip_address_backup: Vec<&str> = Vec::new();


    let mut check_first_codeword_list: Vec<String> = Vec::new();

    loop 
    {
        if let Some(message) = rx.recv().await {
            match message.message 
            {               
                // Match the Echo message type
                ConsensusMessage::EchoMessage(echo) => {
                    // Handle Echo message
                    // println!("received echo {:?}", message.sender);

                    let value = format!("{} {}", echo.value, message.sender);

                    
                    echo_value.push(value);
                    

                    

                    let (count, pi): (usize, Vec<String>);

                    if echo_value.len()==ip_address.clone().len()
                    {   
                        let V = format!("{}-{}", V1.clone(), V2.clone());
                        (count, pi) = gba::check_echo_major_v(echo_value.clone(), V.clone());

                        echo_value = Vec::new(); 
                       
                        forward_check = gba::forward_phase(tx_sender.clone(), count, pi, 
                            ip_address.clone(), args.clone()).await;
                        
                    }

                    


                }

                // Match the Forward message type
                ConsensusMessage::ForwardMessage(forward) => {
                    // Handle Forward message
                    // println!("received forward, {:?}", message.sender);

                    let value = format!("{} {}", forward.value,  message.sender);

                    // sleep(Duration::from_millis(30)).await;                    

                    forward_value.push(value);                    

                    if forward_value.len()==ip_address.clone().len() 
                    { 

                        let forward_value_copy = forward_value.clone();

                        let first_string_parts: Vec<&str> = forward_value_copy[0].split(' ').collect();
                        let first_part_to_compare = first_string_parts[0];

                        // Check if the first part of all strings matches the first part of the first string
                        let all_parts_match = forward_value.iter().all(|string| {
                            let parts: Vec<&str> = string.split(' ').collect();
                            let first_part = parts[0];
                            first_part == first_part_to_compare
                        });

                        forward_value = Vec::new(); 

                        if all_parts_match && forward_check{
                            forward_helper(tx_sender.clone(), ip_address.clone(), args.clone(), first_part_to_compare.to_string(), 1).await;
                        }
                    }


                    
                }

                // Match the Vote message type
                ConsensusMessage::VoteMessage(vote) => {
                    // Handle Vote message
                    // println!("received vote, {:?}", message.sender);

                    let value = format!("{} {}", vote.value,  message.sender);
                   
                    sleep(Duration::from_millis(30)).await;

                    if vote.no==1
                    {
                        vote1_value.push(value);
                    }
                    else
                    {
                        vote2_value.push(value);
                    }                    


                    if vote1_value.len()==ip_address.clone().len()/2 + 1 //vote phase
                    { 
                        

                        for output in vote1_value
                        {
                            let split_output: Vec<&str> = output.split(" ").collect();

                            C1.push((split_output[0].to_string(), split_output[1].to_string()));

                        }

                        sleep(Duration::from_millis(30)).await;                 

                        vote1_value = Vec::new();       

                        if C1.len() >0 //second vote phase
                        {   
                            let (v, _) = &C1[0];
                            forward_helper(tx_sender.clone(), ip_address.clone(), args.clone(), v.to_string(), 2).await;

                        }
                    }

                    if vote2_value.len()==ip_address.clone().len()/2 + 1 //second vote phase    
                    {
                        for output in vote2_value
                        {
                            let split_output: Vec<&str> = output.split(" ").collect();

                            C2.push((split_output[0].to_string(), split_output[1].to_string()));

                        }

                        sleep(Duration::from_millis(30)).await;

                        vote2_value = Vec::new();

                        if C1.len()>0 // output generation
                        {
                            let (v1_prime, _) =  C1[0].clone();

                            let (v2_prime, _) =  C2[0].clone();


                            if v1_prime==v2_prime
                            {
                                g =1;
                                BA_V = v1_prime;
                            }
                        }
                        

                        sleep(Duration::from_millis(30)).await;

                        let _ = propose_helper(tx_sender.clone(), ip_address.clone(), args.clone(), BA_V.clone()).await;

                        
                        
                    }   
                        
                    
                }

                // Match the Committee message type
                ConsensusMessage::CommitteeMessage(committee) => 
                {   
                    // println!("received committee, {:?}", message.sender);
                    // Handle Committee message
                    sleep(Duration::from_millis(30)).await;
                    (_, check_first_codeword_list) = codeword_helper(tx_sender.clone(), ip_address.clone(), committee.codewords, committee.witness, 
                    committee.value, committee.index, committee.leaves_len, committee.part, args.clone(), check_first_codeword_list.clone()).await;
                }

                ConsensusMessage::CodewordRetrieveMessage(retrieve) =>
                {   
                    // println!("received cordwordretrieve, {:?}", message.sender);
                    // Handle Retrieve message
                    retrieved_hashmap
                    .entry(retrieve.part)
                    .or_insert_with(HashMap::new)
                    .insert(message.sender, retrieve.codewords);

                    let mut total_length = 0;

                    for (_, inner_map) in &retrieved_hashmap {
                        for _ in inner_map.values() {
                            total_length += 1
                        }
                    }


                    if flag==0
                    {
                        if total_length == 2*ip_address.clone().len()
                        {     sleep(Duration::from_millis(30)).await;
                            flag = 1;

                            let pvss_vec = codeword_retrieve(retrieved_hashmap.clone(), 
                                ip_address.clone().len());

                            total_length=0;

                            check_first_codeword_list = Vec::new();
                            
                            retrieved_hashmap = HashMap::new();

                            committee_selection(tx_sender.clone(), qual.clone(), pvss_vec.clone(), ip_address.clone(), args.clone()).await;
                        
                            
                        }
                    }
                    if flag == 1
                    {   sleep(Duration::from_millis(30)).await;
                        if total_length == 2*ip_address.clone().len() 
                        {   
                            flag = 0;
                           
                            
                            let pvss_vec = codeword_retrieve(retrieved_hashmap.clone(), 
                                ip_address.clone().len());

                            retrieved_hashmap = HashMap::new();

                            let mut temp: Vec<String> = Vec::new();

                            for (_, map) in pvss_vec
                            {
                                temp.push(String::from_utf8(map).unwrap());
                            }
    
                            pvss_data = aggregate(temp.clone());
    
                            println!("{:?}, {:?}", pvss_data, String::from_utf8(pvss_data.clone()));


                            if sorted.clone().len()>level+1
                            {   
                                level+=1;

                                (_, ip_addresses_comb) = sorted[level];

                                ip_address = ip_addresses_comb.split(" ").collect();
                                        
                                (acc_value_zl, state) = reactor_init(pvss_data.clone(), ip_address.clone());
                            
                                
                                let accum_network_message = accum_init(acc_value_zl.clone(), ip_address.clone(), args.clone());
                            
                                let _ = tx_sender.send(accum_network_message).await;
                            }
                            else 
                            {
                               sleep(Duration::from_millis(50)).await;
                               return;
                            }

                            
                        }
                    }
                                       
                }


                // Match the Codeword message type
                ConsensusMessage::CodewordMessage(codeword) => 
                {
                    // Handle Codeword message
                    // println!("received codeword, {:?}", message.sender);
                    let data: String;

                    (data, check_first_codeword_list) = codeword_helper(tx_sender.clone(), ip_address.clone(), codeword.codewords, codeword.witness, 
                        codeword.value, codeword.index, codeword.leaves_len, codeword.part, args.clone(), check_first_codeword_list.clone()).await;

                    if ip_address.clone().len()==2
                    {
                        updated_pvss.push(data);

                        if updated_pvss.len()==ip_address.clone().len()
                        {                      
                            pvss_data = aggregate(updated_pvss.clone());

                            updated_pvss = Vec::new();
                        
                            println!("{:?}, {:?}", pvss_data, String::from_utf8(pvss_data.clone()));
    
                            
                            level+=1;

                            (_, ip_addresses_comb) = sorted[level];

                            ip_address = ip_addresses_comb.split(" ").collect();
                                    
                            (acc_value_zl, state) = reactor_init(pvss_data.clone(), ip_address.clone());
                        
                            
                            let accum_network_message = accum_init(acc_value_zl.clone(), ip_address.clone(), args.clone());
                        
                            let _ = tx_sender.send(accum_network_message).await;
                        
                                
    
                        }
                    }

                    
                    
                }

                // Match the Accum message type
                ConsensusMessage::AccumMessage(accum) => 
                {
                    
                    // Handle Accum message
                    // println!("received accum, {:?}", message.sender);
                    let value = format!("{} {:?}", accum.value, message.sender);

                    if state.get_level() == message.level
                    {
                        qual = Vec::new();
                        accum_value.push(value);

                        if storage.contains_key(&state.get_level())
                        {                            
                            let stored_pair = storage.remove(&state.get_level());

                            match stored_pair {
                                Some(inner_map) => {
                                    // Inner HashMap was removed, you can access its values
                                    let values: Vec<String> = inner_map.values().cloned().collect();

                                    for value in values
                                    {
                                        accum_value.push(value);
                                    }
                                }
                                None => {
                                    println!("Key 'key1' not found in the original HashMap.");
                                }
                            }

                            
                        }
                    }
                    else 
                    {
                        let value = format!("{} {:?}", accum.value, message.sender);

                        storage.entry(message.level).or_insert_with(HashMap::new)
                            .insert(message.sender, value);
                        
                    }
                    

                    if accum_value.len()==ip_address.clone().len()
                    {
                        split_vec_recursively(&ip_address, &mut ip_address_left, &mut ip_address_right);

                        let own_ip = args[6].clone();

                        ip_address_left.retain(|inner_vec| inner_vec.contains(&&own_ip.as_str()));
                        ip_address_right.retain(|inner_vec| inner_vec.contains(&&own_ip.as_str()));
                    
                        ip_address_backup = ip_address.clone();

                        C1 = Vec::new();
                        C2 = Vec::new();


                        (V1, V2) = accum_helper(accum_value.clone(), level.clone(), 
                            ip_address.clone().len()).await;

                        let V = format!("{}-{}", V1, V2);

                        if level!=1
                        {
                            byzar::BA_setup(tx_sender.clone(), ip_address.clone(),  args.clone(),
                                V.clone()).await;
                        }
                        else 
                        {
                            if V1!="bot" && V1!=""
                            {        
                                qual.push(1);
                            }
                            if V2!="bot" && V2!=""
                            {
                                qual.push(2);
                            }

                            for val in qual.clone()
                            {   
                                if val==1 && V1==acc_value_zl
                                {   
                                let (codeword_vec, witnesses_vec, merkle_len) = 
                                        deliver::deliver_encode(pvss_data.clone(), V1.clone(), 
                                    ip_address.clone().len());


                                    let network_vec = codeword_init( 
                                        ip_address.clone(), level, args.clone(), 
                                        V1.clone(), merkle_len, codeword_vec, witnesses_vec, 1);


                                    for network_msg in network_vec
                                    {   
                                        let _  = tx_sender.send(network_msg).await;
                                    }

                                }

                                if val==2 && V2==acc_value_zl
                                {                                  
                                let (codeword_vec, witnesses_vec, merkle_len) = 
                                        deliver::deliver_encode(pvss_data.clone(), V2.clone(), 
                                    ip_address.clone().len());
                                    
                                    
                                    let network_vec = codeword_init( 
                                        ip_address.clone(), level, args.clone(), 
                                        V2.clone(), merkle_len, codeword_vec, witnesses_vec, 2);

                                    
                                    for network_msg in network_vec
                                    {   
                                        let _  = tx_sender.send(network_msg).await;
                                    }
                                }
                            }
                        }
                                                                                       
                        
                        accum_value = Vec::new();
                    }
                    
                }


                // Match the Propose message type
                ConsensusMessage::ProposeMessage(propose) => {
                    // Handle Propose message
                    // println!("received propose, {:?}", message.sender);

                    let value = format!("{} {}", propose.value,  message.sender);
                   
                    sleep(Duration::from_millis(30)).await;

                    propose_value.push(value); 


                    if propose_value.len() == ip_address.clone().len()/2 + 1
                    {                        
                        println!("{:?}, {:?}, {:?}", g, BA_V, propose_value);

                        propose_value = Vec::new();
                        
                        //run BA
                        if ip_address_left.len()>0
                        {
                            ip_address = ip_address_left[0].clone();

                            ip_address_left.remove(0);
                            println!("{:?}", ip_address);
                            byzar::BA_setup(tx_sender.clone(), ip_address.clone(),  args.clone(),
                                BA_V.clone()).await;
                        }
                        else if ip_address_right.len()>0
                        {
                            ip_address = ip_address_right[0].clone();

                            ip_address_right.remove(0);
                            println!("{:?}", ip_address);
                            byzar::BA_setup(tx_sender.clone(), ip_address.clone(),  args.clone(),
                                BA_V.clone()).await;
                        }                        
                        else 
                        {
                            ip_address = ip_address_backup.clone();

                            if V1!="bot" && V1!=""
                            {        
                                qual.push(1);
                            }
                            if V2!="bot" && V2!=""
                            {
                                qual.push(2);
                            }

                            for val in qual.clone()
                            {   
                                if val==1 && V1==acc_value_zl
                                {   
                                let (codeword_vec, witnesses_vec, merkle_len) = 
                                        deliver::deliver_encode(pvss_data.clone(), V1.clone(), 
                                    ip_address.clone().len());


                                    let network_vec = codeword_init( 
                                        ip_address.clone(), level, args.clone(), 
                                        V1.clone(), merkle_len, codeword_vec, witnesses_vec, 1);


                                    for network_msg in network_vec
                                    {   
                                        let _  = tx_sender.send(network_msg).await;
                                    }

                                }

                                if val==2 && V2==acc_value_zl
                                {                                  
                                let (codeword_vec, witnesses_vec, merkle_len) = 
                                        deliver::deliver_encode(pvss_data.clone(), V2.clone(), 
                                    ip_address.clone().len());
                                    
                                    
                                    let network_vec = codeword_init( 
                                        ip_address.clone(), level, args.clone(), 
                                        V2.clone(), merkle_len, codeword_vec, witnesses_vec, 2);

                                    
                                    for network_msg in network_vec
                                    {   
                                        let _  = tx_sender.send(network_msg).await;
                                    }
                                }
                            }
                        }
                    }



                }
            
                
            }
        }    
        
    }

}

