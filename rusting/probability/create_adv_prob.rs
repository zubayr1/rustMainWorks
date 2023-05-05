use rand::Rng;

pub fn create_prob() -> bool
{
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();

    if n1 % 3 ==0
    {
        return true;
    }
    return false;
}