
#[path = "../merkle_tree/merkle_tree.rs"]
mod merkle_tree;

#[path = "../algos/pvss_agreement.rs"]
mod pvss_agreement;



pub fn deliver_encode(pvss_data: &[u8], accum_value: String, committee_length: usize) -> (Vec<Vec<u8>>, usize)
{

    // Step 1.1: Partition m and run Encode algorithm
    let code_words = pvss_agreement::encoder(pvss_data, committee_length/2);


    // Step 1.2: create merkle proof: createWit
    let merkle_tree = merkle_tree::create_tree(code_words.clone());

    let mut index = 0;

    let mut witnesses_vec: Vec<Vec<u8>>= Vec::new();

    for word in code_words.clone()
    {
        let mut leaf_values_to_prove: Vec<String> = Vec::new(); 
        leaf_values_to_prove.push(word.to_string());

        let indices_to_prove = vec![index];

        let witness_proof_bytes = merkle_tree::create_proof_bytes(indices_to_prove.clone(), merkle_tree.clone());

        witnesses_vec.push(witness_proof_bytes.clone());

        index+=1;


    }


    return (witnesses_vec, merkle_tree.leaves_len());
    
    
    
}