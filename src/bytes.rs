use std::ops::{
    AddAssign,
    SubAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    DivAssign,
    MulAssign,
    Not,
    RemAssign,
    ShlAssign,
    ShrAssign,
};

use crate::{
    util, 
    bytes_iter::{BytesIter, BytesIterMut}
};

pub struct ByteString<'a> {
    bytes: &'a mut [u8],
    interpret_reverse_endian: bool
}

impl<'a> ByteString<'a> {

    pub fn new(bytes: &'a mut [u8]) -> ByteString<'a> {
        Self { bytes, interpret_reverse_endian: false }
    }

    pub fn interpret_reverse_endian(&mut self) {
        self.interpret_reverse_endian = !self.interpret_reverse_endian;
    }

    pub fn byte_len(&self) -> usize {
        self.bytes.len()
    }

    pub fn bit_len(&self) -> usize {
        self.byte_len() * 8
    }

    pub fn bytes(&self) -> &[u8] {
        self.bytes
    }

    pub fn bytes_mut(&mut self) -> &mut [u8] {
        self.bytes
    }

    pub fn iter(&self) -> BytesIter {
        BytesIter::new(self.bytes, self.interpret_reverse_endian)
    }

    pub fn iter_mut(&mut self) -> BytesIterMut {
        BytesIterMut::new(self.bytes, self.interpret_reverse_endian)
    }

    pub fn set_bytes_with_value(&mut self, value: u8) {
        util::set_bytes(self.bytes, value);
    }

    pub fn set_zero(&mut self) {
        self.set_bytes_with_value(0);
    }

    pub fn is_zero(&self) -> bool {
        self.bytes().iter().all(|e| e == &0)
    }

    pub fn reverse_byte_endianness(&mut self) {

        let count = self.byte_len();
        let mid = ( count - ( count % 2 ) ) / 2;

        let (a, b) = self.bytes_mut().split_at_mut(mid);

        let iter = a.iter_mut().zip(b.iter_mut().rev());

        for (x,y) in iter {
            util::swap_bytes(x, y);
        }
    }

    pub fn reverse_bit_endianness(&mut self) {
        for byte in self.bytes_mut() {
            *byte = util::reverse_bit_endianness(*byte);
        }
    }

    pub fn rotl_bytes(&mut self, count: usize) {
        if self.interpret_reverse_endian {
            self.bytes.rotate_right(count);
            return;
        }

        self.bytes.rotate_left(count);
    }

}

impl<'a> PartialEq for ByteString<'a> {
    fn eq(&self, other: &Self) -> bool {

        let eq_len = self.byte_len() == other.byte_len();
        let eq_mem = self.iter().eq(other.iter());

        eq_len && eq_mem
    }
}
impl<'a> Eq for ByteString<'a> {}

impl<'a> Not for ByteString<'a> {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.iter_mut().for_each(|e| *e = e.not() );
        self
    }
}

impl<'a, 'b: 'a> BitAndAssign<&'b ByteString<'b>> for ByteString<'a> {
    fn bitand_assign(&mut self, rhs: &'b ByteString<'b>) {
        let iter = self.iter_mut().zip(rhs.iter());
        
        for (a,b) in iter {
            *a &= *b;
        }
    }
}

impl<'a, 'b: 'a> BitXorAssign<&'b ByteString<'b>> for ByteString<'a> {
    fn bitxor_assign(&mut self, rhs: &'b ByteString<'b>) {
        let iter = self.iter_mut().zip(rhs.iter());
        
        for (a,b) in iter {
            *a ^= *b;
        }
    }
}

impl<'a, 'b: 'a> BitOrAssign<&'b ByteString<'b>> for ByteString<'a> {
    fn bitor_assign(&mut self, rhs: &'b ByteString<'b>) {
        let iter = self.iter_mut().zip(rhs.iter());
        
        for (a,b) in iter {
            *a |= *b;
        }
    }
}

impl<'a> ShlAssign<usize> for ByteString<'a> {
    fn shl_assign(&mut self, rhs: usize) {
        
        if self.is_zero() || rhs == 0 { 
            return; 
        }
        else if rhs >= self.bit_len() {
            self.set_zero();
            return;
        }

        let shift_count = rhs % self.bit_len();
        let shift_per_byte = shift_count % 8;
        let shifted_out_bytes = shift_count / 8;

        // Perform bitshift: ignore top bytes that would be shifted out and start with lowest byte
        if shift_per_byte > 0 {
            let mut carry_bits = 0;
            for byte in self.iter_mut().skip(shifted_out_bytes).rev() {
                
                let tmp_carry_bits = *byte >> (8 - shift_per_byte);
                *byte <<= shift_per_byte;
                *byte |= carry_bits;
                carry_bits = tmp_carry_bits;
            }
        }

        // Zeroize shifted out bytes
        for byte in self.iter_mut().take(shifted_out_bytes) {
            *byte = 0;
        }

        // Rotate bytes in place
        self.rotl_bytes(shifted_out_bytes);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    //TODO: Write tests for multi byte interpret_reverse_endian left-shift

    #[test]
    fn test_shl_multi_byte_bigger_bitlen_shift() {
        let mut a = [1u8,2,3,4];
        let expected = [0;4];

        let mut bytes = ByteString::new(&mut a);
        bytes <<= bytes.bit_len() + 1;
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn test_shl_multi_byte_bitlen_shift() {
        let mut a = [1u8,2,3,4];
        let expected = [0;4];

        let mut bytes = ByteString::new(&mut a);
        bytes <<= bytes.bit_len();
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn test_shl_multi_byte_8shift() {
        let mut a = [1u8,2,3,4];
        let expected = [0x02, 0x03 ,0x04 ,0x00];

        let mut bytes = ByteString::new(&mut a);
        bytes <<= 8;
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn test_shl_multi_byte_7shift() {
        let mut a = [1u8,2,3,4];
        let expected = [0x81 ,0x01 ,0x82 ,0x00];

        let mut bytes = ByteString::new(&mut a);
        bytes <<= 7;
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn test_shl_multi_byte_0shift() {
        let mut a = [1u8,2,3,4];
        let expected = a.clone();

        let mut bytes = ByteString::new(&mut a);
        bytes <<= 0;
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn test_shl_single_byte_bigger_bitlen_shift() {
        let a = [1u8];

        let mut bytes_raw = a.clone();
        
        let mut bytes = ByteString::new(&mut bytes_raw);
        bytes <<= bytes.bit_len() + 1;
        
        let expected = [0u8];
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn test_shl_single_byte_bitlen_shift() {
        let a = [1u8];

        let mut bytes_raw = a.clone();
        
        let mut bytes = ByteString::new(&mut bytes_raw);
        bytes <<= 8;
        
        let expected = [0u8];
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn test_shl_single_byte_7shift() {
        let a = [1u8];

        let mut bytes_raw = a.clone();
        
        let mut bytes = ByteString::new(&mut bytes_raw);
        bytes <<= 7;
        
        let expected = [0x80u8];
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn test_shl_single_byte_0shift() {
        let a = [1u8];

        let mut bytes_raw = a.clone();
        
        let mut bytes = ByteString::new(&mut bytes_raw);
        bytes <<= 0;
        
        let expected = [1u8];
        assert!(bytes.iter().eq(expected.iter()));
    }

    #[test]
    fn test_iter() {

        let mut a = [1u8,2,3,4];
        let expected = a.clone();
        let bytes = ByteString::new(&mut a);

        assert!(expected.iter().eq(bytes.iter()));
    }

    #[test]
    fn test_set_zero() {

        let mut a = [1u8,2,3,4];
        let mut bytes = ByteString::new(&mut a);

        bytes.set_zero();
        assert_eq!(a, [0; 4]);
    }

    #[test]
    fn test_is_zero() {
        let mut a = [1u8,2,3,4];
        let mut bytes = ByteString::new(&mut a);

       
        bytes.set_zero();
        assert!(bytes.is_zero());
    }

    #[test]
    fn test_eq() {

        let mut a = [1u8,2,3,4];

        let mut eq_a = a.clone();

        let mut ne_a = a.clone();
        ne_a[0] = 2;

        let mut ne_endianness_a = a.clone();

        let bytes_a = ByteString::new(&mut a);
        let bytes_eq_a = ByteString::new(&mut eq_a);
        let bytes_ne_a = ByteString::new(&mut ne_a);
        let mut bytes_ne_endianness_a = ByteString::new(&mut ne_endianness_a);
        bytes_ne_endianness_a.interpret_reverse_endian();

        assert!(bytes_a == bytes_eq_a);
        assert!(bytes_a != bytes_ne_a);
        assert!(bytes_a != bytes_ne_endianness_a);
    }

    #[test]
    fn test_reinterpret() {

        let mut arr = [0xFFu8; 8];
        let (mut u16_split1, rem1) = arr.split_at_mut(2);
        let (mut u16_split2, rem2) = rem1.split_at_mut(2);
        let (mut u16_split3, rem3) = rem2.split_at_mut(2);
        let (mut u16_split4, rem4) = rem3.split_at_mut(2);

        assert!(rem4.is_empty());

        let mut u16_reintp1 = ByteString::new(&mut u16_split1);
        let mut _u16_reintp2 = ByteString::new(&mut u16_split2);
        let mut u16_reintp3 = ByteString::new(&mut u16_split3);
        let mut _u16_reintp4 = ByteString::new(&mut u16_split4);

        u16_reintp1.set_zero();
        u16_reintp3.set_zero();

        let expected = [0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0xFF, 0xFF];

        assert_eq!(arr, expected);
    }

}