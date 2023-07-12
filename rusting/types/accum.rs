
#[path = "../consensus/timer.rs"]
mod timer; 



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

    println!("{:?}", V);

    for strings in V.clone()
    {
        
        let split_strings: Vec<&str> = strings.split(',').collect();

        let temp_value = split_strings[1].trim().to_string().clone();

        id = [id.to_string(), split_strings[0].trim().to_string().clone()].join(" ");

        if unique_merkle_root_check.contains(&split_strings[1].to_string().clone()) {

        }
        else 
        {
            unique_merkle_root_check.push(split_strings[1].trim().to_string().clone());
            
            final_value = temp_value.clone();
            final_committee = split_strings[2].trim().to_string().clone();

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