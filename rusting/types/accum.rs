pub fn accum_reaction(medium: String, received_texts: Vec<String>, committee_length: usize)
{
    let mut C1: Vec<String> = Vec::new();

    if (received_texts.len())>=committee_length/2
    {
        if medium=="prod_init"
        {
           for text in received_texts
           {
            println!("{}", text);
           }             
        }
        else 
        {
            C1.push(received_texts[1].to_string());
        }      
                
    }
    println!("{:?}", C1);
}