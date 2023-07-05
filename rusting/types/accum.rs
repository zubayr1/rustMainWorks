#[path = "../consensus/timer.rs"]
mod timer; 

pub fn accum_check(received_texts: Vec<String>, committee_length: usize) -> bool
{
    if (received_texts.len())>=committee_length/2
    {
        return true;
    }
    return false;
}


pub fn accum_reaction(medium: String, received_texts: Vec<String>) -> Vec<(String, String)>
{
    let mut c: Vec<(String, String)> = Vec::new();
    
    if medium=="prod_init"
    {
        for text in received_texts
        {
        let split_text: Vec<&str> = text.split(',').collect();
        let accum_tuple = (split_text[0].to_string(), split_text[1].to_string());
        c.push(accum_tuple);
        }             
    }
    else 
    {
        let accum_tuple = (received_texts[0].to_string(), received_texts[1].to_string());

        c.push(accum_tuple);
    }      
    
    
    return c;
}


pub fn call_byzar(c: Vec<(String, String)>)
{
    timer::wait(1);

    let v1: Vec<(String, String)> = Vec::new();

    //byzar()
    //byzar()

}