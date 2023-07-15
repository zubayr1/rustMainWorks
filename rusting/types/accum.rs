#[path = "../consensus/timer.rs"]
mod timer; 

use std::collections::HashMap;


#[allow(non_snake_case)]
pub fn accum_check(received_texts: Vec<String>, medium: String, committee_length: usize) -> String
{
    let BOT = "bot".to_string();
    let mut count_map: HashMap<String, usize> = HashMap::new();

    let mut accum_val: String; 


    if medium=="prod_init"
    {
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
    else 
    {
        if received_texts.len()==0
        {
            return BOT;
        }
        let split_text: Vec<&str> = received_texts[0].split(", ").collect();
        accum_val = split_text[1].to_string();

        return accum_val;

    }

    
}


#[allow(non_snake_case)]
pub fn call_byzar(V: Vec<String>) -> (String, String, String)
{
    timer::wait(1);
    
    let mut v: (String, String, String) = ("".to_string(), "".to_string(), "".to_string());

    let mut unique_zl: (String, String, String) = ("".to_string(), "".to_string(), "".to_string());

    let mut unique_merkle_root_check: Vec<String> =  Vec::new();

    let mut id: String = "".to_string();
    let mut final_value: String = "".to_string();
    let mut final_committee: String = "".to_string();

    let mut tempFINALVALUE = "".to_string();

    for strings in V.clone()
    {
        
        let split_strings: Vec<&str> = strings.split(", ").collect();

        let temp_value = split_strings[1].trim().to_string().clone();

        id = [split_strings[0].trim().to_string().clone(), id.to_string()].join(" ");

        if unique_merkle_root_check.contains(&split_strings[1].to_string().clone()) {

        }
        else 
        {
            unique_merkle_root_check.push(split_strings[1].trim().to_string().clone());
            
            final_value = temp_value.clone();
            final_committee = split_strings[3].trim().to_string().clone();

            tempFINALVALUE = [tempFINALVALUE, final_value.clone()].join(" ");

            unique_zl = (id.clone(), final_value.clone(), final_committee.clone());

            //byzar(unique_zl)

        }

    }


    if unique_merkle_root_check.len() >= 1
    {
        v = (id, tempFINALVALUE.clone(), final_committee.clone());

    }

    
    
    return v;

}