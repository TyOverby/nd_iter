extern crate range_nd;
use range_nd::range_3d;

fn main() {
    // Iterate through the 3d range with x from [1 to 3) and
    // with y from [3, 5) and z from [10, 12).
    for (x, y, z) in range_3d((1u, 3), (3, 5), (10, 12)) {
        println!("({}, {}, {})", x, y, z);
    }
    // (1, 3, 10) (1, 3, 11) (1, 4, 10) (1, 4, 11) ...
}
