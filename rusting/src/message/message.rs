use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

pub struct InternalState {
    pub level: usize,
    pub addresses: Vec<SocketAddr>,
}

impl InternalState {
    
    // Method to return the 'level'
    pub fn get_level(&self) -> usize {
        self.level
    }

    // Method to return the 'addresses'
    pub fn _get_addresses(&self) -> &Vec<SocketAddr> {
        &self.addresses
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkMessage {
    pub sender: SocketAddr,
    pub addresses: Vec<SocketAddr>, // Vector containing all recipients.
    pub message: ConsensusMessage,
    pub level: usize
}



// Enum to represent the different message types
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConsensusMessage {
    EchoMessage(Echo),
    ForwardMessage(Forward),
    VoteMessage(Vote),
    CommitteeMessage(Committee),
    CodewordMessage(Codeword),
    CodewordRetrieveMessage(CodewordRetrieve),
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
pub struct Forward
{
    pub sign: String,
    pub value: String,
    pub types: String
}

#[allow(unused)]
impl Forward
{
    pub fn create_forward(sign: String, value: String) -> Self
    {
        Forward{sign:sign, value: value, types: "forward".to_string()}
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
    pub no: usize,
    pub types: String
}

#[allow(unused)]
impl Vote
{
    pub fn create_vote(sign: String, no: usize, value: String) -> Self
    {
        Vote{sign:sign, value: value, no: no, types: "vote".to_string()}
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
    pub codewords: String,
    pub witness: Vec<u8>,
    pub value: String,
    pub index: String,
    pub leaves_len: usize,
    pub part: usize,
    pub types: String
}

#[allow(unused)]
impl Committee
{
    pub fn create_committee(sign: String, codewords: String, witness: Vec<u8>, value: String, index: String, leaves_len: usize, part: usize) -> Self
    {
        Committee{sign:sign, codewords: codewords, witness: witness, value: value, 
            index: index, leaves_len: leaves_len, part, types: "committee".to_string()}
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
    pub part: usize,
    pub types: String
}

#[allow(unused)]
impl Codeword
{
    pub fn create_codeword(sign: String, codewords: String, witness: Vec<u8>, value: String, 
        index: String, leaves_len: usize, part: usize, types: String) -> Self
    {
        Codeword{sign:sign, codewords: codewords, witness: witness, value: value, 
            index: index, leaves_len: leaves_len, part, types: types}
    }

    pub fn to_vec(self) -> Vec<String> {
        let witness_string = format!("{:?}", self.witness);

        let modified_string = witness_string.replace(", ", "; ");

        vec![self.sign, self.codewords, modified_string, self.value, self.index, self.leaves_len.to_string(), self.types]
    }
}



#[allow(unused)]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CodewordRetrieve
{
    pub sign: String,
    pub codewords: String,  
    pub part: usize, 
    pub communication_type: String, 
    pub types: String
}

#[allow(unused)]
impl CodewordRetrieve
{
    pub fn create_codeword_retrieve(sign: String, codewords: String, part: usize, communication_type: String) -> Self
    {
        CodewordRetrieve{sign:sign, codewords: codewords, part: part, communication_type: communication_type, types: "codewordretrieve".to_string()}
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