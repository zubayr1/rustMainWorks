pub struct Echo
{
    pub sign: String,
    pub value: String,
    pub types: String
}

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

pub struct Vote
{
    pub sign: String,
    pub value: String,
    pub types: String
}

impl Vote
{
    pub fn create_vote(sign: String, value: String) -> Self
    {
        Vote{sign:sign, value: value, types: "vote".to_string()}
    }
}

pub struct Committee
{
    pub sign: String,
    pub value: String,
    pub types: String
}

impl Committee
{
    pub fn create_committee(sign: String, value: String) -> Self
    {
        Committee{sign:sign, value: value, types: "committee".to_string()}
    }
}

pub struct Codeword
{
    pub sign: String,
    pub codewords: Vec<String>,
    pub witness: Vec<u8>,
    pub accumulation_value: String,
    pub index: Vec<usize>,
    pub leaves_len: usize,
    pub types: String
}

impl Codeword
{
    pub fn create_codeword(sign: String, codewords: Vec<String>, witness: Vec<u8>, accumulation_value: String, index: Vec<usize>, leaves_len: usize) -> Self
    {
        Codeword{sign:sign, codewords: codewords, witness: witness, accumulation_value: accumulation_value, 
            index: index, leaves_len: leaves_len, types: "codeword".to_string()}
    }
}

pub struct Accum
{
    pub sign: String,
    pub accumulation_value: String,
    pub types: String
}

impl Accum
{
    pub fn create_accum(sign: String, accumulation_value: String) -> Self
    {
        Accum{sign:sign, accumulation_value: accumulation_value, types: "accum".to_string()}
    }

    pub fn to_vec(self) -> Vec<String> {
        vec![self.sign, self.accumulation_value, self.types]
    }
}