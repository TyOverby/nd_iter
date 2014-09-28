extern crate range_nd;
use range_nd::range_2d;

fn main() {
    // Iterate through the 2d range with x from [1 to 3) and
    // with y from [3, 5).
    for (x, y) in range_2d((1u, 3), (3, 5)) {
        println!("({}, {})", x, y);
    }
    // (1, 3) (1, 4) (2, 3) (2, 4)
}
