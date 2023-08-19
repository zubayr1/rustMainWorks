

#[path = "../merkle_tree/merkle_tree.rs"]
mod merkle_tree;


#[allow(unused)]
pub fn verify_codeword(output: Vec<String>) -> (bool,Vec<String>)
{
    let parts: Vec<&str> = output[0].split(", ").collect();

    let codeword_str = parts[1];
    let witness_str = parts[2];
    let accum_value_str = parts[3];
    let indices_to_prove_str = parts[4];
    let merkle_len_str = parts[5];

    let codeword_str = codeword_str.to_string().replace(";", ",");
    let witness_str = witness_str.to_string().replace(";", ",");

    let hex_bytes = accum_value_str.to_string()
        .as_bytes()
        .chunks(2)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
        .collect::<Vec<u8>>();

    if hex_bytes.len() != 32 {
        panic!("Hexadecimal string must be 32 bytes long");
    }

    let mut u8_array: [u8; 32] = Default::default();
    u8_array.copy_from_slice(&hex_bytes);


    let mut codeword: Vec<String> = Vec::new();
    codeword.push(codeword_str.to_string());

    
    let trimmed_str = witness_str.trim_start_matches('[').trim_end_matches(']');
    let witness: Vec<u8> = trimmed_str
        .split(',')
        .map(|s| s.trim().parse::<u8>().unwrap())
        .collect();


    let indices_to_prove_nested: usize = indices_to_prove_str.to_string().parse().unwrap();
    let mut indices_to_prove: Vec<usize> = Vec::new();
    indices_to_prove.push(indices_to_prove_nested);

    let merkle_len: usize = merkle_len_str.to_string().parse().unwrap();
    

    let proof = merkle_tree::merkle_proof(witness, indices_to_prove, 
        codeword.clone(), u8_array, merkle_len);

    return (proof, codeword);
}
