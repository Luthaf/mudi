use std::ops::Range;

/// A (set of) dimensions in an array. A `Dimensions` object carries informations
/// about its size, and how to convert a multi-dimensional index to a linear
/// index.
pub trait Dimensions: Clone {
    /// The type to use to index this dimension
    type Index: Copy;
    /// Convert an index to a linear offset for this dimension size
    fn offset(&self, index: Self::Index) -> usize;
    /// Get the number of elements in this dimension
    fn size(&self) -> usize;
}


impl Dimensions for usize {
    type Index = usize;

    #[inline(always)]
    fn offset(&self, index: usize) -> usize {
        assert!(index < *self);
        index
    }

    #[inline(always)]
    fn size(&self) -> usize {
        *self
    }
}

impl Dimensions for Range<usize> {
    type Index = usize;

    #[inline(always)]
    fn offset(&self, index: usize) -> usize {
        assert!(self.start <= index && index < self.end);
        index - self.start
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.end - self.start
    }
}

impl Dimensions for Range<i32> {
    type Index = i32;

    #[inline(always)]
    fn offset(&self, index: i32) -> usize {
        assert!(self.start <= index && index < self.end);
        assert!(index - self.start > 0);
        (index - self.start) as usize
    }

    #[inline(always)]
    fn size(&self) -> usize {
        (self.end - self.start) as usize
    }
}


impl<A, B> Dimensions for (A, B) where A: Dimensions, B: Dimensions {
    type Index = (A::Index, B::Index);

    #[inline(always)]
    fn offset(&self, index: Self::Index) -> usize {
        self.1.size() * self.0.offset(index.0) + self.1.offset(index.1)
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.0.size() * self.1.size()
    }
}

impl<A, B, C> Dimensions for (A, B, C) where A: Dimensions, B: Dimensions, C: Dimensions {
    type Index = (A::Index, B::Index, C::Index);

    #[inline(always)]
    fn offset(&self, index: Self::Index) -> usize {
        self.2.size() * (self.1.size() * self.0.offset(index.0) + self.1.offset(index.1)) + self.2.offset(index.2)
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.0.size() * self.1.size() * self.2.size()
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    mod urange {
        use super::*;
        #[test]
        fn offset() {
            let dim = 4..10;
            assert_eq!(dim.size(), 6);
            assert_eq!(dim.offset(5), 1);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim() {
            let dim = 4..10;
            dim.offset(11);
        }

        #[test]
        #[should_panic]
        fn smaller_than_dim() {
            let dim = 4..10;
            dim.offset(2);
        }
    }

    mod irange {
        use super::*;
        #[test]
        fn offset() {
            let dim = -4..10;
            assert_eq!(dim.size(), 14);
            assert_eq!(dim.offset(5), 9);
            assert_eq!(dim.offset(-3), 1);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim() {
            let dim = -4..10;
            dim.offset(11);
        }

        #[test]
        #[should_panic]
        fn smaller_than_dim() {
            let dim = -4..10;
            dim.offset(-6);
        }
    }

    mod dim1 {
        use super::*;
        #[test]
        fn offset() {
            let dim = 8;
            assert_eq!(dim.size(), 8);
            assert_eq!(dim.offset(3), 3);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim() {
            let dim = 4;
            dim.offset(5);
        }
    }

    mod dim2 {
        use super::*;
        #[test]
        fn offset() {
            let dim = (8, 6);
            assert_eq!(dim.size(), 48);
            assert_eq!(dim.offset((3, 5)), 23);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_one() {
            let dim = (8, 6);
            dim.offset((9, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_two() {
            let dim = (8, 6);
            dim.offset((7, 8));
        }
    }

    mod dim3 {
        use super::*;
        #[test]
        fn offset() {
            let dim = (2, 6, 6);
            assert_eq!(dim.size(), 72);
            assert_eq!(dim.offset((1, 5, 3)), 69);

            let dim = (1, 2, 3);
            assert_eq!(dim.size(), 6);
            assert_eq!(dim.offset((0, 1, 2)), 5);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_one() {
            let dim = (2, 6, 6);
            dim.offset((9, 4, 3));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_two() {
            let dim = (2, 6, 6);
            dim.offset((1, 8, 3));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_three() {
            let dim = (2, 6, 6);
            dim.offset((1, 4, 12));
        }
    }
}
