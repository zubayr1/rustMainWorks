pub fn accum_reaction(received_texts: Vec<String>, threshold: usize)
{
    let mut C1: Vec<String> = Vec::new();

    if (received_texts.len())/2>=threshold
    {
        for text in received_texts
        {
            println!("{:?}", text);
        }
        
    }
}