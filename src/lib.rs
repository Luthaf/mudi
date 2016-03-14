#![warn(missing_docs)]

//! `mudi` is a multi-dimensional array library. It provides multi-dimensional
//! arrays which are just storage for some data, and do not aims at providing
//! complex algorithms. Instead, it provides powerful indexing scheme, and
//! iterators. For example, it is possible to use range dimensions (`-42..42`)
//! and the corresponding negative indexing (`array[-23]`).
//!
//! `mudi` Array type is closer to Fortran array that to Numpy's `ndarray`. The
//! fact that negative indexing is supported means that python style *indexing
//! from the end* will not be supported. If the dimension do not contains
//! negative range then negative indexing is disalowed:
//!
//! ```no_run
//! use mudi::Array;
//! let array = Array::from_element(0.0, (2, 6));
//! // This fails to compile
//! array[(1, -1)];
//! ```
//!
//! And if the dimensions contains a negative range, then negative indexing
//! will yield the corresponding value
//!
//! ```
//! use mudi::Array;
//! let array = Array::from_element(0.0, (-42..42));
//! assert_eq!(array[-12], 0.0);
//! ```
//!
//! The basic type is `Array`, which provides an owned array for any kind of
//! data. The size of an array is fixed when the array is first constructed.

mod dimensions;
pub use dimensions::Dimensions;

mod storage;
pub use storage::Storage;

mod arrays;
pub use arrays::{Array, ArrayBase};
