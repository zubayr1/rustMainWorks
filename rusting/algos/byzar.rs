
#[path = "./gba.rs"]
mod gba; 

#[allow(non_snake_case)]
pub async fn byzar( 
    committee_id: u32, ip_address: &Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, V: String, mode: String, types: String, committee_length: usize) -> String
{   

    let (w, _g) = gba::gba(committee_id, ip_address.to_vec(), level, port_count, _index, args.clone(),
    V.clone(), mode.clone(), types.clone(), committee_length).await;

    let b = twobyzar(committee_id, ip_address, level, port_count, _index, args.clone(),
    V.clone(), mode.clone(), types, committee_length).await;
    
    if b==false{
        return  "bot".to_string();
    }
    return w;
    
}


#[allow(non_snake_case)]
pub async fn twobyzar( 
    committee_id: u32, ip_address: &Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, V: String, mode: String, types: String, committee_length: usize) -> bool
{   
    let (V, _g) = gba::gba(committee_id, ip_address.to_vec(), level, port_count, _index, args.clone(),
    V.clone(), mode.clone(), types, committee_length).await;

    
    
    return true;
    
}