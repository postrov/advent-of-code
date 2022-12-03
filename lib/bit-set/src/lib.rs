#[derive(Debug, Eq, PartialEq)]
pub enum Bit {
    Zero = 0,
    One = 1,
}

pub trait BitSet {
    fn new() -> Self;
    fn set_bit_value(&mut self, b: u8, v: Bit);
    fn set(&mut self, b: u8) {
        self.set_bit_value(b, Bit::One);
    }
    fn clear(&mut self, b: u8) {
        self.set_bit_value(b, Bit::Zero);
    }
    fn invert(&mut self, b: u8);
    fn get(&self, b: u8) -> Bit;
    fn union(&self, other: &Self) -> Self;
    fn intersection(&self, other: &Self) -> Self;
}

pub struct BitBuffer<T> {
    bits: T,
}

impl Default for BitBuffer<u128> {
    fn default() -> Self {
        Self { bits: 0u128 } 
    }
}

impl BitSet for BitBuffer<u128> {
    fn new() -> Self {
        Self::default()
    }

    fn set_bit_value(&mut self, b: u8, v: Bit) {
        assert!(b < 128);
        let mask = 1 << b;
        match v {
            Bit::Zero => self.bits &= !mask,
            Bit::One => self.bits |= mask,
        };
    }

    fn invert(&mut self, b: u8) {
        assert!(b < 128);
        self.bits ^= 1 << b;
    }

    fn get(&self, b: u8) -> Bit {
        assert!(b < 128);
        match self.bits & (1 << b) {
            0 => Bit::Zero,
            _ => Bit::One,
        }
    }

    fn union(&self, other: &Self) -> Self {
        let bits = self.bits | other.bits;
        Self { bits }
    }

    fn intersection(&self, other: &Self) -> Self {
        let bits = self.bits & other.bits;
        Self { bits }
    }
}


pub struct BitBufferIter<'a> {
    bit_buffer: &'a BitBuffer<u128>,
    pos: u8,
}

impl<'a> Iterator for BitBufferIter<'a> {
    type Item = (u8, Bit);

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.pos;
        if pos >= 128 {
            None
        } else {
            self.pos += 1;
            Some((pos, self.bit_buffer.get(pos)))
        }
    }
}

impl<'a> IntoIterator for &'a BitBuffer<u128> {
    type Item = (u8, Bit);

    type IntoIter = BitBufferIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BitBufferIter {
            bit_buffer: self,
            pos: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_works() {
        let mut s = BitBuffer::new();
        assert_eq!(Bit::Zero, s.get(4));
        s.set(4);
        assert_eq!(Bit::One, s.get(4));
    }

    #[test]
    fn invert_works() {
        let mut s = BitBuffer::new();
        s.invert(18);
        assert_eq!(Bit::One, s.get(18));
        s.invert(18);
        assert_eq!(Bit::Zero, s.get(18));
    }

    #[test]
    fn union_works() {
        let mut s = BitBuffer::new();
        let mut t = BitBuffer::new();
        s.set(5);
        t.set(15);
        let u = s.union(&t);
        assert_eq!(Bit::One, u.get(5));
        assert_eq!(Bit::One, u.get(15));
        assert_eq!(Bit::Zero, u.get(4));
    }

    #[test]
    fn intersection_works() {
        let mut s = BitBuffer::new();
        let mut t = BitBuffer::new();
        s.set(5);
        s.set(10);
        t.set(10);
        t.set(15);
        let i = s.intersection(&t);
        assert_eq!(Bit::One, i.get(10));
        assert_eq!(Bit::Zero, i.get(15));
        assert_eq!(Bit::Zero, i.get(5));
    }

    #[test]
    fn iter_works() {
        let mut s = BitBuffer::new();
        s.set(1);
        s.set(13);
        s.set(5);

        let mut max_pos = 0;
        let mut set_positions = Vec::new();
        for (pos, b) in &s {
            max_pos = pos;
            if b == Bit::One {
                set_positions.push(pos);
            }
        }
        assert_eq!(127, max_pos);
        assert_eq!(vec![1, 5, 13], set_positions);
    }

    #[test]
    #[should_panic]
    fn set_out_of_bounds_panics() {
        let mut s = BitBuffer::new();
        s.set(150);
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds_panics() {
        let s = BitBuffer::new();
        s.get(150);
    }

    #[test]
    #[should_panic]
    fn invert_out_of_bounds_panics() {
        let mut s = BitBuffer::new();
        s.invert(150);
    }
}
