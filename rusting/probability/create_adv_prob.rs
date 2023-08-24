use rand::Rng;

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

pub fn modify_string(input: &str, modify_probability: f64) -> String
{
    let mut rng = rand::thread_rng();
    let mut modified_string = String::new();

    for c in input.chars() {
        let mut modified_byte = c as u8;
        for bit_position in 0..8 {
            let random_probability: f64 = rng.gen(); // Generate a random probability between 0 and 1
            if random_probability < modify_probability {
                modified_byte ^= 1 << bit_position; // Flip the bit
            }
        }
        modified_string.push(modified_byte as char);
    }

    modified_string
}