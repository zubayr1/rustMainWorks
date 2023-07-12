
use std::io::Write;
use std::fs::OpenOptions;
use std::collections::HashMap;
use chrono::Utc;
#[path = "../crypto/schnorrkel.rs"]
mod schnorrkel; 

// #[path = "../probability/create_adv_prob.rs"]
// mod create_adv_prob;


#[path = "../consensus/reactor.rs"]
mod reactor;

pub fn create_keys() // schnorr key generation
{
    schnorrkel::_create_keys_schnorrkel();

}

pub async fn initiate(filtered_committee: HashMap<u32, String>, args: Vec<String>)
{  
    let mut file: std::fs::File = OpenOptions::new().append(true).open("output.log").unwrap();

    let mut sorted: Vec<(&u32, &String)> = filtered_committee.iter().collect();

    sorted.sort_by_key(|a| a.0);

    let start_time = Utc::now().time();

    for _index in 1..(args[7].parse::<u32>().unwrap()+1) // iterate for all epoch
    {   
        println!("epoch {}", _index);

        let mut text;

        text = ["epoch ".to_string(), _index.to_string()].join(": ");
        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();

        let mut port_count: u32 = 0;
        
        if args[5]=="prod" // in prod mode
        {
            let mut level = 0;
            for (committee_id, ip_addresses_comb) in sorted.clone()
            {
                let ip_address: Vec<&str> = ip_addresses_comb.split(" ").collect();

                let mut _pvss_data: String = ["pvss_data".to_string(), committee_id.to_string()].join(" ");

                if ip_address.len()==1
                {
                    //GET PVSS DATA FROM DIMITRIS
                    _pvss_data = ["pvss_data".to_string(), committee_id.to_string()].join(" ");
                }
                else 
                {
                    port_count+=1;

                    reactor::reactor_init(_pvss_data.clone(),committee_id.clone(), ip_address.clone(), level, _index, args.clone(), port_count.clone(), "prod_init".to_string()).await;
                    level+=1;
                }

                
            }
                           
        }
        else 
        {           
            let pvss_data = ["pvss_data".to_string(), 999.to_string()].join(" ");     
            let mut ip_address: Vec<&str> = Vec::new();
            let address:&str = "127.0.0.1";
            ip_address.push(address);
            let level = 0;
            reactor::reactor_init(pvss_data.clone(), 999, ip_address.clone(), level, _index, args.clone(), port_count.clone(), "dev_init".to_string()).await;

        }

        text = "--------------------------------".to_string();

        file.write_all(text.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();


    }

    let end_time = Utc::now().time();

    let diff = end_time - start_time;
    
    println!("End by {}. time taken {} seconds", args[6], diff.num_seconds());
    
    

}