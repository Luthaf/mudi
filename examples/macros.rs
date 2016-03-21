#[macro_use]
extern crate mudi;

fn main() {
    let array = array![3.0, 4.0, 5.0,
                       6.0, 7.0, 8.0,
                       9.0, 1.0, 2.0,
                       3.0, 4.0, 5.0; (3, 4)];

    let (nx, ny) = array.shape();
    for i in 0..nx {
        for j in 0..ny {
            println!("{:?}", array[(i, j)]);
        }
    }
}
