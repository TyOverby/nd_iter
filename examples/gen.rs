extern crate iter_nd;
use iter_nd::iter_2d;
use iter_nd::iter_3d;
use std::iter::range_inclusive;

fn main() {
    for (x, y) in iter_2d(range(0u, 2), range_inclusive(2u, 5)) {
        println!("{}, {}", x, y);
    }

    for (x, y, z) in iter_3d(range(0u, 2), range(2u, 4), range(0u, 2)) {
        println!("{}, {}, {}", x, y, z);
    }
}

