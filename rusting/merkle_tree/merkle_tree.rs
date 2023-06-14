use rs_merkle::MerkleTree;
use rs_merkle::MerkleProof;
use rs_merkle::algorithms::Sha256;
use sha2::{digest::FixedOutput, Digest};

fn hash(data: &[u8]) -> [u8; 32] 
{
    let mut hasher = sha2::Sha256::new();

    hasher.update(data);
    <[u8; 32]>::from(hasher.finalize_fixed())
}

fn create_tree(leaf_values: Vec<String>) -> MerkleTree<Sha256>
{
    let leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| hash(x.as_bytes()))
        .collect();

    let merkle_tree = MerkleTree::<Sha256>::from_leaves(&leaves);

    return merkle_tree;
}

fn append_to_tree(mut merkle_tree: MerkleTree<Sha256>, leaf_values: Vec<String>) -> MerkleTree<Sha256>
{
    let mut leaves: Vec<[u8; 32]> = leaf_values
        .iter()
        .map(|x| hash(x.as_bytes()))
        .collect();

    merkle_tree
    .append(&mut leaves)
    .commit();

    return  merkle_tree;
}

fn get_root(merkle_tree: MerkleTree<Sha256>) -> String
{
    let root = merkle_tree.root_hex().ok_or("couldn't get the merkle root").unwrap();

    return root;
}



fn create_proof_bytes(indices_to_prove: Vec<usize>, merkle_tree: MerkleTree<Sha256>) -> Vec<u8>
{

    let merkle_proof = merkle_tree.proof(&indices_to_prove);

    let proof_bytes = merkle_proof.to_bytes();

    return proof_bytes;

}


fn merkle_proof(proof_bytes: Vec<u8>, indices_to_prove: Vec<usize>, leaf_values_to_prove: Vec<String>, root: [u8; 32], len: usize) -> bool
{
    let proof = MerkleProof::<Sha256>::try_from(proof_bytes).unwrap();

    let leaves_to_proof: Vec<[u8; 32]>  = leaf_values_to_prove
        .iter()
        .map(|x| hash(x.as_bytes()))
        .collect();

    let leaves_to_proof = leaves_to_proof.get(0..1).ok_or("can't get leaves to prove").unwrap();

    println!("{:?}", leaves_to_proof);


    if proof.verify(root, &indices_to_prove, leaves_to_proof, len)
    {
        return true;
    }

    return false;
}




// fn main() {
    
//     let index = 2;

//     let mut leaf_values: Vec<String> = Vec::new();
//     leaf_values.push("a".to_string());
//     leaf_values.push("b".to_string());
//     leaf_values.push("c".to_string());

//     let merkle_tree = create_tree(leaf_values.clone());

    
//     let root = get_root(merkle_tree.clone());

//     println!("{}", root);


//     let mut leaf_values: Vec<String> = Vec::new();
//     leaf_values.push("d".to_string());
//     leaf_values.push("e".to_string());


//     let merkle_tree: MerkleTree<Sha256> = append_to_tree(merkle_tree.clone(), leaf_values.clone());

//     let root = get_root(merkle_tree.clone());

//     println!("{:?}", hash(root.as_bytes()));

  
//     let mut leaf_values_to_prove: Vec<String> = Vec::new(); 
//     leaf_values_to_prove.push("x".to_string());


    
//     let indices_to_prove = vec![index];

//     let proof_bytes = create_proof_bytes(indices_to_prove.clone(), merkle_tree.clone());

//     let merkle_root = merkle_tree.root().ok_or("couldn't get the merkle root").unwrap();


//     let proof = merkle_proof(proof_bytes, indices_to_prove, leaf_values_to_prove, merkle_root, merkle_tree.leaves_len());

//     println!("{}", proof);
  

// }
