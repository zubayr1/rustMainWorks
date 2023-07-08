
use hex::FromHex;
use std::convert::TryInto;

#[path = "../merkle_tree/merkle_tree.rs"]
mod merkle_tree;

pub fn verify_codeword(value: String)
{
    let value_split: Vec<&str> = value.split(", ").collect();

    let mut leaf_values_to_prove: Vec<String> = Vec::new(); 
    leaf_values_to_prove.push(value_split[1].to_string());

    let indices_to_prove: Vec<usize> = vec![value_split[4].clone().parse().unwrap()];

    
    let witness_str = value_split[2];

    println!("{:?}", witness_str);

    let witness: Vec<u8> = witness_str
    .strip_prefix('[')
    .unwrap()
    .strip_suffix(']')
    .unwrap()
    .split(", ")
    .map(|s| s.parse().unwrap())
    .collect();

    let merkle_len = value_split[5].parse().unwrap();

    let accum_value = value_split[3];

    let root = Vec::from_hex(accum_value.clone()).ok().unwrap();
    let byte_root: [u8; 32] = root[..32].try_into().expect("Invalid length of byte vector");


    let proof = merkle_tree::merkle_proof(witness.clone(), indices_to_prove.clone(), leaf_values_to_prove, byte_root, merkle_len);


    println!("{:?}", proof);
}
