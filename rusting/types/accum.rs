pub fn accum_reaction(medium: String, received_texts: Vec<String>, committee_length: usize) -> Vec<(String, String)>
{
    let mut c1: Vec<(String, String)> = Vec::new();

    if (received_texts.len())>=committee_length/2
    {
        if medium=="prod_init"
        {
           for text in received_texts
           {
            let split_text: Vec<&str> = text.split(',').collect();
            let accum_tuple = (split_text[0].to_string(), split_text[1].to_string());
            c1.push(accum_tuple);
           }             
        }
        else 
        {
            let accum_tuple = (received_texts[0].to_string(), received_texts[1].to_string());

            c1.push(accum_tuple);
        }      
                
    }
    
    return c1;
}