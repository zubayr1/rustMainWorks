
#[path = "./gba.rs"]
mod gba; 
use async_recursion::async_recursion;

#[path = "../networking/communication.rs"]
mod communication;

#[allow(non_snake_case)]
pub async fn byzar( 
    committee_id: u32, ip_address: &Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, V: String, mode: String, types: String, committee_length: usize) -> String
{   

    let (w, _g) = gba::gba(committee_id, ip_address.to_vec(), level, port_count, _index, args.clone(),
    V.clone(), mode.clone(), types.clone(), committee_length).await;

    // let b = twobyzar(committee_id, ip_address, level, port_count, _index, args.clone(),
    // V.clone(), mode.clone(), types, committee_length).await;
    
    // if b==false{
    //     return  "bot".to_string();
    // }
    return w;
    
}


#[allow(non_snake_case)]
#[async_recursion]
pub async fn BA<'a>( 
    committee_id: u32, ip_address: &Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, V: String, mode: String, types: String, committee_length: usize) -> String
{   
    let ip_address_vec: Vec<&str> = ip_address.to_vec();

    let (V, _g) = gba::gba(committee_id, ip_address_vec.clone(), level, port_count, _index, args.clone(),
    V.clone(), mode.clone(), types.clone(), committee_length).await;

    let mut value: Vec<String> = Vec::new();

    value.push(V.clone());

    let output = communication::prod_communication(committee_id, ip_address.clone(), level, port_count, 
        _index, args.clone(), value.clone(), mode.clone(), "broadcast".to_string()).await;


    // println!("{:?}", output);

    if committee_length!=2
    {
        let midpoint = ip_address_vec.len() / 2;
        let (first_half, second_half) = ip_address_vec.split_at(midpoint);
    
        let new_committee_length = committee_length/2;

        println!("first half {:?}", first_half);
        println!("second half {:?}", second_half);

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