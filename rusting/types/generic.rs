pub struct Echo
{
    pub sign: String,
    pub value: String
}

impl Echo
{
    pub fn create_echo(sign: String, value: String) -> Self
    {
        Echo{sign:sign, value: value}
    }
}

pub struct Vote
{
    pub sign: String,
    pub value: String
}

impl Vote
{
    pub fn create_vote(sign: String, value: String) -> Self
    {
        Vote{sign:sign, value: value}
    }
}


pub struct Committee
{
    pub sign: String,
    pub value: String
}

impl Committee
{
    pub fn create_committee(sign: String, value: String) -> Self
    {
        Committee{sign:sign, value: value}
    }
}

pub struct Codeword
{
    pub sign: String,
    pub codewords: String,
    pub witness: String,
    pub accumulation_value: String
}

impl Codeword
{
    pub fn create_codeword(sign: String, codewords: String, witness: String, accumulation_value: String) -> Self
    {
        Codeword{sign:sign, codewords: codewords, witness: witness, accumulation_value: accumulation_value}
    }
}

pub struct Accum
{
    pub sign: String,
    pub accumulation_value: String
}

impl Accum
{
    pub fn create_accum(sign: String, accumulation_value: String) -> Self
    {
        Accum{sign:sign, accumulation_value: accumulation_value}
    }
}