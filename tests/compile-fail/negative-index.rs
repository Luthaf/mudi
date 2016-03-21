extern crate mudi;
use mudi::Array;

fn main() {
    let array = Array::from_element(0.0, (2, 6));
    array[(1, -1)];
    //~^ error: unary negation of unsigned integer
}
