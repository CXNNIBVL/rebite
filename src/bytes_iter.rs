use std::{
    iter::Rev,
    ops::Range
};

enum Index {
    Iter(Range<usize>),
    RevIter(Rev<Range<usize>>)
}

impl Index {
    pub fn new_iter(range: Range<usize>) -> Self {
        Self::Iter(range)
    }   

    pub fn new_rev_iter(range: Range<usize>) -> Self {
        Self::RevIter(range.rev())
    }

    pub fn next(&mut self) -> Option<usize> {
        use Index::{Iter, RevIter};

        match self {
            Iter(it) => it.next(),
            RevIter(it) => it.next()
        }
    }

    pub fn next_back(&mut self) -> Option<usize> {
        use Index::{Iter, RevIter};

        match self {
            Iter(it) => it.next_back(),
            RevIter(it) => it.next_back()
        }
    }

    pub fn size_hint(&self) -> (usize, Option<usize>) {
        use Index::{Iter, RevIter};

        match self {
            Iter(it) => it.size_hint(),
            RevIter(it) => it.size_hint()
        }
    }


}

pub struct BytesIter<'a> {
    bytes: &'a [u8],
    index: Index
}

impl<'a> BytesIter<'a> {
    pub fn new(bytes: &'a [u8], should_iter_reverse: bool) -> Self {

        let range = 0..bytes.len();

        let index = if should_iter_reverse { Index::new_rev_iter(range) } 
        else { Index::new_iter(range) };

        Self { bytes, index }
    }
}

impl<'a> Iterator for BytesIter<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        
        if let Some(ix) = self.index.next() {
            return Some(&self.bytes[ix])
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.index.size_hint()
    }
}

impl<'a> DoubleEndedIterator for BytesIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(ix) = self.index.next_back() {
            return Some(&self.bytes[ix])
        }

        None
    }
}

impl<'a> ExactSizeIterator for BytesIter<'a> {}


pub struct BytesIterMut<'a> {
    bytes: &'a mut [u8],
    index: Index
}

impl<'a> BytesIterMut<'a> {
    pub fn new(bytes: &'a mut [u8], should_iter_reverse: bool) -> Self {

        let range = 0..bytes.len();

        let index = if should_iter_reverse { Index::new_rev_iter(range) } 
        else { Index::new_iter(range) };

        Self { bytes, index }
    }
}

impl<'a> Iterator for BytesIterMut<'a> {
    type Item = &'a mut u8;

    fn next(&mut self) -> Option<Self::Item> {
        
        if let Some(ix) = self.index.next() {
            /* 
                This is ok, since we know the index does exits
                as per the creation of our Index iterator
            */
            unsafe {
                let ptr = self.bytes.as_mut_ptr();
                return Some(&mut *ptr.add(ix))
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.index.size_hint()
    }
}

impl<'a> DoubleEndedIterator for BytesIterMut<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(ix) = self.index.next_back() {
            /* 
                This is ok, since we know the index does exits
                as per the creation of our Index iterator
            */
            unsafe {
                let ptr = self.bytes.as_mut_ptr();
                return Some(&mut *ptr.add(ix))
            }
        }

        None
    }
}

impl<'a> ExactSizeIterator for BytesIterMut<'a>{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_len() {
        let a = [1u8,2,3,4];
        let it = BytesIter::new(&a, false);

        assert_eq!(it.len(), 4);
        
        let skipped_once = it.skip(1);
        assert_eq!(skipped_once.len(), 3);

        let skipped_thrice = skipped_once.skip(2);
        assert_eq!(skipped_thrice.len(), 1);

        let skipped_fourth = skipped_thrice.skip(1);
        assert_eq!(skipped_fourth.len(), 0);
    }

    #[test]
    fn test_iter_mut_len() {
        let mut a = [1u8,2,3,4];
        let it = BytesIterMut::new(&mut a, false);

        assert_eq!(it.len(), 4);
        
        let skipped_once = it.skip(1);
        assert_eq!(skipped_once.len(), 3);

        let skipped_thrice = skipped_once.skip(2);
        assert_eq!(skipped_thrice.len(), 1);

        let skipped_fourth = skipped_thrice.skip(1);
        assert_eq!(skipped_fourth.len(), 0);
    }

    #[test]
    fn test_iter_rev() {
        let a = [1u8,2,3,4];
        let b = a.clone();

        let it = BytesIter::new(&a, false).rev();
        assert!(it.eq(b.iter().rev()));
    }

    #[test]
    fn test_iter_mut_rev() {
        let mut a = [1u8,2,3,4];
        let b = a.clone();

        let it = BytesIterMut::new(&mut a, false).rev();
        assert!(it.eq(b.iter().rev()));
    }

    #[test]
    fn test_iter_be_as_be() {

        let a = [1u8,2,3,4];
        let b = a.clone();

        let it = BytesIter::new(&a, false);
        assert!(it.eq(b.iter()));
    }

    #[test]
    fn test_iter_be_as_le() {

        let a = [1u8,2,3,4];
        let b = a.clone();

        let it = BytesIter::new(&a, true);
        assert!(it.eq(b.iter().rev()));
    }

    #[test]
    fn test_iter_mut_be_as_be() {

        let mut a = [1u8,2,3,4];
        let b = a.clone();

        let it = BytesIterMut::new(&mut a, false);
        assert!(it.eq(b.iter()));
    }

    #[test]
    fn test_iter_mut_be_as_le() {

        let mut a = [1u8,2,3,4];
        let b = a.clone();

        let it = BytesIterMut::new(&mut a, true);
        assert!(it.eq(b.iter().rev()));
    }
}