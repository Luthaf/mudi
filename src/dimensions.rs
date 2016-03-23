use std::ops::Range;

/// A (set of) dimensions in an array. `Dimensions` objects carry informations
/// about there size, and how to convert a multi-dimensional index to a linear
/// offset.
pub trait Dimensions: Clone + PartialEq {
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
        assert!(index < *self, "index out of bound: len is {} but index is {}", self, index);
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
        assert!(self.start <= index && index < self.end,
                "index out of bound: range is ({}..{}) but index is {}",
                self.start, self.end, index);
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
        assert!(self.start <= index && index < self.end,
                "index out of bound: range is ({}..{}) but index is {}",
                self.start, self.end, index);
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

impl<A, B, C> Dimensions for (A, B, C)
    where A: Dimensions, B: Dimensions, C: Dimensions {
    type Index = (A::Index, B::Index, C::Index);

    #[inline(always)]
    fn offset(&self, index: Self::Index) -> usize {
        self.2.offset(index.2) + self.2.size() * (
            self.1.offset(index.1) + self.1.size() * self.0.offset(index.0)
        )
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.0.size() * self.1.size() * self.2.size()
    }
}

impl<A, B, C, D> Dimensions for (A, B, C, D)
    where A: Dimensions, B: Dimensions, C: Dimensions, D:Dimensions {
    type Index = (A::Index, B::Index, C::Index, D::Index);

    #[inline(always)]
    fn offset(&self, index: Self::Index) -> usize {
        self.3.offset(index.3) + self.3.size() * (
            self.2.offset(index.2) + self.2.size() * (
                self.1.offset(index.1) + self.1.size() * self.0.offset(index.0)
            )
        )
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.0.size() * self.1.size() * self.2.size() * self.3.size()
    }
}

impl<A, B, C, D, E> Dimensions for (A, B, C, D, E)
    where A: Dimensions, B: Dimensions, C: Dimensions, D:Dimensions, E:Dimensions {
    type Index = (A::Index, B::Index, C::Index, D::Index, E::Index);

    #[inline(always)]
    fn offset(&self, index: Self::Index) -> usize {
        self.4.offset(index.4) + self.4.size() * (
            self.3.offset(index.3) + self.3.size() * (
                self.2.offset(index.2) + self.2.size() * (
                    self.1.offset(index.1) + self.1.size() * self.0.offset(index.0)
                )
            )
        )
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.0.size() * self.1.size() * self.2.size() * self.3.size() * self.4.size()
    }
}

impl<A, B, C, D, E, F> Dimensions for (A, B, C, D, E, F)
    where A: Dimensions, B: Dimensions, C: Dimensions, D:Dimensions,
          E: Dimensions, F:Dimensions{
    type Index = (A::Index, B::Index, C::Index, D::Index, E::Index, F::Index);

    #[inline(always)]
    fn offset(&self, index: Self::Index) -> usize {
        self.5.offset(index.5) + self.5.size() * (
            self.4.offset(index.4) + self.4.size() * (
                self.3.offset(index.3) + self.3.size() * (
                    self.2.offset(index.2) + self.2.size() * (
                        self.1.offset(index.1) + self.1.size() * self.0.offset(index.0)
                    )
                )
            )
        )
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.0.size() * self.1.size() * self.2.size() *
        self.3.size() * self.4.size() * self.5.size()
    }
}

impl<A, B, C, D, E, F, G> Dimensions for (A, B, C, D, E, F, G)
    where A: Dimensions, B: Dimensions, C: Dimensions, D:Dimensions,
          E: Dimensions, F: Dimensions, G:Dimensions {
    type Index = (A::Index, B::Index, C::Index, D::Index, E::Index, F::Index, G::Index);

    #[inline(always)]
    fn offset(&self, index: Self::Index) -> usize {
        self.6.offset(index.6) + self.6.size() * (
            self.5.offset(index.5) + self.5.size() * (
                self.4.offset(index.4) + self.4.size() * (
                    self.3.offset(index.3) + self.3.size() * (
                        self.2.offset(index.2) + self.2.size() * (
                            self.1.offset(index.1) + self.1.size() * self.0.offset(index.0)
                        )
                    )
                )
            )
        )
    }

    #[inline(always)]
    fn size(&self) -> usize {
        self.0.size() * self.1.size() * self.2.size() *
        self.3.size() * self.4.size() * self.5.size() * self.6.size()
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
            let dim = (2, 3);
            assert_eq!(dim.size(), 6);

            assert_eq!(dim.offset((0, 1)), 1);
            assert_eq!(dim.offset((1, 0)), 3);

            assert_eq!(dim.offset((1, 2)), 5);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_1() {
            let dim = (8, 6);
            dim.offset((8, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_2() {
            let dim = (8, 6);
            dim.offset((7, 8));
        }
    }

    mod dim3 {
        use super::*;
        #[test]
        fn offset() {
            let dim = (2, 2, 5);
            assert_eq!(dim.size(), 20);

            assert_eq!(dim.offset((0, 0, 1)), 1);
            assert_eq!(dim.offset((0, 1, 0)), 5);
            assert_eq!(dim.offset((1, 0, 0)), 10);

            assert_eq!(dim.offset((1, 1, 4)), 19);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_1() {
            let dim = (2, 6, 6);
            dim.offset((9, 4, 3));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_2() {
            let dim = (2, 6, 6);
            dim.offset((1, 8, 3));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_3() {
            let dim = (2, 6, 6);
            dim.offset((1, 4, 12));
        }
    }

    mod dim4 {
        use super::*;
        #[test]
        fn offset() {
            let dim = (2, 2, 5, 3);
            assert_eq!(dim.size(), 60);

            assert_eq!(dim.offset((0, 0, 0, 1)), 1);
            assert_eq!(dim.offset((0, 0, 1, 0)), 3);
            assert_eq!(dim.offset((0, 1, 0, 0)), 15);
            assert_eq!(dim.offset((1, 0, 0, 0)), 30);

            assert_eq!(dim.offset((1, 1, 4, 2)), 59);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_1() {
            let dim = (2, 6, 6, 42);
            dim.offset((9, 4, 3, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_2() {
            let dim = (2, 6, 6, 42);
            dim.offset((1, 8, 3, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_3() {
            let dim = (2, 6, 6, 42);
            dim.offset((1, 4, 6, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_4() {
            let dim = (2, 6, 6, 42);
            dim.offset((1, 4, 3, 45));
        }
    }

    mod dim5 {
        use super::*;
        #[test]
        fn offset() {
            let dim = (2, 2, 5, 3, 10);
            assert_eq!(dim.size(), 600);

            assert_eq!(dim.offset((0, 0, 0, 0, 1)), 1);
            assert_eq!(dim.offset((0, 0, 0, 1, 0)), 10);
            assert_eq!(dim.offset((0, 0, 1, 0, 0)), 30);
            assert_eq!(dim.offset((0, 1, 0, 0, 0)), 150);
            assert_eq!(dim.offset((1, 0, 0, 0, 0)), 300);

            assert_eq!(dim.offset((1, 1, 4, 2, 9)), 599);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_1() {
            let dim = (2, 6, 6, 42, 28);
            dim.offset((890, 4, 3, 40, 20));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_2() {
            let dim = (2, 6, 6, 42, 28);
            dim.offset((1, 6, 3, 40, 20));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_3() {
            let dim = (2, 6, 6, 42, 28);
            dim.offset((1, 4, 8, 40, 20));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_4() {
            let dim = (2, 6, 6, 42, 28);
            dim.offset((1, 4, 3, 130, 20));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_5() {
            let dim = (2, 6, 6, 42, 28);
            dim.offset((1, 4, 3, 40, 40));
        }
    }

    mod dim6 {
        use super::*;
        #[test]
        fn offset() {
            let dim = (2, 2, 2, 3, 2, 5);
            assert_eq!(dim.size(), 240);

            assert_eq!(dim.offset((0, 0, 0, 0, 0, 1)), 1);
            assert_eq!(dim.offset((0, 0, 0, 0, 1, 0)), 5);
            assert_eq!(dim.offset((0, 0, 0, 1, 0, 0)), 10);
            assert_eq!(dim.offset((0, 0, 1, 0, 0, 0)), 30);
            assert_eq!(dim.offset((0, 1, 0, 0, 0, 0)), 60);
            assert_eq!(dim.offset((1, 0, 0, 0, 0, 0)), 120);

            assert_eq!(dim.offset((1, 1, 1, 2, 1, 4)), 239);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_1() {
            let dim = (2, 6, 6, 42, 28, 98);
            dim.offset((890, 4, 3, 40, 20, 70));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_2() {
            let dim = (2, 6, 6, 42, 28, 98);
            dim.offset((1, 6, 3, 40, 20, 70));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_3() {
            let dim = (2, 6, 6, 42, 28, 98);
            dim.offset((1, 4, 8, 40, 20, 70));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_4() {
            let dim = (2, 6, 6, 42, 28, 98);
            dim.offset((1, 4, 3, 130, 20, 70));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_5() {
            let dim = (2, 6, 6, 42, 28, 98);
            dim.offset((1, 4, 3, 40, 40, 70));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_6() {
            let dim = (2, 6, 6, 42, 28, 98);
            dim.offset((1, 4, 3, 40, 20, 100));
        }
    }

    mod dim7 {
        use super::*;
        #[test]
        fn offset() {
            let dim = (2, 2, 5, 3, 4, 5, 6);
            assert_eq!(dim.size(), 7200);

            assert_eq!(dim.offset((0, 0, 0, 0, 0, 0, 1)), 1);
            assert_eq!(dim.offset((0, 0, 0, 0, 0, 1, 0)), 6);
            assert_eq!(dim.offset((0, 0, 0, 0, 1, 0, 0)), 30);
            assert_eq!(dim.offset((0, 0, 0, 1, 0, 0, 0)), 120);
            assert_eq!(dim.offset((0, 0, 1, 0, 0, 0, 0)), 360);
            assert_eq!(dim.offset((0, 1, 0, 0, 0, 0, 0)), 1800);
            assert_eq!(dim.offset((1, 0, 0, 0, 0, 0, 0)), 3600);

            assert_eq!(dim.offset((1, 1, 4, 2, 3, 4, 5)), 7199);
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_1() {
            let dim = (2, 6, 6, 42, 28, 98, 6);
            dim.offset((890, 4, 3, 40, 20, 70, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_2() {
            let dim = (2, 6, 6, 42, 28, 98, 6);
            dim.offset((1, 6, 3, 40, 20, 70, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_3() {
            let dim = (2, 6, 6, 42, 28, 98, 6);
            dim.offset((1, 4, 8, 40, 20, 70, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_4() {
            let dim = (2, 6, 6, 42, 28, 98, 6);
            dim.offset((1, 4, 3, 130, 20, 70, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_5() {
            let dim = (2, 6, 6, 42, 28, 98, 6);
            dim.offset((1, 4, 3, 40, 40, 70, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_6() {
            let dim = (2, 6, 6, 42, 28, 98, 6);
            dim.offset((1, 4, 3, 40, 20, 100, 4));
        }

        #[test]
        #[should_panic]
        fn bigger_than_dim_7() {
            let dim = (2, 6, 6, 42, 28, 98, 6);
            dim.offset((1, 4, 3, 40, 20, 70, 9));
        }
    }
}
