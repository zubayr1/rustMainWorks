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

fn modify_accum(input: String) -> String
{
    let mut rng = thread_rng();
    let mut chars: Vec<char> = input.chars().collect();
    chars.shuffle(&mut rng);
    chars.iter().collect()
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