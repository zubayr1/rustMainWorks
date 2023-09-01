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
    EchoMessage(Echo),
    VoteMessage(Vote),
    CommitteeMessage(Committee),
    CodewordMessage(Codeword),
    AccumMessage(Accum),
    ProposeMessage(Propose),
}




#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Echo
{
    pub sign: String,
    pub value: String,
    pub types: String
}

#[allow(unused)]
impl Echo
{
    pub fn create_echo(sign: String, value: String) -> Self
    {
        Echo{sign:sign, value: value, types: "echo".to_string()}
    }

    pub fn to_vec(self) -> Vec<String> {
        vec![self.sign, self.value, self.types]
    }
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Vote
{
    pub sign: String,
    pub value: String,
    pub types: String
}

#[allow(unused)]
impl Vote
{
    pub fn create_vote(sign: String, value: String) -> Self
    {
        Vote{sign:sign, value: value, types: "vote".to_string()}
    }

    pub fn to_vec(self) -> Vec<String> {
        vec![self.sign, self.value, self.types]
    }
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Committee
{
    pub sign: String,
    pub value: String,
    pub types: String
}

#[allow(unused)]
impl Committee
{
    pub fn create_committee(sign: String, value: String) -> Self
    {
        Committee{sign:sign, value: value, types: "committee".to_string()}
    }
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Codeword
{
    pub sign: String,
    pub codewords: String,
    pub witness: Vec<u8>,
    pub value: String,
    pub index: String,
    pub leaves_len: usize,
    pub types: String
}

#[allow(unused)]
impl Codeword
{
    pub fn create_codeword(sign: String, codewords: String, witness: Vec<u8>, value: String, index: String, leaves_len: usize) -> Self
    {
        Codeword{sign:sign, codewords: codewords, witness: witness, value: value, 
            index: index, leaves_len: leaves_len, types: "codeword".to_string()}
    }

    pub fn to_vec(self) -> Vec<String> {
        let witness_string = format!("{:?}", self.witness);

        let modified_string = witness_string.replace(", ", "; ");

        vec![self.sign, self.codewords, modified_string, self.value, self.index, self.leaves_len.to_string(), self.types]
    }
}

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Accum
{
    pub sign: String,
    pub value: String,
    pub types: String
}

#[allow(unused)]
impl Accum
{
    pub fn create_accum(sign: String, value: String) -> Self
    {
        Accum{sign:sign, value: value, types: "accum".to_string()}
    }

    pub fn to_vec(self) -> Vec<String> {
        vec![self.sign, self.value, self.types]
    }
}



#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Propose
{
    pub sign: String,
    pub value: String,
    pub types: String
}

#[allow(unused)]
impl Propose
{
    pub fn create_propose(sign: String, value: String) -> Self
    {
        Propose{sign:sign, value: value, types: "propose".to_string()}
    }

    pub fn to_vec(self) -> Vec<String> {
        vec![self.sign, self.value, self.types]
    }
}