extern crate mudi;
use mudi::Array;

fn main() {
    let mut array = Array::from_element(0.0, (5, 2, 9));

    // Iterations using the size of the array
    let (nx, ny, nz) = array.shape();
    for i in 0..nx {
        for j in 0..ny {
            for k in 0..nz {
                array[(i, j, k)] = (i + j + k) as f64;
            }
        }
    }

    // Linear iteration over the array
    for value in array.flat_iter() {
         print!("{} ", value);
    }
}
