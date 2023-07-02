pub fn accum_reaction(received_texts: Vec<String>, committee_length: usize)
{
    let mut C1: Vec<String> = Vec::new();

    if (received_texts.len())>=committee_length/2
    {
        for text in received_texts
        {
            println!("{:?}", text);
        }
        
    }
}