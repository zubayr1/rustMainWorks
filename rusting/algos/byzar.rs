
#[path = "./gba.rs"]
mod gba; 

#[path = "../types/generic.rs"]
mod generic; 

use async_recursion::async_recursion;

use std::collections::HashMap;


#[path = "../networking/communication.rs"]
mod communication;


pub fn check_equal(input: String) -> String
{
    println!("{:?}", input);
    let parts: Vec<&str> = input.split(" && ").collect();

    if parts.len() == 2 && parts[0] == parts[1] {
        return parts[0].to_string();
    } else {
        return "bot".to_string();
    }
}

#[allow(non_snake_case)]
#[async_recursion]
pub async fn BA<'a>( 
    committee_id: u32, ip_address: &Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, V: String, mode: String, types: String, committee_length: usize) -> String
{   
    let ip_address_vec: Vec<&str> = ip_address.to_vec();


    let (mut V, g) = gba::gba(committee_id, ip_address_vec.clone(), level, port_count, _index, args.clone(),
    V.clone(), mode.clone(), types.clone(), committee_length).await;

    
    let propose = generic::Propose::create_propose("".to_string(), V.to_string());
    let propose_vec = propose.to_vec();

    let output = communication::prod_communication(committee_id, ip_address.clone(), level, port_count, 
        _index, args.clone(), propose_vec.clone(), mode.clone(), "broadcast".to_string()).await;


    if g==0
    {
        let mut value_vec: Vec<String> = Vec::new();
        let mut count_map: HashMap<String, u32> = HashMap::new();

        for val in output
        {
            let split_val: Vec<&str> = val.split(", ").collect();
            value_vec.push(split_val[0].to_string());
        }

        for element in value_vec {
            let count = count_map.entry(element).or_insert(0);
            *count += 1;
        }

        let tempval = &"".to_string();
    
        let (most_common_element, _) = count_map
            .iter()
            .max_by_key(|&(_, count)| count)
            .unwrap_or((&tempval, &0));

        V = most_common_element.to_string();
        
    }

    if committee_length!=2
    {
        let midpoint = ip_address_vec.len() / 2;
        let (first_half, second_half) = ip_address_vec.split_at(midpoint);
    
        let new_committee_length = committee_length/2;

        
        let first_result = BA( 
            committee_id, &first_half.to_vec(), level, port_count, _index, 
            args.clone(), V.clone(), mode.clone(), types.clone(), new_committee_length).await;

            
        let second_result =BA( 
            committee_id, &second_half.to_vec(), level, port_count, _index, 
            args, V, mode, types, new_committee_length).await;
        
        format!("{} && {}", first_result, second_result)

    }
    else 
    {   
        return V;
    }
    
}