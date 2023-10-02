extern crate optrand_pvss;
use ark_bls12_381::Bls12_381;
use ark_ec::PairingEngine;

use rand::thread_rng;
use optrand_pvss::signature::schnorr::SchnorrSignature;
use optrand_pvss::signature::scheme::SignatureScheme;
use optrand_pvss::modified_scrape::dealer::Dealer;
use optrand_pvss::modified_scrape::participant::Participant;
use optrand_pvss::modified_scrape::config::Config;

use ark_ec::bls12::Bls12;
use std::marker::PhantomData;
use ark_ec::short_weierstrass_jacobian::GroupAffine;
use ark_bls12_381::g1::Parameters;
use ark_serialize::{CanonicalSerialize, CanonicalDeserialize};


pub fn pvss_gen(args: Vec<String>) -> (Vec<u8>, 
        Config<Bls12<ark_bls12_381::Parameters>>
        , SchnorrSignature<GroupAffine<Parameters>>
        ,Dealer<Bls12_381,  
        SchnorrSignature<<Bls12_381 as PairingEngine>::G1Affine>>
        , rand::rngs::ThreadRng)
{
    let node_len = args[3].parse::<usize>().unwrap();

    let rng: &mut rand::rngs::ThreadRng = &mut thread_rng();

    let srs = optrand_pvss::modified_scrape::srs::SRS::<Bls12_381>::setup(rng).unwrap();

    let schnorr_srs = 
        optrand_pvss::signature::schnorr::srs::SRS::<<Bls12_381 as PairingEngine>::G1Affine>::setup(rng).unwrap();

    let schnorr_sig = optrand_pvss::signature::schnorr::SchnorrSignature { srs: schnorr_srs };

    
    // generate key pairs
    let dealer_keypair_sig  = schnorr_sig.generate_keypair(rng).unwrap();

    let eddsa_keypair = optrand_pvss::generate_production_keypair(); 

    let id = args[2].parse::<usize>().unwrap();

    // create the dealer instance
    let dealer: Dealer<Bls12_381,  
            SchnorrSignature<<Bls12_381 as PairingEngine>::G1Affine>> = Dealer {
            private_key_sig: dealer_keypair_sig.0,
            private_key_ed: eddsa_keypair.1,
            
            participant: Participant {
                pairing_type: PhantomData,
                id: id - 1,
                public_key_sig: dealer_keypair_sig.1,
                public_key_ed: eddsa_keypair.0,
            },
        };

    let config = Config {
            srs: srs.clone(),
            degree: (node_len/2 - 1),
            num_participants: node_len,
        };

    
    let mut serialized_data = Vec::new();
    dealer.clone().participant.serialize(&mut serialized_data).unwrap();


    let deserialized_data: Participant<Bls12_381, SchnorrSignature<<Bls12_381 as PairingEngine>::G1Affine>> = Participant::deserialize(&serialized_data[..]).unwrap();

        

    return (serialized_data, config, schnorr_sig, dealer, *rng);


}