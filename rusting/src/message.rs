use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BroadcastMessage {
    pub sender: usize,
    pub content: String,
    pub round: usize
}
