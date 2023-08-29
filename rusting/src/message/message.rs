use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[path = "../../types/generic.rs"]
mod generic; 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkMessage {
    pub sender: SocketAddr,
    pub addresses: Vec<SocketAddr>, // Vector containing all recipients.
    pub message: ConsensusMessage,
}



// Enum to represent the different message types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsensusMessage {
    EchoMessage(generic::Echo),
    VoteMessage(generic::Vote),
    CommitteeMessage(generic::Committee),
    CodewordMessage(generic::Codeword),
    AccumMessage(generic::Accum),
    ProposeMessage(generic::Propose),
}
