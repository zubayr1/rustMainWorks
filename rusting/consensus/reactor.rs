#[path = "../types/generic.rs"]
mod generic; 

enum Phase 
{
    echo, vote, committee, codeword, accum
}

impl Phase 
{
    pub fn is_weekday(&self) -> bool
    {
        match self 
        {
            &Phase:: echo => return false,
            _=> return true
        }
    }
}


pub async fn reactor_init(line: String) -> String
{

    if line.contains("echo")
    {
        let echo = generic::Echo::create_echo("".to_string(), "".to_string());
        return "echo".to_string();
    }
    else if line.contains("vote")
    {
        let vote = generic::Vote::create_vote("".to_string(), "".to_string());
        return "vote".to_string();
    }
    if line.contains("committee")
    {
        let committee = generic::Committee::create_committee("".to_string(), "".to_string());
        return "committee".to_string();
    }
    else if line.contains("codeword")
    {
        return "codeword".to_string();
    }
    else 
    {
        return "accum".to_string();
    }
     
}