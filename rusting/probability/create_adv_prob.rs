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
pub fn modify_accum(input: String) -> String {
    let length = input.len();
    let mut rng = rand::thread_rng();
    let random_string: String = (0..length).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();
    random_string
}



pub fn shuffle_codewords(input: String) -> String {
    // Extract the values from the input string
    let values: Vec<&str> = input
        .trim_matches(|c| c == '[' || c == ']')
        .split(';')
        .map(|s| s.trim())
        .collect();

    // Shuffle the values
    let mut rng = thread_rng();
    let mut shuffled_values = values.clone();
    shuffled_values.shuffle(&mut rng);

    // Join the shuffled values into a string
    let result = format!("[{}]", shuffled_values.join("; "));
    result
}