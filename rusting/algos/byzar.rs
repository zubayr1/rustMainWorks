
#[path = "./gba.rs"]
mod gba; 


use tokio::sync::mpsc::Sender;

use crate::nodes::reactor::NetworkMessage;


#[allow(non_snake_case)]
pub async fn BA_setup( 
    tx_sender: Sender<NetworkMessage>, ip_address: Vec<&str>, 
    args: Vec<String>, V: String, level: usize) 
{
    gba::gba_setup(tx_sender, ip_address, args, V, level).await;
}   

