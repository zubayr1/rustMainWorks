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


pub fn aggregrate_intermediate()
{
    
}