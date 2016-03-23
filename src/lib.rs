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

//! `mudi` provides Fortran-like multi-dimensional arrays to Rust.
//!
//! `mudi` provides multi-dimensional arrays to store data. The aim of this
//! library is not to provide complex algorithms, but powerful indexing scheme.
//! The `mudi` array type is closer to a Fortran array than to Numpy's `ndarray`.
//!
//! For example, `mudi` allow to use range dimensions (`10..30` or even
//! `-42..42`) and negative indexing for negatives ranges.
//!
//! The basic type is `Array`, which provides an owned array for any kind of
//! data. The size of the array is fixed at initial construction.
//!
//! # Creating arrays
//!
//! The `array!` macro can be used to create an array. It is used like the
//! standard `vec!` macro, but with an addition dimensions parameter.
//!
//! ```
//! #[macro_use]
//! extern crate mudi;
//!
//! # fn main() {
//! // Create a 3-dimensional array, filled with 0
//! let a = array!(0.0; (3, 4, -10..10));
//!
//! // Create a 2-dimensional array, with values from the list
//! let a = array!(1.0, 0.0, 0.0, 0.0,
//!                0.0, 2.0, 1.0, 0.0,
//!                0.0, 0.0, 3.0, 0.0,
//!                0.0, 0.0, 0.0, 4.0; (4, 4));
//! # }
//! ```
//!
//! # Dimensions and indexing
//!
//! Dimensions are represented by tuples of either single `usize` value, or
//! ranges. `3` and `10..30` are dimensions for 1-dimensional arrays, and
//! `(3, 4, 5)` or `(-20..20, 5, 6..8)` are dimensions for 3-dimensional arrays.
//! Indexing is implemented for tuple dimensions up to 7-dimensional arrays.
//!
//! ```
//! # #[macro_use]
//! # extern crate mudi;
//! # fn main() {
//! let mut a = array!(0.0; (3, 4, -10..10));
//!
//! // Use tuples for indexing
//! assert_eq!(a[(0, 0, 0)], 0.0);
//! // Negatives indexes in negative range dimensions
//! a[(1, 2, -6)] = 42.0;
//! assert_eq!(a[(1, 2, -6)], 42.0);
//! # }
//! ```

mod dimensions;
pub use dimensions::Dimensions;

mod storage;
pub use storage::Storage;

mod arrays;
pub use arrays::{Array, ArrayBase};

#[macro_use]
mod macros;
