use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

use Dimensions;
use Storage;

/// An array with owned storage.
pub type Array<T, D> = ArrayBase<Box<[T]>, D>;

#[derive(Debug)]
/// ArrayBase implements all the operations on arrays, using a `Storage` and
/// some `Dimensions`.
pub struct ArrayBase<S, D> where D: Dimensions, S: Storage {
    data: S,
    dims: D,
}

impl<S, D> ArrayBase<S, D> where D: Dimensions, S: Storage {
    /// Create a new array using content from the vector.
    ///
    /// ```
    /// use mudi::Array;
    /// // One-dimensional array, with shape (4).
    /// let array = Array::from_vector(vec![1, 2, 3, 4], 4);
    /// assert_eq!(array[2], 3);
    ///
    /// // Two-dimensional array, with shape (2, 2)
    /// let array = Array::from_vector(vec![1, 2, 3, 4], (2, 2));
    /// assert_eq!(array[(1, 0)], 3);
    /// ```
    ///
    /// # Panics
    /// If the size of the vector does not match the size of the dimensions.
    ///
    /// ```should_panic
    /// use mudi::Array;
    /// // The dimension size is 14 here, and the array only contains 4 elements
    /// let array = Array::from_vector(vec![1, 2, 3, 4], (2, 7));
    /// ```
    pub fn from_vector(vec: Vec<S::Item>, dims: D) -> ArrayBase<S, D> {
        assert!(vec.len() == dims.size(), "Data length does not match the dimensions");
        ArrayBase {
            data: S::from_vec(vec),
            dims: dims,
        }
    }

    /// Get the shape of the array.
    ///
    /// ```
    /// use mudi::Array;
    /// let array = Array::from_vector(vec![1, 2, 3, 4], 4);
    /// assert_eq!(array.shape(), 4);
    ///
    /// let array = Array::from_element(vec![1, 2, 3, 4], (2, 7));
    /// assert_eq!(array.shape(), (2, 7));
    /// ```
    pub fn shape(&self) -> D {
        self.dims.clone()
    }

    /// Flat (linear) iteration over array elements.
    ///
    /// ```
    /// use mudi::Array;
    /// let mut array = Array::from_element(0, (3, 2));
    /// for i in 0..3 {
    ///     for j in 0..2 {
    ///         array[(i, j)] = i;
    ///     }
    /// }
    ///
    /// for value in array.flat_iter() {
    ///     print!("{} ", value);
    /// }
    /// // This will print "0 0 1 1 2 2"
    /// ```
    pub fn flat_iter(&self) -> Iter<S::Item> {
        self.data.as_ref().iter()
    }

    /// Flat (linear) iteration over mutable array elements. See the
    /// documentation for [`Array::flat_iter`](#method.flat_iter).
    pub fn flat_iter_mut(&mut self) -> IterMut<S::Item> {
        self.data.as_mut().iter_mut()
    }
}

impl<S, D> ArrayBase<S, D> where D: Dimensions, S: Storage, S::Item: Clone {
    /// Create a new array by cloning a specific element as needed.
    ///
    /// ```
    /// use mudi::Array;
    /// // Two-dimensional array, with shape (2, 6)
    /// let array = Array::from_element(42, (2, 6));
    /// assert_eq!(array[(1, 4)], 42);
    /// ```
    pub fn from_element(element: S::Item, dims: D) -> ArrayBase<S, D> {
        let data = vec![element; dims.size()];
        ArrayBase {
            data: S::from_vec(data),
            dims: dims,
        }
    }
}

impl<S, D, I: Copy> Index<I> for ArrayBase<S, D>
    where D: Dimensions<Index = I>, S: Storage {
    type Output = S::Item;
    fn index(&self, index: I) -> &S::Item {
        &self.data.as_ref()[self.dims.offset(index)]
    }
}

impl<S, D, I: Copy> IndexMut<I> for ArrayBase<S, D>
    where D: Dimensions<Index = I>, S: Storage {
    fn index_mut(&mut self, index: I) -> &mut S::Item {
        &mut self.data.as_mut()[self.dims.offset(index)]
    }
}

impl<S, D> PartialEq for ArrayBase<S, D>
    where D: Dimensions, S: Storage, S::Item: PartialEq {
    fn eq(&self, other: &ArrayBase<S, D>) -> bool {
        if self.dims != other.dims {
            false
        } else {
            self.data.as_ref() == other.data.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexing() {
        let mut a = Array::from_vector(vec![3, 4, 5], 3);

        assert_eq!(a[0], 3);
        assert_eq!(a[1], 4);
        assert_eq!(a[2], 5);

        a[2] = 42;
        assert_eq!(a[2], 42);
    }

    #[test]
    fn from_vector() {
        let mut a = Array::from_vector(vec![3, 4, 5, 78], 4);
        assert_eq!(a[3], 78);
        a[3] = 42;
        assert_eq!(a[3], 42);
    }

    #[test]
    fn from_element() {
        let mut a = Array::from_element(678, (7, 7));
        assert_eq!(a[(3, 4)], 678);
        a[(3, 4)] = 42;
        assert_eq!(a[(3, 4)], 42);
    }

    #[test]
    fn shape() {
        let a = Array::from_element(678, (7, 7));
        assert_eq!(a.shape(), (7, 7));

        let a = Array::from_element(678, (7, 7..10));
        assert_eq!(a.shape(), (7, 7..10));
    }
}
