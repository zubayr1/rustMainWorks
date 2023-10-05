extern crate optrand_pvss;

use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};
use optrand_pvss::modified_scrape::share::*;


pub fn aggregrate_intermediate(share1: Vec<u8>, share2: Vec<u8>, num_participants: usize)
{
    let deserialized_share1: optrand_pvss::modified_scrape::share::PVSSAggregatedShare<ark_ec::bls12::Bls12<ark_bls12_381::Parameters>> = 
        PVSSAggregatedShare::deserialize(&share1[..]).unwrap();

    let deserialized_share2: optrand_pvss::modified_scrape::share::PVSSAggregatedShare<ark_ec::bls12::Bls12<ark_bls12_381::Parameters>> = 
    PVSSAggregatedShare::deserialize(&share2[..]).unwrap();

    let degree =  num_participants/2 - 1;

    // let share1: PVSSAggregatedShare<_> = PVSSAggregatedShare::empty(degree, num_participants);
    // let share2: PVSSAggregatedShare<_> = PVSSAggregatedShare::empty(degree, num_participants);

    
    match deserialized_share1.aggregate(&deserialized_share2) {
        Ok(aggregated_share) => {
            // The result is an aggregated PVSS share
            // You can use the aggregated_share for further operations
            let mut serialized_data = Vec::new();
            aggregated_share.serialize(&mut serialized_data).unwrap();

            println!("Aggregated share: {:?}", serialized_data);
        }
        Err(err) => {
            // Handle the error if the aggregation fails
            println!("Error: {:?}", err);
        }
    }
}