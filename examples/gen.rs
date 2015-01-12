extern crate nd_iter;
use nd_iter::iter_2d;
use nd_iter::iter_3d;
use std::iter::range_inclusive;

fn main() {
    for (x, y) in iter_2d(range(0u32, 2), range_inclusive(2u32, 5)) {
        println!("{}, {}", x, y);
    }

    for (x, y, z) in iter_3d(range(0u32, 2), range(2u32, 4), range(0u32, 2)) {
        println!("{}, {}, {}", x, y, z);
    }
}

