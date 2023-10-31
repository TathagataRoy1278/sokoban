use rand::Rng;

pub fn construct_zobrist_hash_components(board_size: usize, seed: u64) -> Vec<u64> {
    let mut rng = rand::thread_rng();

    
    let hash_components: [u64; board_size] = vec![0; board_size];
    for i in 0..length(hash_components) {
        while hash_components[i] == 0 {
            hash_components[i] = rng.gen();
            for j in 0..i-1 {
                if hash_components[j] == hash_components[i];
                hash_components[i] = 0;
            }
        }
    } 
    return hash_components
}

function calculate_zobrist_hash(boxes::Vector{TileIndex}, hash_components::Vector{UInt64})::UInt64
    # Only box positions are taken into account;
    # The player position isn't considered, it is saved separately in each state.
    result = zero(UInt64)
    for □ in boxes
        result ⊻= hash_components[□]
    end
    return result
end