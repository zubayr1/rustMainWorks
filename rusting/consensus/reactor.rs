use std::env;
use async_recursion::async_recursion;
use tokio::net::TcpStream;
use std::collections::HashMap;
use crate::nodes::Node;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::OpenOptions;

#[path = "../networking/communication.rs"]
mod communication;

#[path = "../types/generic.rs"]
mod generic; 


#[path = "../types/accum.rs"]
mod accum;

#[path = "../algos/pvss_agreement.rs"]
mod encoder;

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
    committee_id: u32, ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, medium: String, mode: String,
    initial_port: u32, test_port: u32, value: Vec<String>, communication_type: String) -> Vec<String>
{
    let mut output: Vec<String>= Vec::new();

    if medium=="prod_init"
        {
            output = communication::prod_communication(committee_id, ip_address.clone(), level, port_count, 
                _index, args.clone(), value.clone(), mode.clone(), communication_type.to_string()).await;
    
           
        }
        if medium=="dev_init"
        {
            if mode=="accum"
             {
                output = communication::dev_communication(committee_id, ["127.0.0.1".to_string(), (initial_port + _index).to_string()].join(":"), 
                ["127.0.0.1".to_string(), (test_port + _index).to_string()].join(":"), value.clone(), args.clone()).await;
    
            }
            else 
            {
                // output = communication::nested_dev_communication(nodes, committee_id, (initial_port + _index).to_string(), 
                // (test_port + _index).to_string(), value.clone(), args.clone()).await;
    
            }
            
        }
    
    return output;
}


pub async fn reactor_init(    
    pvss_data: String, committee_id: u32, 
    ip_address: Vec<&str>, level: u32, _index: u32, 
    args: Vec<String>, port_count: u32, medium: String)
{ 
 
    let committee_length = ip_address.len();
    

    let leaves = encoder::encoder(pvss_data.as_bytes(), committee_length.clone(), medium.clone());


    // create accum value
    let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

    let acc_value_zl = merkle_tree::get_root(merkle_tree.clone());

    let qual: Vec<u32> = Vec::new();

    let empty_codeword_vec: Vec<String> = Vec::new();
    let empty_witness_vec: Vec<Vec<u8>> = Vec::new();    
    
    reactor(pvss_data, committee_id, &ip_address, level, _index, args, port_count, acc_value_zl, 0, 
        empty_codeword_vec, empty_witness_vec, 
        "accum".to_string(), medium, committee_length, qual).await;
}



pub async fn reaction(output: Vec<Vec<String>>, medium: String, mode: String, _committee_length: usize,
    committee_id: u32, ip_address:  &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    initial_port: u32, test_port: u32
) -> bool
{
    let check: bool = false;

    if medium.clone()=="prod_init"
    {

        for a in output.clone()
        {
            println!("{:?}\n", a);
        }
        
        
        // let mut witness_to_deliver: Vec<String> = Vec::new();

        // witness_to_deliver.push(" ".to_string());

        // let mut check=false;
        
        // for words in output
        // {   
        //     if check==false
        //     {
        //         for value in words.clone()
        //         {                    
        //             let value_split: Vec<&str> = value.split(", ").collect();

                    
        //             let witness_verify =  codeword::verify_codeword(value.clone());

        //             if witness_verify==true
        //             {
        //                 witness_to_deliver.push(value_split[1].to_string());

        //                 check = true;
                        
        //                 break;
        //             }
                    
        //         }
        //     }             
                            
        // }
        // // send witness to nodes if have received the first valid code word: prod
        // let output = communication(committee_id.clone(), ip_address.clone(), level, _index, args.clone(), port_count, 
        //                     medium.clone(), mode.clone(), initial_port, test_port, witness_to_deliver, "broadcast".to_string()).await;

        // let mut s_values: Vec<String> = Vec::new();

        // for data in output
        // {
        //     let parts: Vec<&str> = data.split(", ").collect();

        //     s_values.push(parts[1].trim().to_string());
        // }
        

        // let _converted_s_values: Vec<u8> = s_values.iter()
        //         .map(|s| s.parse::<u8>().expect("Failed to convert to u8"))
        //         .collect();


        // for output_string in output
        // {
        //     let modified_vec: Vec<String> = output_string.split(", ")
        //         .map(|s| s.to_string())
        //         .collect();


        //     let encoded = &modified_vec[1..modified_vec.len() - 1];

        //     let ecc_len = 2*committee_length/2;


        //     let converted_data: Vec<u8> = encoded.iter()
        //         .map(|s| s.parse::<u8>().expect("Failed to convert to u8"))
        //         .collect();


        //     let enc = Encoder::new(ecc_len);
        //     let encoded = enc.encode(&converted_data[..]);

        //     // println!("{:?},   {:?},   {:?}", committee_length, converted_data, encoded);

        //     // let encoded = enc.encode(&[192, 137][..]); 
        //     // pvss_agreement::decoder(encoded, committee_length/2);
        // }
        
    
        
    }
    else 
    {
        
        if mode=="codeword"
        {
            timer::wait(1);

            let mut s_values: Vec<String> = Vec::new();

            let mut witness_to_deliver: Vec<String> = Vec::new();


            for words in output
            {
                let value = words[1].clone();
                if s_values.contains(&value.clone())
                {

                }
                else 
                {
                    s_values.push(value);

                    let words_string: String = words.join(", ");
                    
                    let witness_verify =  codeword::verify_codeword(words_string.clone());

                    if witness_verify==true
                    {
                        witness_to_deliver.push(words[1].to_string());
                    }
                }
            }
            // send witness to nodes if have received the first valid code word: dev
            let _output = communication(committee_id.clone(), ip_address.clone(), level, _index, args.clone(), port_count, 
                                medium.clone(), mode.clone(), initial_port, test_port, witness_to_deliver, "individual".to_string()).await;

            // println!("{:?}", output);
        }
    }
    return check;
}

#[async_recursion]
pub async fn reactor<'a>(     
    pvss_data: String, committee_id: u32, ip_address: &'a Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    value: String, merkle_len: usize, codeword_vec: Vec<String>, witnesses_vec: Vec<Vec<u8>>, mode: String, medium: String, committee_length: usize,
    qual: Vec<u32>) 
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
        
    let mut codeword_output: Vec<Vec<String>>= Vec::new();

    
    if mode.contains("codeword")
    {     
        codeword_output = codeword_reactor(committee_id, ip_address, level, _index, args.clone(), port_count, 
            value, merkle_len, codeword_vec, witnesses_vec, mode.clone(), medium.clone(), initial_port, test_port).await;

        
        let _codeword_reaction_check = reaction(codeword_output, medium, mode, committee_length,            
            committee_id, ip_address, level, _index,  args, port_count, 
            initial_port, test_port
        ).await;
        
    }
    else 
    {
        let (codeword_vec, witnesses_vec, merkle_len, qual): 
        (Vec<String>, Vec<Vec<u8>>, usize, Vec<u32>) = accum_reactor(
            pvss_data.clone(), committee_id, &ip_address, level, _index, args.clone(), port_count, 
            value.clone(), mode, medium.clone(), committee_length, initial_port, test_port, qual).await;


        reactor(pvss_data, committee_id, ip_address, level, _index, args, port_count, value, 
            merkle_len, codeword_vec, witnesses_vec, "codeword".to_string(), medium, committee_length, qual).await;
    }

    
     
}


pub async fn codeword_reactor( 
    committee_id: u32, ip_address: &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
value: String, merkle_len: usize, codeword_vec: Vec<String>, witnesses_vec: Vec<Vec<u8>>, mode: String, medium: String, initial_port: u32, test_port: u32)
-> Vec<Vec<String>>
{
    let mut index = 0;

    let mut codeword_output: Vec<Vec<String>> =  Vec::new();
    for witness in witnesses_vec
    {
        if medium=="prod_init"
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
            let leaf_values_to_prove = codeword_vec[index].to_string();
    
            
            let indices_to_prove = index.clone().to_string();
    
    
            let codeword = generic::Codeword::create_codeword("".to_string(), leaf_values_to_prove.clone(), witness.clone(), 
            value.to_string(), indices_to_prove.clone(), merkle_len);
            index+=1;
    
            let codeword_vec = codeword.to_vec();

            // send codeword_vec individually to nodes: prod
            let output = communication(committee_id.clone(), subset_vec.clone(), level, _index, args.clone(), port_count, 
            medium.clone(), mode.clone(), initial_port, test_port, codeword_vec, "individual".to_string()).await;
            
            codeword_output.push(output);
        }
        else 
        {            
            let leaf_values_to_prove = codeword_vec[index].to_string();

            
            let indices_to_prove = index.clone().to_string();


            let codeword = generic::Codeword::create_codeword("".to_string(), leaf_values_to_prove.clone(), witness.clone(), 
            value.to_string(), indices_to_prove.clone(), merkle_len);
            index+=1;

            let codeword_vec = codeword.to_vec();

            // send codeword_vec individually to nodes: dev
            let output = communication(committee_id.clone(), ip_address.clone(), level, _index, args.clone(), port_count, 
            medium.clone(), mode.clone(), initial_port, test_port, codeword_vec, "broadcast".to_string()).await;
            
            codeword_output.push(output);
            
        }
        

    }

    return codeword_output;
}

#[allow(non_snake_case)]
pub async fn accum_reactor(    
    pvss_data: String, committee_id: u32, ip_address: &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    acc_value_zl: String, mode: String, medium: String, committee_length: usize, initial_port: u32, test_port: u32, mut qual: Vec<u32>) 
    ->  (Vec<String>, Vec<Vec<u8>>, usize, Vec<u32>)
{
    

    let accum = generic::Accum::create_accum("sign".to_string(), acc_value_zl.clone());
    let accum_vec = accum.to_vec();

    //WORK ON THIS: WHEN RECEIVED SAME ACCUM VALUE FROM q/2 PARTIES: STOP ; also V1, V2
    let V: Vec<String> = communication(committee_id.clone(), ip_address.clone(), level.clone(), _index, args.clone(), port_count, 
        medium.clone(), mode.clone(), initial_port, test_port, accum_vec, "broadcast".to_string()).await;

    let mut V1_vec: Vec<String> = Vec::new();
    let mut V2_vec: Vec<String> = Vec::new();

    if medium=="prod_init"
    {
        let file_path = "./updatednodeinfo.txt";

        // Open the file for writing
        let _file1 = OpenOptions::new().append(true).open(file_path).await.unwrap();

        // Write to the file (assuming you have this part somewhere)

        // Open the file for reading
        
        for val in V.clone() {
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
        
        
    }
    
    else 
    {
        V1_vec =V.clone();
    }
    
    // Get majority accum value
    let V1 = accum::accum_check(V1_vec.clone(), medium.clone(), committee_length.clone());

    let V2 = accum::accum_check(V2_vec.clone(), medium.clone(), committee_length.clone());

    
    let v1 = byzar::byzar(committee_id, ip_address, level, port_count, _index, args.clone(),
            V1.clone(), medium.clone(), mode.clone(), "broadcast".to_string(), committee_length.clone()).await;
    let v2 = byzar::byzar( committee_id, ip_address, level, port_count, _index, args.clone(), 
        V2.clone(), medium.clone(), mode.clone(), "broadcast".to_string(), committee_length.clone()).await;


    let mut codeword_vec: Vec<String> = Vec::new();
    let mut witnesses_vec: Vec<Vec<u8>>= Vec::new();

    let mut merkle_len: usize= 0;

    
    if v1!="bot"
    {
        qual.push(1);
    }
    if v2!="bot"
    {
        qual.push(2);
    }
    let qual_clone = qual.clone();

    
    for val in qual
    {
        if val==1 && v1==acc_value_zl
        {
            (codeword_vec, witnesses_vec, merkle_len) = deliver::deliver_encode(pvss_data.as_bytes(), v1.clone(), committee_length.clone(), medium.clone());

        }

        if val==2 && v2==acc_value_zl
        {
            (codeword_vec, witnesses_vec, merkle_len) = deliver::deliver_encode(pvss_data.as_bytes(), v2.clone(), committee_length.clone(), medium.clone());

        }
    }
           
    return (codeword_vec, witnesses_vec, merkle_len, qual_clone);
    }