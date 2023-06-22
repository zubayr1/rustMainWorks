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
    pub value: String
}

impl Codeword
{
    pub fn create_codeword(sign: String, value: String) -> Self
    {
        Codeword{sign:sign, value: value}
    }
}