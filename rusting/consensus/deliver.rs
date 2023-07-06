
#[path = "../merkle_tree/merkle_tree.rs"]
mod merkle_tree;

#[path = "../algos/pvss_agreement.rs"]
mod pvss_agreement;


pub fn deliver(pvss_data: &[u8], accum_value: String, committee_length: usize)
{

    // Step 1.1: Partition m and run Encode algorithm
    let code_words = pvss_agreement::encoder(pvss_data, committee_length/2);

    println!("{} {:?}   {:?}", committee_length, pvss_data, code_words);

    // Step 1.2: create merkle proof: createWit
    merkle_tree::create_tree(code_words);

    // let mut witnessess: Vec<String> = Vec::new();

    // for word in code_words.clone()
    // {
    //     let wit = create_witness(accum_value.clone(), accum_value.clone(), word);

    //     witnessess.push(wit);
    // }

    // println!("{:?}", witnessess);
}