
/// Storage of data from an array
pub trait Storage {
    /// Type of items in this array
    type Item;
    /// Create the storage from a vector
    fn from_vec(vector: Vec<Self::Item>) -> Self;
    /// Convert the storage to a slice
    fn as_ref(&self) -> &[Self::Item];
    /// Convert the storage to a mutable slice
    fn as_mut(&mut self) -> &mut [Self::Item];
}

impl<T> Storage for Box<[T]> {
    type Item = T;

    fn from_vec(vector: Vec<T>) -> Box<[T]> {
        vector.into_boxed_slice()
    }

    fn as_ref(&self) -> &[T] {
        &self
    }

    fn as_mut(&mut self) -> &mut [T] {
        &mut *self
    }
}
