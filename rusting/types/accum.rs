
#[path = "../consensus/timer.rs"]
mod timer; 


use serde_derive::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
struct CValueTuple {
    id_details: String,
    value: String,
    committee_id: String,
}




pub fn accum_check(received_texts: Vec<String>, medium: String, committee_length: usize) -> bool
{

    let mut accum_val: String; 

    let mut check_len = 0;

    if medium=="prod_init"
    {
        for text in received_texts.clone()
        {
            let split_text: Vec<&str> = text.split(',').collect();

            accum_val = split_text[2].to_string();

            if accum_val.contains("accum")
            {   
                check_len+=1;
            }
            
        }             
    }
    else 
    {
        accum_val = received_texts[2].to_string();

        if accum_val=="accum".to_string()
        {
            check_len+=1;
        }

    }


    if (check_len)>=committee_length/2
    {
        return true;
    }
    return false;
}


pub fn accum_reaction(medium: String, received_texts: Vec<String>) -> Vec<(String, String, String)>
{
    let mut c: Vec<(String, String, String)> = Vec::new();
    
    if medium=="prod_init"
    {
        for text in received_texts
        {
            let split_text: Vec<&str> = text.split(',').collect();

            let accum_tuple = (split_text[0].to_string(), split_text[1].to_string(), split_text[3].to_string());

            
            c.push(accum_tuple);
        }             
    }
    else 
    {
        let accum_tuple = (received_texts[0].to_string(), received_texts[1].to_string(), received_texts[3].to_string());

        c.push(accum_tuple);
    }      
    
    
    return c;
}


pub fn call_byzar(c: Vec<(String, String, String)>)
{
    timer::wait(1);
    
    let mut v: Vec<(String, String, String)> = Vec::new();

    let mut unique_merkle_root_check: Vec<String> =  Vec::new();

    for tuple in c.clone()
    {
        let json_string = serde_json::to_string(&tuple).unwrap();
    

        let deserialized_tuple: CValueTuple = serde_json::from_str(&json_string.to_string()).unwrap();

        let CValueTuple {id_details, value, committee_id} = deserialized_tuple;

        if unique_merkle_root_check.contains(&value) {

        }
        else 
        {
            unique_merkle_root_check.push(value);
        }

    }


    if unique_merkle_root_check.len() == 1
    {
        v = c;
    }
    

    //byzar()



}