use azalea_buf::{BufReadError, McBufReadable, McBufWritable};
use std::io::{Read, Write};

/// Represents Java's BitSet, a list of bits.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct BitSet {
    data: Vec<u64>,
}

// the Index trait requires us to return a reference, but we can't do that
impl BitSet {
    pub fn new(size: usize) -> Self {
        BitSet {
            data: vec![0; size.div_ceil(64)],
        }
    }

    pub fn index(&self, index: usize) -> bool {
        (self.data[index / 64] & (1u64 << (index % 64))) != 0
    }

    fn check_range(&self, from_index: usize, to_index: usize) {
        assert!(
            from_index <= to_index,
            "fromIndex: {} > toIndex: {}",
            from_index,
            to_index
        );
    }

    fn word_index(&self, bit_index: usize) -> usize {
        bit_index >> 6
    }

    pub fn clear(&mut self, from_index: usize, mut to_index: usize) {
        self.check_range(from_index, to_index);

        if from_index == to_index {
            return;
        }

        let start_word_index = self.word_index(from_index);
        if start_word_index >= self.data.len() {
            return;
        }

        let mut end_word_index = self.word_index(to_index - 1);
        if end_word_index >= self.data.len() {
            to_index = self.len();
            end_word_index = self.data.len() - 1;
        }

        let first_word_mask = u64::MAX << from_index;
        let last_word_mask = u64::MAX >> (64 - (to_index % 64));
        if start_word_index == end_word_index {
            // Case 1: One word
            self.data[start_word_index] &= !(first_word_mask & last_word_mask);
        } else {
            // Case 2: Multiple words
            // Handle first word
            self.data[start_word_index] &= !first_word_mask;

            // Handle intermediate words, if any
            for i in start_word_index + 1..end_word_index {
                self.data[i] = 0;
            }

            // Handle last word
            self.data[end_word_index] &= !last_word_mask;
        }
    }

    /// Returns the maximum potential items in the BitSet. This will be divisible by 64.
    fn len(&self) -> usize {
        self.data.len() * 64
    }
}

impl McBufReadable for BitSet {
    fn read_from(buf: &mut impl Read) -> Result<Self, BufReadError> {
        Ok(Self {
            data: Vec::<u64>::read_from(buf)?,
        })
    }
}

impl McBufWritable for BitSet {
    fn write_into(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        self.data.write_into(buf)
    }
}

impl BitSet {
    pub fn set(&mut self, bit_index: usize) {
        self.data[bit_index / 64] |= 1u64 << (bit_index % 64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset() {
        let mut bitset = BitSet::new(64);
        assert_eq!(bitset.index(0), false);
        assert_eq!(bitset.index(1), false);
        assert_eq!(bitset.index(2), false);
        bitset.set(1);
        assert_eq!(bitset.index(0), false);
        assert_eq!(bitset.index(1), true);
        assert_eq!(bitset.index(2), false);
    }

    #[test]
    fn test_clear() {
        let mut bitset = BitSet::new(128);
        bitset.set(62);
        bitset.set(63);
        bitset.set(64);
        bitset.set(65);
        bitset.set(66);

        bitset.clear(63, 65);

        assert_eq!(bitset.index(62), true);
        assert_eq!(bitset.index(63), false);
        assert_eq!(bitset.index(64), false);
        assert_eq!(bitset.index(65), true);
        assert_eq!(bitset.index(66), true);
    }
}
