use std::env;
use async_recursion::async_recursion;


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

// enum Phase 
// {
//     echo, vote, committee, codeword, accum
// }

// impl Phase 
// {
//     pub fn is_weekday(&self) -> bool
//     {
//         match self 
//         {
//             &Phase:: echo => return false,
//             _=> return true
//         }
//     }
// }


async fn communication(committee_id: u32, ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, medium: String, mode: String,
    initial_port: u32, test_port: u32, value: Vec<String>, communication_type: String) -> Vec<String>
{
    let mut output: Vec<String>= Vec::new();

    if mode=="accum"
    {
        if medium=="prod_init"
        {
            output = communication::prod_communication(committee_id, ip_address.clone(), level, port_count, 
                _index, args.clone(), value.clone(), mode.clone(), communication_type.to_string()).await;
    
           
        }
        if medium=="dev_init"
        {
            output = communication::dev_communication(committee_id, ["127.0.0.1".to_string(), (initial_port + _index).to_string()].join(":"), 
                ["127.0.0.1".to_string(), (test_port + _index).to_string()].join(":"), value.clone(), args.clone()).await;
    
        }
    }
    else if mode=="codeword"
    {
        if medium=="prod_init"
        {
            output = communication::prod_communication(committee_id, ip_address.clone(), level, port_count, 
                _index, args.clone(), value.clone(), mode.clone(), communication_type.to_string()).await;
    
           
        }
        if medium=="dev_init"
        {
            output = communication::codeword_dev_communication(committee_id, (initial_port + _index).to_string(), 
                (test_port + _index).to_string(), value.clone(), args.clone()).await;
    
        }
    }
    

    return output;
}


pub async fn reactor_init(pvss_data: String, committee_id: u32, ip_address: Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, medium: String)
{       
    let committee_length = ip_address.len();

    let leaves = encoder::encoder(pvss_data.as_bytes(), committee_length.clone()/2);

    let merkle_tree = merkle_tree::create_tree(leaves.clone()); 

    let acc_value_zl = merkle_tree::get_root(merkle_tree.clone());

    let empty_vec: Vec<Vec<u8>> = Vec::new();
   
    timer::wait(1);
    reactor(pvss_data, committee_id, &ip_address, level, _index, args, port_count, acc_value_zl, 0, empty_vec, 
        "accum".to_string(), medium, committee_length).await;
}



pub async fn reaction(output: Vec<Vec<String>>, medium: String, mode: String, committee_length: usize,
    committee_id: u32, ip_address:  &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    initial_port: u32, test_port: u32
) -> bool
{
    let mut check: bool = false;

    if medium.clone()=="prod_init"
    {
        
        if mode=="codeword"
        {

            let mut witness_to_deliver: Vec<String> = Vec::new();

            witness_to_deliver.push(" ".to_string());

            let mut check=false;
            
            for words in output
            {   
                if check==false
                {
                    for value in words.clone()
                    {                    
                        let value_split: Vec<&str> = value.split(", ").collect();
    
                        
                        let witness_verify =  codeword::verify_codeword(value.clone());
    
                        if witness_verify==true
                        {
                            witness_to_deliver.push(value_split[1].to_string());

                            check = true;
                            
                            break;
                        }
                        
                    }
                }             
                                
            }
            // send witness to nodes if have received the first valid code word: prod
            let output = communication(committee_id.clone(), ip_address.clone(), level, _index, args.clone(), port_count, 
                                medium.clone(), mode.clone(), initial_port, test_port, witness_to_deliver, "broadcast".to_string()).await;

            let mut s_values: Vec<String> = Vec::new();

            for data in output
            {
                let parts: Vec<&str> = data.split(", ").collect();

                s_values.push(parts[1].trim().to_string());
            }
            

            let converted_s_values: Vec<u8> = s_values.iter()
                    .map(|s| s.parse::<u8>().expect("Failed to convert to u8"))
                    .collect();


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
            let output = communication(committee_id.clone(), ip_address.clone(), level, _index, args.clone(), port_count, 
                                medium.clone(), mode.clone(), initial_port, test_port, witness_to_deliver, "individual".to_string()).await;

            // println!("{:?}", output);
        }
    }
    return check;
}

#[async_recursion]
pub async fn reactor<'a>(pvss_data: String, committee_id: u32, ip_address: &'a Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    value: String, merkle_len: usize,  witnesses_vec: Vec<Vec<u8>>, mode: String, medium: String, committee_length: usize) 
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
        
    let codeword_output: Vec<Vec<String>>;

    if mode.contains("vote")
    {
        let vote: generic::Vote = generic::Vote::create_vote("".to_string(), "".to_string());
    }
    else if mode.contains("committee")
    {
        let committee = generic::Committee::create_committee("".to_string(), "".to_string());
    }
    else if mode.contains("codeword")
    {
        
        codeword_output = codeword_reactor(pvss_data.clone(), committee_id, ip_address, level, _index, args.clone(), port_count, 
            value, merkle_len,  witnesses_vec, mode.clone(), medium.clone(), committee_length, initial_port, test_port).await;

        let codeword_reaction_check = reaction(codeword_output, medium, mode, committee_length,            
            committee_id, ip_address, level, _index,  args, port_count, 
            initial_port, test_port
        ).await;
        
    }
    else 
    {
        let (witnesses_vec, merkle_len): (Vec<Vec<u8>>, usize) = accum_reactor(pvss_data.clone(), committee_id, &ip_address, level, _index, args.clone(), port_count, 
            value.clone(), mode, medium.clone(), committee_length, initial_port, test_port).await;


        reactor(pvss_data, committee_id, ip_address, level, _index, args, port_count, value, 
            merkle_len, witnesses_vec, "codeword".to_string(), medium, committee_length).await;
    }

    
     
}


pub async fn codeword_reactor(pvss_data: String, committee_id: u32, ip_address: &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
value: String, merkle_len: usize,  witnesses_vec: Vec<Vec<u8>>, mode: String, medium: String, committee_length: usize, initial_port: u32, test_port: u32)
-> Vec<Vec<String>>
{
    let mut index = 0;

    let code_words = pvss_agreement::encoder(pvss_data.as_bytes(), committee_length/2);

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
            let leaf_values_to_prove = code_words[index].to_string();
    
            
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
            let leaf_values_to_prove = code_words[index].to_string();

            
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
pub async fn accum_reactor(pvss_data: String, committee_id: u32, ip_address: &Vec<&str>, level: u32, _index: u32, args: Vec<String>, port_count: u32, 
    acc_value_zl: String, mode: String, medium: String, committee_length: usize, initial_port: u32, test_port: u32) ->  (Vec<Vec<u8>>, usize)
{                    
        let accum = generic::Accum::create_accum("sign".to_string(), acc_value_zl);
        let accum_vec = accum.to_vec();

        //WORK ON THIS: WHEN RECEIVED SAME ACCUM VALUE FROM q/2 PARTIES: STOP ; also V1, V2
        let V: Vec<String> = communication(committee_id.clone(), ip_address.clone(), level, _index, args.clone(), port_count, 
            medium.clone(), mode.clone(), initial_port, test_port, accum_vec, "broadcast".to_string()).await;

        let mut V1_vec: Vec<String> = Vec::new();
        let mut V2_vec: Vec<String> = Vec::new();

        if medium=="prod_init"
        {
            for val in V.clone()
            {
                let data_stream: Vec<&str>  = val.split(", ").collect();

                if data_stream[5].contains("l")
                {
                    V1_vec.push(val);
                }
                else 
                {
                    V2_vec.push(val);
                }
            }
        }
        else 
        {
            V1_vec =V.clone();
        }
        
        
        let V1 = accum::accum_check(V1_vec.clone(), medium.clone(), committee_length);


        let V2 = accum::accum_check(V2_vec.clone(), medium.clone(), committee_length);


        let v1 = byzar::byzar(committee_id, ip_address, level, port_count, _index, args.clone(),
             V1.clone(), mode.clone(), "broadcast".to_string()).await;
        let v2 = byzar::byzar(committee_id, ip_address, level, port_count, _index, args.clone(), 
            V2.clone(), mode.clone(), "broadcast".to_string()).await;




        let mut witnesses_vec: Vec<Vec<u8>>= Vec::new();

        let mut merkle_len: usize= 0;

        
        (witnesses_vec, merkle_len) = deliver::deliver_encode(pvss_data.as_bytes(), v1.clone(), committee_length.clone());

        (witnesses_vec, merkle_len) = deliver::deliver_encode(pvss_data.as_bytes(), v2.clone(), committee_length.clone());
        

        return (witnesses_vec, merkle_len);
    }