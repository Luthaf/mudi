#![warn(missing_docs)]
#![cfg_attr(feature="lint", feature(plugin))]
#![cfg_attr(feature="lint", plugin(clippy))]
#![cfg_attr(feature="lint", warn(clippy))]
// Additional lints from the Allow group in clippy
#![cfg_attr(feature="lint", warn(
    enum_glob_use, mut_mut, option_unwrap_used, print_stdout,
    result_unwrap_used, single_match_else, wrong_pub_self_convention
))]

#![cfg_attr(feature="lint", allow(inline_always))]

//! `mudi` is a multi-dimensional array library.
//!
//! `mudi` provides multi-dimensional arrays to stora data.
//! The aim of this library is not to provide complex algorithms, but a
//! powerful indexing scheme and iterators.
//! For example, with `mudi` it is possible to use range dimensions
//! (`-42..42`) and the corresponding negative indexing (`array[-23]`).
//!
//! The `mudi` array type is closer to a Fortran array than to Numpy's
//! `ndarray`. The fact that negative indexing is supported means that python
//! style *indexing from the end* will not be supported.
//! If the dimension does not contain a negative range then negative indexing
//! is not permitted:
//!
//! ```
//! use mudi::Array;
//! let array = Array::from_element(0.0, (2, 6));
//! // This fails to compile
//! // array[(1, -1)];
//! ```
//!
//! If the dimensions contain a negative range, negative indexing
//! will yield the corresponding value
//!
//! ```
//! use mudi::Array;
//! let array = Array::from_element(0.0, (-42..42));
//! assert_eq!(array[-12], 0.0);
//! ```
//!
//! The basic type is `Array`, which provides an owned array for any kind of
//! data. The size of the array is fixed at initial construction.

mod dimensions;
pub use dimensions::Dimensions;

mod storage;
pub use storage::Storage;

mod arrays;
pub use arrays::{Array, ArrayBase};
