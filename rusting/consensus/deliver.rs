
#[path = "../merkle_tree/merkle_tree.rs"]
mod merkle_tree;

#[path = "../algos/pvss_agreement.rs"]
mod pvss_agreement;



pub fn deliver_encode(pvss_data: &[u8], _accum_value: String, committee_length: usize, medium: String) -> (Vec<String>, Vec<Vec<u8>>, usize)
{ 
    // Step 1.1: Partition m and run Encode algorithm
    let code_words = pvss_agreement::encoder(pvss_data, committee_length, medium);

    // Step 1.2: create merkle proof: createWit
    let merkle_tree = merkle_tree::create_tree(code_words.clone());

    let mut index = 0;

    let mut witnesses_vec: Vec<Vec<u8>>= Vec::new();

    for word in code_words.clone()
    {
        // let mut leaf_values_to_prove: Vec<String> = Vec::new(); 
        // leaf_values_to_prove.push(word.to_string());

        let indices_to_prove = vec![index];

        let witness_proof_bytes = merkle_tree::create_proof_bytes(indices_to_prove.clone(), merkle_tree.clone());

        witnesses_vec.push(witness_proof_bytes.clone());

        index+=1;


    }

    let mut i = 0;
    let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root").unwrap();
    println!("{:?}   {:?}", merkle_root, _accum_value);

    let hex_bytes = _accum_value
        .as_bytes()
        .chunks(2)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
        .collect::<Vec<u8>>();

    if hex_bytes.len() != 32 {
        panic!("Hexadecimal string must be 32 bytes long");
    }

    let mut u8_array: [u8; 32] = Default::default();
    u8_array.copy_from_slice(&hex_bytes);

    println!("{:?}", u8_array);

    // for i in 1..index
    // {
    //     let ivec: Vec<usize> = Vec::new();
    //     ivec.push(i);

    //     let codevec: Vec<String> = Vec::new();
    //     codevec.push(code_words[i]);
    //     let proof = merkle_tree::merkle_proof(witnesses_vec[i], ivec, 
    //         codevec, _accum_value, merkle_tree.leaves_len());

    //     println!("{}", proof);
    // }


    return (code_words, witnesses_vec, merkle_tree.leaves_len());
    
    
    
}