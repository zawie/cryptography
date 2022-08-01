
fn word_to_bytes(word: u64) -> [u8; 8] {
    let mut arr: [u8; 8] = [0,0,0,0,0,0,0,0];
    for i in 1..=8 {
        arr[i-1] = (word >> (8 - i)*8) as u8
    }

    arr
}