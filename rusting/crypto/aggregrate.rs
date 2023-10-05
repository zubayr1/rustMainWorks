extern crate optrand_pvss;
use ark_bls12_381::Bls12_381;
use ark_ec::PairingEngine;

use optrand_pvss::signature::schnorr::SchnorrSignature;

use optrand_pvss::modified_scrape::participant::Participant;



pub fn aggregrate_intermediate(share1: Vec<u8>, share2: Vec<u8>, num_participants: usize)
{
    let deserialized_share1: Participant<Bls12_381, SchnorrSignature<<Bls12_381 as PairingEngine>::G1Affine>> = 
        Participant::deserialize(&share1[..]).unwrap();

    let deserialized_share2: Participant<Bls12_381, SchnorrSignature<<Bls12_381 as PairingEngine>::G1Affine>> = 
        Participant::deserialize(&share2[..]).unwrap();

    
    match deserialized_share1.aggregate(&deserialized_share2) {
        Ok(aggregated_share) => {
            // The result is an aggregated PVSS share
            // You can use the aggregated_share for further operations
            println!("Aggregated share: {:?}", aggregated_share);
        }
        Err(err) => {
            // Handle the error if the aggregation fails
            println!("Error: {:?}", err);
        }
    }
}