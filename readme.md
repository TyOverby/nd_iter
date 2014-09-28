# iter_nd
### Create 2 and 3 dimensional iterations

Have you ever wanted to iterate through 2 or 3 dimensional space?  You are
probably stuck writing something like this:
```rust
for x in range(0, n) {
    for y in range(0, n) {
        for z in range(0, n) {
            do_something(x, y, z);
        }
    }
}
```

This is a common enough pattern for me that I wrote `iter_nd` to squash
iterators.

With `iter_nd` you could write the above code as:

```rust
for (x, y, z) in iter_3d(range(0,n), range(0,n), range(0,n)) {
    do_something(x, y, z);
}
```

Way simpler, way flatter, and most importantly you can easily define a function
that returns the result of your call to `iter_3d`, something that would be
*way* harder to do in the nested for-loop example.

## Example

```rust
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


```
