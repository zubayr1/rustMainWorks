
use hex::FromHex;
use std::convert::TryInto;

#[path = "../merkle_tree/merkle_tree.rs"]
mod merkle_tree;

pub fn verify_codeword(output: Vec<String>) -> bool
{
    let parts: Vec<&str> = output[0].split(", ").collect();

    let codeword_str = parts[1];
    let witness_str = parts[2];
    let accum_value_str = parts[3];
    let indices_to_prove_str = parts[4];
    let merkle_len_str = parts[5];


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

    let witness = witness_str.as_bytes().to_vec();

    let indices_to_prove: Vec<usize> = indices_to_prove_str.chars().map(|c| c as usize).collect();


    println!("{:?},   {:?},   {:?},   {:?},   {:?}", witness, indices_to_prove, codeword, u8_array, merkle_len_str);

    // let proof = merkle_tree::merkle_proof(witness, indices_to_prove, 
    // codeword, u8_array, merkle_len_str);

    
    return false;
}
