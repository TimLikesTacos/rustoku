use crate::bitset::*;

impl<'a, T: BitSetInt> IntoIterator for &'a BitSet<T> {
    type Item = Bit<T>;
    type IntoIter = BitSetIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        BitSetIter {
            set: self,
            mark: Some(Bit(T::one())),
        }
    }
}

pub struct BitSetIter<'a, T: BitSetInt> {
    pub(crate) set: &'a BitSet<T>,
    pub(crate) mark: Option<Bit<T>>,
}

impl<'a, T: BitSetInt> BitSetIter<'a, T> {
    pub(crate) fn new(bitset: &'a BitSet<T>) -> BitSetIter<'a, T> {
        BitSetIter {
            set: bitset,
            mark: Some(Bit::zero()),
        }
    }
}

impl<'a, T: BitSetInt> Iterator for BitSetIter<'a, T> {
    type Item = Bit<T>;

    fn next(&mut self) -> Option<Bit<T>> {
        loop {
            if let Some(mark) = self.mark {
                if mark.0 > self.set.0 {
                    return None;
                }
                if self.set.contains(mark) {
                    self.mark = mark.inc();
                    return Some(mark);
                } else {
                    self.mark = mark.inc();
                }
            } else {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod bitset_iter {
    use super::*;

    #[test]
    fn itert() {
        let b = BitSet(0b110);
        let mut it = b.into_iter();
        let two = Bit(0b10);
        let three = Bit(0b100);
        assert_eq!(it.next().unwrap(), two);
        assert_eq!(it.next().unwrap(), three);
        assert!(it.next().is_none());

        let v: Vec<Bit<u32>> = b.into_iter().collect();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], Bit(0b10));
        assert_eq!(v[1], Bit(0b100));
    }
}
