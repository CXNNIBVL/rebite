
/// From [Bit Twiddling Hacks](https://graphics.stanford.edu/~seander/bithacks.html#ReverseByteWith64BitsDiv)
pub fn reverse_bit_endianness(byte: u8) -> u8 {
    let mut as_u64 = byte as u64;

    as_u64 = (as_u64 * 0x0202020202u64 & 0x010884422010u64) % 1023;

    as_u64 as u8
}

// From [Bit Twiddling Hacks](https://graphics.stanford.edu/~seander/bithacks.html#SwappingValuesXOR)
pub fn swap_bytes(a: &mut u8, b: &mut u8) {
    *a ^= *b;
    *b ^= *a;
    *a ^= *b;
}

pub fn set_bytes(bytes: &mut [u8], value: u8) {
    for byte in bytes {
        *byte = value;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rev_bit_endianness_one() {

        let byte = 0b1;
        let expected = 0b10000000;

        assert_eq!(reverse_bit_endianness(byte), expected);
    }

    #[test]
    fn test_rev_bit_endianness_arbitrary() {

        let byte = 0b10011010;
        let expected = 0b01011001;

        assert_eq!(reverse_bit_endianness(byte), expected);
    }

    #[test]
    fn test_swap_bytes() {

        let mut a = 1;
        let mut b = 2;

        swap_bytes(&mut a, &mut b);

        assert_eq!(a, 2);
        assert_eq!(b, 1);
    }

}