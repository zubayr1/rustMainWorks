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
    // let d = Phase::line;

    if line.contains("echo")
    {
        
        return "echo".to_string();
    }
    else if line.contains("vote")
    {
        return "vote".to_string();
    }
    if line.contains("committee")
    {
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