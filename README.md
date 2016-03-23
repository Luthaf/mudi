# Mudi: Rust multi-dimensional arrays

[![Build Status](https://travis-ci.org/Luthaf/mudi.svg?branch=master)](https://travis-ci.org/Luthaf/mudi)
[![Code Coverage](https://codecov.io/github/Luthaf/mudi/coverage.svg?branch=master)](https://codecov.io/github/Luthaf/mudi?branch=master)

Mudi provides multi-dimensional arrays to store data. The aim of this
library is not to provide complex algorithms, but powerful indexing scheme.
The Mudi array type is closer to a Fortran array than to Numpy's `ndarray`.

For example, Mudi allow to use range dimensions (`10..30` or even
`-42..42`) and negative indexing for negatives ranges.

## [Documentation](http://luthaf.github.io/mudi)

## Creating arrays

The `array!` macro can be used to create an array. It is used like the
standard `vec!` macro, but with an addition dimensions parameter.

```rust
#[macro_use]
extern crate mudi;

// Create a 3-dimensional array, filled with 0
let a = array!(0.0; (3, 4, -10..10));

// Create a 2-dimensional array, with values from the list
let a = array!(1.0, 0.0, 0.0, 0.0,
               0.0, 2.0, 1.0, 0.0,
               0.0, 0.0, 3.0, 0.0,
               0.0, 0.0, 0.0, 4.0; (4, 4));
```

## Dimensions and indexing

Dimensions are represented by tuples of either single `usize` value, or
ranges. `3` and `10..30` are dimensions for 1-dimensional arrays, and
`(3, 4, 5)` or `(-20..20, 5, 6..8)` are dimensions for 3-dimensional arrays.
Indexing is implemented for tuple dimensions up to 7-dimensional arrays.

```rust
let mut a = array!(0.0; (3, 4, -10..10));

// Use tuples for indexing
assert_eq!(a[(0, 0, 0)], 0.0);
// Negatives indexes in negative range dimensions
a[(1, 2, -6)] = 42.0;
assert_eq!(a[(1, 2, -6)], 42.0);
```

## License

Mudi is licensed under either of Apache License Version 2.0 or MIT license at
your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
