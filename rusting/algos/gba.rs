
#[path = "../types/generic.rs"]
mod generic; 

#[path = "../networking/communication.rs"]
mod communication;

#[allow(non_snake_case)]
pub async fn gba(committee_id: u32, ip_address: Vec<&str>, level: u32, port_count: u32, _index:u32, 
    args: Vec<String>, V: String, mode: String, types: String, committee_length: usize)
{
    let mut W: String = "".to_string();
    let mut C1: String = "".to_string();
    let mut C2: String = "".to_string();

    let b = committee_length;

    let echo = generic::Echo::create_echo("".to_string(), V.to_string());
    let echo_vec = echo.to_vec();

    let output = communication::prod_communication(committee_id, ip_address, level, port_count, _index, 
        args, echo_vec.clone(), mode, types).await;

    println!("{:?}, {:?}", output, output.len());


}