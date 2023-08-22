#[path = "../consensus/timer.rs"]
mod timer; 

use std::collections::HashMap;


#[allow(non_snake_case)]
pub fn accum_check(received_texts: Vec<String>, committee_length: usize) -> String
{
    let BOT = "bot".to_string();
    let mut count_map: HashMap<String, usize> = HashMap::new();

    let mut accum_val: String; 


    
    let mut accum_values: Vec<String> = Vec::new();
    for text in received_texts.clone()
    {
        let split_text: Vec<&str> = text.split(", ").collect();

        accum_val = split_text[1].to_string();

        accum_values.push(accum_val);
        
    } 

    // Count occurrences of each string
    for accum_string in accum_values {
        let count = count_map.entry(accum_string).or_insert(0);
        *count += 1;
    }

    for (accum_string, count) in count_map 
    {
        if count>=committee_length/2
        {
            return accum_string;
        }
    }

    return BOT;

    
    

    
}

