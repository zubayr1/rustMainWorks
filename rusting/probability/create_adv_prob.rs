use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_prob(num_nodes: usize) -> bool
{
    let mut rng = rand::thread_rng();

    let n1: usize = rng.gen();

    if n1 % num_nodes ==0
    {
        return true;
    }
    return false;
}

#[allow(unused)]
fn generate_random_string(input: String) -> String {
    let length = input.len();
    let mut rng = rand::thread_rng();
    let random_string: String = (0..length).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();
    random_string
}

pub fn modify_accum(input: String) -> String
{
    let parts: Vec<&str> = input.split("-").collect();

    let p1 = generate_random_string(parts[0].to_string());

    let p2 = generate_random_string(parts[1].to_string());

    let p = format!("{}-{}", p1, p2);

    p


}

pub fn modify_string(mut input_str: Vec<String>) -> Vec<String>
{    
    if input_str.last().unwrap().to_string()=="accum".to_string()
    {
        let input = input_str.get(1).unwrap();
        let mut mutable_input = input.to_string();
        mutable_input = modify_accum(mutable_input);

        input_str[1] = mutable_input;

        println!("{:?}", input_str);

    }
    
    input_str
    
}