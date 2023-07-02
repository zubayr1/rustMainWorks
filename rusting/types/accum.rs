pub fn accum_reaction(received_texts: Vec<String>, committee_length: usize)
{
    let mut C1: Vec<String> = Vec::new();

    if (received_texts.len())>=committee_length/2
    {           
        C1.push(received_texts[1].to_string());
        println!("{:?}", C1);
        
    }
}