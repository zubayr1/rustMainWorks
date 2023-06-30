extern crate reed_solomon;
use reed_solomon::Encoder;
use reed_solomon::Decoder;

pub fn encoder(pvss_data: &[u8], e: usize)
{
    // Length of error correction code
    let ecc_len = 2*e;

    let enc = Encoder::new(ecc_len);
    let dec = Decoder::new(ecc_len);

    // Encode data
    let encoded = enc.encode(&pvss_data[..]);

    // Simulate some transmission errors
    let mut corrupted = *encoded;
    for i in 0..4 {
        corrupted[i] = 0x0;
    }

    // Try to recover data
    let known_erasures = [0];

    let recovered = dec.correct(&mut corrupted, Some(&known_erasures)).unwrap();

    let orig_str = std::str::from_utf8(pvss_data).unwrap();
    let recv_str = std::str::from_utf8(recovered.data()).unwrap();

    println!("message:               {:?}", orig_str);
    println!("original data:         {:?}", pvss_data);
    println!("error correction code: {:?}", encoded.ecc());
    println!("corrupted:             {:?}", corrupted);
    println!("repaired:              {:?}", recv_str);

}

pub fn decoder()
{
    
}

