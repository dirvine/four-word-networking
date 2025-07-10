use three_word_networking::dictionary65k::get_global_dictionary;

fn main() {
    println!("Verifying the category byte issue");
    println!("=================================\n");

    // Let's manually unpack the words to see what's in the first byte
    let words = ["kaufhof", "dingley", "inno", "roupe", "stimuli", "bugger"];

    let dictionary = get_global_dictionary().unwrap();

    // Get indices for first group
    let idx1 = dictionary.get_index(words[0]).unwrap();
    let idx2 = dictionary.get_index(words[1]).unwrap();
    let idx3 = dictionary.get_index(words[2]).unwrap();

    println!("First group indices: {idx1} {idx2} {idx3}");

    // Combine into 48-bit value (before Feistel unmixing)
    let mixed = ((idx1 as u64) << 32) | ((idx2 as u64) << 16) | (idx3 as u64);
    println!("Mixed value: {mixed:#x}");

    // Apply reverse Feistel to get original value
    let unmixed = feistel_unmix_48(mixed);
    println!("Unmixed value: {unmixed:#x}");

    // Convert to bytes
    let mut bytes = [0u8; 6];
    for i in 0..6 {
        bytes[i] = ((unmixed >> (40 - i * 8)) & 0xFF) as u8;
    }

    println!("First group bytes: {bytes:?}");
    println!("First byte (should be category): {:#x}", bytes[0]);
    println!("Expected Loopback category value: {:#x}", 0u8); // Loopback enum value
}

fn feistel_unmix_48(input: u64) -> u64 {
    const ROUNDS: u32 = 6;

    let mut left = ((input >> 24) & 0xFFFFFF) as u32;
    let mut right = (input & 0xFFFFFF) as u32;

    for round in (0..ROUNDS).rev() {
        let new_left = right ^ feistel_round_function(left, round);
        right = left;
        left = new_left;
    }

    ((left as u64) << 24) | (right as u64)
}

fn feistel_round_function(input: u32, round: u32) -> u32 {
    let mut hash = input.wrapping_mul(0x9E3779B9);
    hash ^= round.wrapping_mul(0x85EBCA6B);
    hash ^= hash >> 16;
    hash = hash.wrapping_mul(0x85EBCA6B);
    hash ^= hash >> 13;
    hash = hash.wrapping_mul(0xC2B2AE35);
    hash ^= hash >> 16;
    hash & 0xFFFFFF
}
