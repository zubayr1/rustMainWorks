use async_recursion::async_recursion;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs::OpenOptions;

use tokio::sync::mpsc::Receiver;

use crate::message::{NetworkMessage, ConsensusMessage};


#[path = "../networking/communication.rs"]
mod communication;

#[path = "../types/generic.rs"]
mod generic; 


#[path = "../types/accum.rs"]
mod accum;


#[path = "../algos/byzar.rs"]
mod byzar;

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

async fn communication(
    committee_id: u32, ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, mode: String,
    value: Vec<String>) -> Vec<String>
{    
    let output = communication::prod_communication(committee_id, ip_address.clone(), level, 
        _index, args.clone(), value.clone(), mode.clone()).await;

    return output;
}


fn reactor_init(pvss_data: Vec<u8>, ip_address: Vec<&str>) -> String
{
    let committee_length = ip_address.len();    

    let leaves = pvss_agreement::encoder(pvss_data.clone(), committee_length.clone());
    // create accum value
    let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

    let acc_value_zl = merkle_tree::get_root(merkle_tree.clone());

    acc_value_zl

}



pub async fn reactor(mut rx: Receiver<NetworkMessage>, sorted: Vec<(&u32, &String)>, args: Vec<String>)
{
    let mut level = 0;

    let (mut committee_id, mut ip_addresses_comb) = sorted[level];
    let mut ip_address: Vec<&str> = ip_addresses_comb.split(" ").collect(); 

    let mut pvss_data: Vec<u8> = "".to_string().into_bytes();


    if ip_address.len()==1
    {
        //GET PVSS DATA FROM DIMITRIS
        pvss_data = ["pvss_datapvss_data".to_string(), args[2].to_string()].join(" ").into_bytes();
        level+=1
    }


    (committee_id, ip_addresses_comb) = sorted[level];

    ip_address = ip_addresses_comb.split(" ").collect();
            
    let acc_value_zl = reactor_init(pvss_data, ip_address);

    let accum = generic::Accum::create_accum("sign".to_string(), acc_value_zl.clone());
    let accum_vec = accum.to_vec();

    let accum_consensus_message: ConsensusMessage = ConsensusMessage::AccumMessage(accum);

    loop 
    {
        if let Some(message) = rx.recv().await {
            match message.message 
            {                
                // Match the Echo message type
                ConsensusMessage::EchoMessage(echo) => {
                    // Handle Echo message
                    println!("received echo");
                }

                // Match the Vote message type
                ConsensusMessage::VoteMessage(vote) => {
                    // Handle Vote message
                    println!("received vote");
                }

                // Match the Committee message type
                ConsensusMessage::CommitteeMessage(committee) => {
                    // Handle Committee message
                    println!("received committee");
                }


                // Match the Codeword message type
                ConsensusMessage::CodewordMessage(codeword) => {
                    // Handle Codeword message
                    println!("received codeword");
                }

                // Match the Accum message type
                ConsensusMessage::AccumMessage(accum) => {
                    // Handle Accum message
                    println!("received accum");
                    
                }


                // Match the Propose message type
                ConsensusMessage::ProposeMessage(propose) => {
                    // Handle Propose message
                    println!("received propose");
                }
            
                
            }
        }    
        
    }

}


pub async fn reactor_init1(    
    pvss_data: Vec<u8>, committee_id: u32, 
    ip_address: Vec<&str>, level: u32, _index: u32, 
    args: Vec<String>) -> Vec<u8>
{     
    let committee_length = ip_address.len();    

    let leaves = pvss_agreement::encoder(pvss_data.clone(), committee_length.clone());
    // create accum value
    let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

    let acc_value_zl = merkle_tree::get_root(merkle_tree.clone());

    let qual: Vec<u32> = Vec::new();

    let empty_codeword_vec: Vec<String> = Vec::new();
    let empty_witness_vec: Vec<Vec<u8>> = Vec::new();  

    let retunval:   Vec<u8> = Vec::new();

    retunval
    
    // return reactor_helper(pvss_data, committee_id, &ip_address, level, _index, args, acc_value_zl, 0, 
    //     empty_codeword_vec, empty_witness_vec, 
    //     "accum".to_string(), committee_length, qual).await;
}


#[allow(non_snake_case)]
pub async fn reaction(output: Vec<Vec<String>>, mode: String, committee_length: usize,
    committee_id: u32, ip_address:  &Vec<&str>, level: u32, _index: u32, args: Vec<String>) -> (String, String, String)
{   
    let mut data: String = "pvss".to_string();

    let mut W1: String = "".to_string();
    let mut W2: String = "".to_string();
             
    let mut received_output: Vec<Vec<String>> = Vec::new();

    let mut check_first_codeword_list: Vec<String> = Vec::new();

    for value in output.clone()
    {            
        let value_clone = value.clone();
        let val_split: Vec<&str> = value_clone[0].split(", ").collect();

        if !check_first_codeword_list.contains(&val_split[1].to_string())
        {
            let (proof, codeword) = codeword::verify_codeword(value);
            
            if proof==true
            {
                check_first_codeword_list.push(val_split[1].to_string());

                // send witness to nodes if have received the first valid code word: prod
                let comm_output = communication(committee_id.clone(), ip_address.clone(), 
                level, _index, args.clone(), 
                    mode.clone(), codeword.clone()).await;
                received_output.push(comm_output);
            }
        }

        
    }

    let mut pvss_wrapper: Vec<String> = Vec::new();
    if level==1
    {
        for output in received_output.clone()
        {
            let codeword_wrapper = output[0].clone();
            let codeword_temp_vec: Vec<&str> = codeword_wrapper.split("]").collect();

            let codeword_temp = codeword_temp_vec[0].replace("[", "");

            let codeword: Vec<u8> = codeword_temp
                .split(", ")
                .map(|s| s.parse::<u8>().expect("Failed to parse u8"))
                .collect();

            let decoded_string = String::from_utf8(codeword).unwrap();

            pvss_wrapper.push(decoded_string);
            
        }

        pvss_wrapper.sort();        
        data = pvss_wrapper.concat();         
        
    } 
    else 
    { 
        for op in received_output.clone()
        {
            let mut codeword_vec: Vec<Vec<u8>> = Vec::new();
            
            for str_data in op.clone()
            {
                let split_str: Vec<&str> = str_data.split("]").collect();

                let codeword_str = split_str[0].replace("[", "");

                let codeword: Vec<u8> = codeword_str
                    .split(", ")
                    .map(|s| s.parse::<u8>().expect("Failed to parse u8"))
                    .collect();

                codeword_vec.push(codeword);

            }
            
            let pvss = pvss_agreement::decode(codeword_vec, committee_length);

            
            if W1=="".to_string()
            {
                W1 = pvss;
            }
            else 
            {
                W2 = pvss;
            }
        }        
        
        

    }       
    
    
    
    return (data, W1, W2);
}

#[allow(non_snake_case)]
pub async fn committee_selection(_pvss_data: String, committee_id: u32, ip_address: &Vec<&str>, level: u32, _index:u32, 
    args: Vec<String>, mut W1: String, mut W2: String, mode: String, committee_length: usize,  mut qual: Vec<u32>) -> Vec<u8>
{
    let mut b: Vec<u32> = Vec::new();

    b.push(1);
    b.push(2);
    
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


    let mut codeword_vec: Vec<String>;
    let mut witnesses_vec: Vec<Vec<u8>>;
    let mut merkle_len: usize;


    for val in qual
    {
        if val==1 && W1!="".to_string()
        {
            // deliver
            (codeword_vec, witnesses_vec, merkle_len) = deliver::deliver_encode(W1.clone().into_bytes(), 
                W1.clone(), committee_length.clone());

            let leaves = pvss_agreement::encoder(W1.clone().into_bytes(), committee_length.clone());
            // create accum value
            let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

            let acc_value_zl_W1 = merkle_tree::get_root(merkle_tree.clone());

            // where 𝑧𝑗 ∈ 𝑉𝑗 and 𝐴𝑇𝑗 ∈ 𝑊𝑗 . Upon decoding a valid APVSS transcript 𝐴𝑇𝑗 for an 𝑗 ∈ Qual s.t. 𝑊𝑗 = ∅, update 𝑊𝑗 := 𝑊𝑗 ∪ {𝐴𝑇𝑗 }}.
            let codeword_output = codeword_reactor(committee_id, ip_address, level, _index, args.clone(), 
            acc_value_zl_W1.clone(), merkle_len, codeword_vec, witnesses_vec, mode.clone()).await;


            let (_, w, _) = reaction(codeword_output, mode.clone(), committee_length,            
                    committee_id, ip_address, level, _index,  args.clone()
                ).await;

            if W1!=w
            {
                W1="bot".to_string();
            }
        }

        if val==2 && W2!="".to_string()
        {
            //deliver
            (codeword_vec, witnesses_vec, merkle_len) = deliver::deliver_encode(W2.clone().into_bytes(), 
                W2.clone(), committee_length.clone());

            let leaves = pvss_agreement::encoder(W2.clone().into_bytes(), committee_length.clone());
            // create accum value
            let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

            let acc_value_zl_W2 = merkle_tree::get_root(merkle_tree.clone());

            // where 𝑧𝑗 ∈ 𝑉𝑗 and 𝐴𝑇𝑗 ∈ 𝑊𝑗 . Upon decoding a valid APVSS transcript 𝐴𝑇𝑗 for an 𝑗 ∈ Qual s.t. 𝑊𝑗 = ∅, update 𝑊𝑗 := 𝑊𝑗 ∪ {𝐴𝑇𝑗 }}.
            let codeword_output = codeword_reactor(committee_id, ip_address, level, _index, args.clone(),  
            acc_value_zl_W2.clone(), merkle_len, codeword_vec, witnesses_vec, mode.clone()).await;


            let (_, w, _) = reaction(codeword_output, mode.clone(), committee_length,            
                    committee_id, ip_address, level, _index,  args.clone()
                ).await;

            
            if W2!=w
            {
                W2="bot".to_string();
            }
        }
    }

    let mut data = "".to_string();
    data+=&W1;
    data+=&W2;
    println!("{:?}", data);
    data.into_bytes()

}


#[async_recursion]
pub async fn reactor_helper<'a>(     
    pvss_data: Vec<u8>, committee_id: u32, ip_address: &'a Vec<&str>, level: u32, _index: u32, args: Vec<String>,  
    value: String, merkle_len: usize, codeword_vec: Vec<String>, witnesses_vec: Vec<Vec<u8>>, mode: String, committee_length: usize,
    qual: Vec<u32>) -> Vec<u8>
{ 
     
    if mode.contains("codeword")
    {     
        let codeword_output = codeword_reactor(committee_id, ip_address, level, _index, args.clone(), 
            value, merkle_len, codeword_vec, witnesses_vec, mode.clone()).await;

        let (pvss_data, w1, w2) = reaction(codeword_output, mode.clone(), committee_length,            
            committee_id, ip_address, level, _index,  args.clone()
        ).await;
        
        if level==1
        {
            return pvss_data.into_bytes();
        }
        
        return committee_selection(pvss_data, committee_id, ip_address, level, _index, args, w1, w2, mode, committee_length, qual).await;

    }    
    else if mode.contains("accum")
    {
        let (codeword_vec, witnesses_vec, merkle_len, qual): 
        (Vec<String>, Vec<Vec<u8>>, usize, Vec<u32>) = accum_reactor(
            pvss_data.clone(), committee_id, &ip_address, level, _index, args.clone(),  
            value.clone(), mode, committee_length, qual).await;

        return reactor_helper(pvss_data, committee_id, ip_address, level, _index, args, value, 
            merkle_len, codeword_vec, witnesses_vec, "codeword".to_string(), committee_length, qual).await;
    }
    else 
    {
        return "".to_string().into_bytes();
    }
    
     
}


pub async fn codeword_reactor( 
    committee_id: u32, ip_address: &Vec<&str>, level: u32, _index: u32, args: Vec<String>, 
    value: String, merkle_len: usize, codeword_vec: Vec<String>, witnesses_vec: Vec<Vec<u8>>, 
    mode: String)
-> Vec<Vec<String>>
{
    let mut index = 0;
    let mut codeword_output: Vec<Vec<String>> =  Vec::new();
    
    for witness in witnesses_vec
    {
        let subset_ip: &str;
        if ip_address.len()==1
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

        let codeword = generic::Codeword::create_codeword("".to_string(), leaf_values_to_prove.clone(), witness.clone(), 
        value.to_string(), indices_to_prove.clone(), merkle_len);
        index+=1;

        let codeword_vec = codeword.to_vec();

        // send codeword_vec individually to nodes: prod
        let output = communication(committee_id.clone(), subset_vec.clone(), level, _index, args.clone(), 
        mode.clone(), codeword_vec).await;
        
        codeword_output.push(output);
    
    }
    
    return codeword_output;
}

#[allow(non_snake_case)]
pub async fn accum_reactor(    
    pvss_data: Vec<u8>, committee_id: u32, ip_address: &Vec<&str>, level: u32, _index: u32, args: Vec<String>, 
    acc_value_zl: String, mode: String, committee_length: usize, mut qual: Vec<u32>) 
    ->  (Vec<String>, Vec<Vec<u8>>, usize, Vec<u32>)
{   
    let accum = generic::Accum::create_accum("sign".to_string(), acc_value_zl.clone());
    let accum_vec = accum.to_vec();

    //WORK ON THIS: WHEN RECEIVED SAME ACCUM VALUE FROM q/2 PARTIES: STOP ; also V1, V2
    let V: Vec<String> = communication(committee_id.clone(), ip_address.clone(), level.clone(), _index, args.clone(),  
        mode.clone(), accum_vec).await;
    
    let mut V1_vec: Vec<String> = Vec::new();
    let mut V2_vec: Vec<String> = Vec::new();

    let file_path = "./updatednodeinfo.txt";

    // Open the file for writing
    let _file1 = OpenOptions::new().append(true).open(file_path).await.unwrap();

    // Write to the file (assuming you have this part somewhere)

    // Open the file for reading
    
    for val in V.clone() 
    {
        let file2 = OpenOptions::new().read(true).open(file_path).await.unwrap();
        let reader = BufReader::new(file2);
        let mut line_stream = reader.lines();
        let val_clone = val.clone();
        let data_stream: Vec<&str> = val.split(", ").collect();

        let ipdetails = data_stream[4].clone();
        let substrings: Vec<&str> = ipdetails.split("/").collect();
        let ip = substrings[1];

        while let Some(line_result) = line_stream.next_line().await.unwrap() {
            let line1 = line_result;

            if line1.contains(ip) {
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
            }
        }
    }
    
    
    // Get majority accum value
    let V1 = accum::accum_check(V1_vec.clone(), committee_length.clone());

    let V2 = accum::accum_check(V2_vec.clone(), committee_length.clone());

    
    let mut v1 = byzar::BA(committee_id, ip_address, level, _index, args.clone(),
            V1.clone(), mode.clone(), committee_length.clone()).await;
    let mut v2 = byzar::BA( committee_id, ip_address, level, _index, args.clone(), 
        V2.clone(), mode.clone(), committee_length.clone()).await;

    if level!=1
    {
        v1 = byzar::check_equal(v1);
        v2 = byzar::check_equal(v2);
    }
    let mut codeword_vec: Vec<String> = Vec::new();
    let mut witnesses_vec: Vec<Vec<u8>>= Vec::new();

    let mut merkle_len: usize= 0;

    if v1!="bot" && v1!=""
    {        
        qual.push(1);
    }
    if v2!="bot" && v2!=""
    {
        qual.push(2);
    }
    let qual_clone = qual.clone();
    
    for val in qual
    {   
        if val==1 && v1==acc_value_zl
        {
            (codeword_vec, witnesses_vec, merkle_len) = deliver::deliver_encode(pvss_data.clone(), v1.clone(), committee_length.clone());

        }

        if val==2 && v2==acc_value_zl
        {   
            (codeword_vec, witnesses_vec, merkle_len) = deliver::deliver_encode(pvss_data.clone(), v2.clone(), committee_length.clone());

        }
    }       
    return (codeword_vec, witnesses_vec, merkle_len, qual_clone);
    }