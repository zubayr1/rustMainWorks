use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NetworkMessage {
    pub sender: SocketAddr,
    pub addresses: Vec<SocketAddr>, // Vector containing all recipients.
    pub message: String,
}
