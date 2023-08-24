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
    committee_id: u32, ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, mode: String,
    initial_port: u32, test_port: u32, value: Vec<String>, communication_type: String) -> Vec<String>
{
    let mut output: Vec<String>= Vec::new();

    
    output = communication::prod_communication(committee_id, ip_address.clone(), level, port_count, 
        _index, args.clone(), value.clone(), mode.clone(), communication_type.to_string()).await;

    
        
        
    
    return output;
}


pub async fn reactor_init(    
    pvss_data: String, committee_id: u32, 
    ip_address: Vec<&str>, level: u32, _index: u32, 
    args: Vec<String>, port_count: u32) -> String
{     
    let committee_length = ip_address.len();    

    let leaves = encoder::encoder(pvss_data.as_bytes(), committee_length.clone());

    // create accum value
    let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

    let acc_value_zl = merkle_tree::get_root(merkle_tree.clone());

    let qual: Vec<u32> = Vec::new();

    let empty_codeword_vec: Vec<String> = Vec::new();
    let empty_witness_vec: Vec<Vec<u8>> = Vec::new();    
    
    return reactor(pvss_data, committee_id, &ip_address, level, _index, args, port_count, acc_value_zl, 0, 
        empty_codeword_vec, empty_witness_vec, 
        "accum".to_string(), committee_length, qual).await;
}


#[allow(non_snake_case)]
pub async fn reaction(output: Vec<Vec<String>>, mode: String, committee_length: usize,
    committee_id: u32, ip_address:  &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    initial_port: u32, test_port: u32
) -> (String, String, String)
{   println!("{:?}", output);
    let mut data: String = "pvss".to_string();

    let mut W1: String = "".to_string();
    let mut W2: String = "".to_string();;
             
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
                level, _index, args.clone(), port_count, 
                    mode.clone(), initial_port, test_port, codeword.clone(), 
                    "broadcast".to_string()).await;
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
        println!("received_output{:?}", received_output)   ;
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
pub async fn committee_selection(W1: String, W2: String, mut qual: Vec<u32>) -> String
{
    let mut b: Vec<u32> = Vec::new();
    b.push(1);
    b.push(2);

    if qual.contains(&1)
    {
        //2BA for W1
        // update b
    }
    if qual.contains(&2)
    {
        //2BA for W2
        // update b
    }
    qual.retain(|&x| b.contains(&x));


    for val in qual
    {
        if val==1 && W1!="".to_string()
        {
            // deliver
            // where 𝑧𝑗 ∈ 𝑉𝑗 and 𝐴𝑇𝑗 ∈ 𝑊𝑗 . Upon decoding a valid APVSS transcript 𝐴𝑇𝑗 for an 𝑗 ∈ Qual s.t. 𝑊𝑗 = ∅, update 𝑊𝑗 := 𝑊𝑗 ∪ {𝐴𝑇𝑗 }}.
        }

        if val==2 && W2!="".to_string()
        {
            //deliver
            // where 𝑧𝑗 ∈ 𝑉𝑗 and 𝐴𝑇𝑗 ∈ 𝑊𝑗 . Upon decoding a valid APVSS transcript 𝐴𝑇𝑗 for an 𝑗 ∈ Qual s.t. 𝑊𝑗 = ∅, update 𝑊𝑗 := 𝑊𝑗 ∪ {𝐴𝑇𝑗 }}.
        }
    }  
    let mut data = "".to_string();
    data+=&W1;
    data+=&W2;

    data

}


#[async_recursion]
pub async fn reactor<'a>(     
    pvss_data: String, committee_id: u32, ip_address: &'a Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    value: String, merkle_len: usize, codeword_vec: Vec<String>, witnesses_vec: Vec<Vec<u8>>, mode: String, committee_length: usize,
    qual: Vec<u32>) -> String
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
        
    
    if mode.contains("codeword")
    {     
        let codeword_output = codeword_reactor(committee_id, ip_address, level, _index, args.clone(), port_count, 
            value, merkle_len, codeword_vec, witnesses_vec, mode.clone(), initial_port, test_port).await;

        
        let (pvss_data, w1, w2) = reaction(codeword_output, mode, committee_length,            
            committee_id, ip_address, level, _index,  args, port_count, 
            initial_port, test_port
        ).await;
        
        if level==1
        {
            return pvss_data;
        }
        
        return committee_selection(w1, w2, qual).await;

    }
    else if mode.contains("accum")
    {
        let (codeword_vec, witnesses_vec, merkle_len, qual): 
        (Vec<String>, Vec<Vec<u8>>, usize, Vec<u32>) = accum_reactor(
            pvss_data.clone(), committee_id, &ip_address, level, _index, args.clone(), port_count, 
            value.clone(), mode, committee_length, initial_port, test_port, qual).await;
            
        return reactor(pvss_data, committee_id, ip_address, level, _index, args, port_count, value, 
            merkle_len, codeword_vec, witnesses_vec, "codeword".to_string(), committee_length, qual).await;
    }
    else 
    {
        return "".to_string();
    }
    
    
     
}


pub async fn codeword_reactor( 
    committee_id: u32, ip_address: &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    value: String, merkle_len: usize, codeword_vec: Vec<String>, witnesses_vec: Vec<Vec<u8>>, 
    mode: String, initial_port: u32, test_port: u32)
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
        let output = communication(committee_id.clone(), subset_vec.clone(), level, _index, args.clone(), port_count, 
        mode.clone(), initial_port, test_port, codeword_vec, "individual".to_string()).await;
        
        codeword_output.push(output);
    
        
        

    }

    return codeword_output;
}

#[allow(non_snake_case)]
pub async fn accum_reactor(    
    pvss_data: String, committee_id: u32, ip_address: &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    acc_value_zl: String, mode: String, committee_length: usize, initial_port: u32, test_port: u32, mut qual: Vec<u32>) 
    ->  (Vec<String>, Vec<Vec<u8>>, usize, Vec<u32>)
{   

    let accum = generic::Accum::create_accum("sign".to_string(), acc_value_zl.clone());
    let accum_vec = accum.to_vec();

    //WORK ON THIS: WHEN RECEIVED SAME ACCUM VALUE FROM q/2 PARTIES: STOP ; also V1, V2
    let V: Vec<String> = communication(committee_id.clone(), ip_address.clone(), level.clone(), _index, args.clone(), port_count, 
        mode.clone(), initial_port, test_port, accum_vec, "broadcast".to_string()).await;

    let mut V1_vec: Vec<String> = Vec::new();
    let mut V2_vec: Vec<String> = Vec::new();

    
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
    
    
    // Get majority accum value
    let V1 = accum::accum_check(V1_vec.clone(), committee_length.clone());

    let V2 = accum::accum_check(V2_vec.clone(), committee_length.clone());

    
    let v1 = byzar::BA(committee_id, ip_address, level, port_count, _index, args.clone(),
            V1.clone(), mode.clone(), "broadcast".to_string(), committee_length.clone()).await;
    let v2 = byzar::BA( committee_id, ip_address, level, port_count, _index, args.clone(), 
        V2.clone(), mode.clone(), "broadcast".to_string(), committee_length.clone()).await;


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
            (codeword_vec, witnesses_vec, merkle_len) = deliver::deliver_encode(pvss_data.as_bytes(), v1.clone(), committee_length.clone());

        }

        if val==2 && v2==acc_value_zl
        {
            (codeword_vec, witnesses_vec, merkle_len) = deliver::deliver_encode(pvss_data.as_bytes(), v2.clone(), committee_length.clone());

        }
    }           

    return (codeword_vec, witnesses_vec, merkle_len, qual_clone);
    }