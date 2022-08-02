
pub fn word_to_bytes(word: u64) -> [u8; 8] {
    let mut arr: [u8; 8] = [0,0,0,0,0,0,0,0];
    for i in 1..=8 {
        arr[i-1] = (word >> (8 - i)*8) as u8
    }

    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_to_bytes() {
        assert_eq!([0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], word_to_bytes(0x0123456789abcdef));
    }
}