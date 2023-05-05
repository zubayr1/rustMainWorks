use rand::{rngs::OsRng};
use schnorrkel::{Keypair,Signature, signing_context, PublicKey};
use schnorrkel::{PUBLIC_KEY_LENGTH, SIGNATURE_LENGTH};
use std::fs;

pub fn _create_keys_schnorrkel()
{
    let keypair: Keypair = Keypair::generate_with(OsRng);

    let context = signing_context(b"signature context");
    let message: &[u8] = "zake kal".as_bytes();
    let signature: Signature = keypair.sign(context.bytes(message));

    let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();
    let signature_bytes:  [u8; SIGNATURE_LENGTH]  = signature.to_bytes();

 

    //convert to string for valid utf-8
    let mut pubkeystr="[".to_string();

    let mut flag=0;
    for i in public_key_bytes
    {   if flag==0
        {
            pubkeystr = [pubkeystr.to_string(), i.to_string()].join("");
        }
        else {
            pubkeystr = [pubkeystr.to_string(), i.to_string()].join(", ");
        }
        flag=1;
        
    }
    pubkeystr = [pubkeystr.to_string(), "]".to_string()].join("");
    pubkeystr = [pubkeystr.to_string(), "//".to_string()].join("");


    let mut signstr="[".to_string();
    flag=0;

    for i in signature_bytes
    {
        if flag==0
        {
            signstr = [signstr.to_string(), i.to_string()].join("");
        }
        else {
            signstr = [signstr.to_string(), i.to_string()].join(", ");
        }
        flag=1;
    }
    signstr = [signstr.to_string(), "]".to_string()].join("");
    signstr = [signstr.to_string(), "//".to_string()].join("");

  

    let public_key: PublicKey = PublicKey::from_bytes(&public_key_bytes).unwrap();

    let signature:  Signature = Signature::from_bytes(&signature_bytes).unwrap();

    println!("{:?}", public_key);
    println!("{:?}", signature);

    fs::write("../pubkey.txt", pubkeystr).expect("Unable to write file");
    fs::write("../sign.txt", signstr).expect("Unable to write file");


}


pub fn _create_adversarial_key() -> String
{
    let keypair: Keypair = Keypair::generate_with(OsRng);

            
    let false_key_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();

    //convert to string for valid utf-8
    let mut false_key="[".to_string();

    let mut flag=0;
    for i in false_key_bytes
    {   if flag==0
        {
            false_key = [false_key.to_string(), i.to_string()].join("");
        }
        else {
            false_key = [false_key.to_string(), i.to_string()].join(", ");
        }
        flag=1;
        
    }
    false_key = [false_key.to_string(), "]".to_string()].join("");
    false_key = [false_key.to_string(), "//".to_string()].join("");

    return false_key;
}


pub fn _verify_schnorrkel_sign(pubkeystr: &str, signstr: &str) -> bool
{
    let pubkeybytes: Vec<u8> = serde_json::from_str(pubkeystr).unwrap();
    let signstrbytes: Vec<u8> = serde_json::from_str(signstr).unwrap();
    
    let public_key: PublicKey = PublicKey::from_bytes(&pubkeybytes).unwrap();

    let signature:  Signature = Signature::from_bytes(&signstrbytes).unwrap();

    
    let context = signing_context(b"signature context");
    let message: &[u8] = "zake kal".as_bytes();

    if public_key.verify(context.bytes(message), &signature).is_ok()
    {
        return true;
    }
    return false;


}
