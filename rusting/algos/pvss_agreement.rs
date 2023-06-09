extern crate reed_solomon;
use reed_solomon::Encoder;
use reed_solomon::Decoder;


pub fn encoder(pvss_data: &[u8], mut e: usize) -> Vec<String>
{
    if e==0
    {
        e=1;
    }
    // Length of error correction code
    let ecc_len = 2*e;

    let enc = Encoder::new(ecc_len);
    

    // Encode data
    let encoded = enc.encode(&pvss_data[..]);

    // Simulate some transmission errors
    // let mut corrupted = *encoded;
    // for i in 0..e {
    //     corrupted[i] = 0x0;
    // }


    // let orig_str = std::str::from_utf8(pvss_data).unwrap();

    
    let mut leaves: Vec<String> = Vec::new();

    for i in encoded.ecc()
    {
        leaves.push(i.to_string());
    }

    return leaves;

    

}

pub fn decoder(encoded: reed_solomon::Buffer, e: usize)
{
    // Length of error correction code
    let ecc_len = 2*e;

    let dec = Decoder::new(ecc_len);
   

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for i in 0..e {
        corrupted[i] = 0x0;
    }

    // Try to recover data
    let known_erasures = [0];

    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();


    let recv_str = std::str::from_utf8(recovered.data()).unwrap();

    println!("{:?}", recv_str);

}

