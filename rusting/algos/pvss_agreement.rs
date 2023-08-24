use reed_solomon_erasure::galois_8::ReedSolomon;

#[allow(unused)]
pub fn to_shards(data: &[u8], num_nodes: usize, num_faults: usize) -> Vec<Vec<u8>> {
    let num_data_shards = num_nodes - num_faults;
    let shard_size = (data.len() + num_data_shards - 1) / num_data_shards;
    
    let padding_size = shard_size * num_data_shards - data.len();
    let mut data_with_padding = data.to_vec();
    data_with_padding.extend(vec![0; padding_size]);
    
    let mut result = Vec::with_capacity(num_nodes);
    for shard in 0..num_data_shards {
        result.push(data_with_padding[shard * shard_size..(shard + 1) * shard_size].to_vec());
    }
    
    for _shard in 0..num_faults {
        result.push(vec![0; shard_size]);
    }
    
    let r = ReedSolomon::new(num_data_shards, num_faults).unwrap();
    r.encode(&mut result).unwrap();
    
    result
}

#[allow(unused)]
pub fn from_shards(mut data: Vec<Option<Vec<u8>>>, num_nodes: usize, num_faults: usize) -> Vec<u8> {
    let num_data_shards = num_nodes - num_faults;
    let r = ReedSolomon::new(num_data_shards, num_faults).unwrap();
    r.reconstruct(&mut data).unwrap();

    let shard_size = data[0].as_ref().unwrap().len();

    let mut result = Vec::with_capacity(num_data_shards * shard_size);

    for shard in 0..num_data_shards {
        let mut shard_data = data[shard].clone().unwrap();
        if shard == num_data_shards - 1 {
            // Trim the padding for the last shard
            shard_data.truncate(shard_size);
        }
        result.extend(shard_data);
    }

    result
}




#[allow(unused)]
pub fn encoder(pvss_data: &[u8], mut committee_size: usize) -> Vec<String>
{    
    let original_data = pvss_data;
    let num_nodes = committee_size;      // Total number of shards
    let num_faults = committee_size/2;
    

    if committee_size==2
    {
        let mut shards : Vec<Vec<u8>> = Vec::new();
        shards.push(original_data.clone().to_vec());
        shards.push(original_data.clone().to_vec());

        let leaves: Vec<String> = shards
        .iter()
        .map(|inner_vec| format!("{:?}", inner_vec))
        .collect();

        return leaves;
    }
    let shards = to_shards(original_data, num_nodes, num_faults);

   
    let leaves: Vec<String> = shards
        .iter()
        .map(|inner_vec| format!("{:?}", inner_vec))
        .collect();
    
    return leaves;


}



#[allow(unused)]
pub fn decode(shards: Vec<Vec<u8>>, mut committee_size: usize) -> String
{
    let num_nodes = committee_size;      // Total number of shards
    let num_faults = committee_size/2;

    let mut received: Vec<_> = shards.iter().cloned().map(Some).collect();

    let reconstructed = from_shards(received, num_nodes, num_faults);

    let string: String = String::from_utf8_lossy(&reconstructed).into();

    return string;
}