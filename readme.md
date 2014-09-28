# range_nd
### 2d and 3d range functions in Rust

```rust
// 2d
pub fn range_2d<A: ...>(x_rng: (A, A), y_rng: (A, A)) -> ...
pub fn range_2d_step<A: ...>(x_rng: (A, A, A), y_rng: (A, A, A)) -> ...
pub fn range_2d_inclusive<A: ...>(x_rng: (A, A), y_rng: (A, A)) -> ...
pub fn range_2d_step_inclusive<A: ...>(x_rng: (A, A, A), y_rng: (A, A, A)) ->...

// 3d
pub fn range_3d<A: ...>(x_rng: (A, A), y_rng: (A, A), z_rng: (A, A)) -> ...
pub fn range_3d_step<A: ...>(x_rng: (A, A, A), y_rng: (A, A, A), z_rng: (A, A, A)) -> ...
pub fn range_3d_inclusive<A: ...>(x_rng: (A, A), y_rng: (A, A), z_rng: (A, A)) -> ...
pub fn range_3d_step_inclusive<A: ...>(x_rng: (A, A, A), y_rng: (A, A, A), z_rng: (A, A, A)) ->...

```

## Examples
### 2d
```rust
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

```
### 3d
```rust
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

```
