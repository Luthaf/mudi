#[macro_export]
/// Creates an Array containing the arguments, like the `vec!` macro.
///
/// This macro can be used either to create an Array containing a given list of
/// elements:
///
/// ```
/// # #[macro_use] extern crate mudi;
/// # fn main() {
/// let a = array![3, 4, 5,
///                6, 7, 8; (2, 3)];
/// assert_eq!(a[(0, 1)], 4);
/// assert_eq!(a[(1, 2)], 8);
/// # }
/// ```
///
/// Or to create an Array from a given element and dimensions:
///
/// ```
/// # #[macro_use] extern crate mudi;
/// # fn main() {
/// let a = array![0.0; (-3..3, 2)];
/// assert_eq!(a[(-1, 0)], 0.0);
/// assert_eq!(a[(2, 1)], 0.0);
/// # }
/// ```
///
/// Like for the `vec!` macro, the dimensions of the array doesn't have to be a
/// constant.
macro_rules! array {
    ($value: expr; $dimensions: expr) => ({
        let dims = $dimensions;
        $crate::Array::from_element($value, dims)
    });
    ($($values: expr), *; $dimensions: expr) => (
        $crate::Array::from_vector(vec![$($values, )*], $dimensions);
    );
}

#[cfg(test)]
mod tests {

    #[test]
    fn from_value() {
        let array = array![0.0; (4, 5, 6)];

        for val in array.flat_iter() {
            assert_eq!(*val, 0.0);
        }
    }

    #[test]
    fn from_vector() {
        let array = array![0.0, 1.0, 2.0, 3.0; (2, 2)];

        assert_eq!(array, ::Array::from_vector(vec![0.0, 1.0, 2.0, 3.0], (2, 2)));
    }
}
