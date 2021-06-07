# smallnum

![crates.io](https://img.shields.io/crates/v/smallnum.svg)
![GitHub Actions](https://github.com/tnballo/smallnum/workflows/test/badge.svg)

Integer optimization: macros return the smallest integer type capable of fitting a static bounds.
Both unsigned (e.g. macro input is maximum) and signed (e.g. macro input is maximum or minimum) numbers supported.
Saves memory on embedded devices.
`!#[no_std]`, `#![forbid(unsafe_code)]`, zero-cost.

### Example: Statically-sized Collection Index

When the size of a collection is known at compile-time, the variable used to index it can be size-optimized.

```rust
use smallnum::{small_unsigned, SmallUnsigned};
use core::mem::size_of_val;

const MAX_CAPACITY: usize = 500;
let my_array: [u8; MAX_CAPACITY] = [0; MAX_CAPACITY];

let idx: usize = 5;
let small_idx: small_unsigned!(MAX_CAPACITY) = 5;

assert_eq!(idx, small_idx.usize());                     // Equivalent values
assert_eq!(my_array[idx], my_array[small_idx.usize()]); // Equivalent collection indexing
assert!(size_of_val(&idx) > size_of_val(&small_idx));   // Memory savings (6 bytes on 64-bit)

#[cfg(target_pointer_width = "64")]
assert_eq!(size_of_val(&idx), 8);

#[cfg(target_pointer_width = "64")]
assert_eq!(size_of_val(&small_idx), 2);
```
### Example: Tree Node Metadata

When the maximum capacity of a tree is known at compile-time, metadata stored in every node can be size-optimized.

```rust
use smallnum::small_unsigned;
use core::mem::size_of;

const MAX_CAPACITY: usize = 500;

// Regular node in a binary tree
struct BinTree<T> {
    value: T,
    left_child: Option<Box<BinTree<T>>>,
    right_child: Option<Box<BinTree<T>>>,
    subtree_size: Option<usize>,
}

// Node with size-optimized metadata
struct SmallBinTree<T> {
    value: T,
    left_child: Option<Box<SmallBinTree<T>>>,
    right_child: Option<Box<SmallBinTree<T>>>,
    subtree_size: Option<small_unsigned!(MAX_CAPACITY)>,
}

// Per-node memory savings (16 bytes on 64-bit)
assert!(size_of::<BinTree<u8>>() > size_of::<SmallBinTree<u8>>());

#[cfg(target_pointer_width = "64")]
assert_eq!(size_of::<BinTree<u8>>(), 40);

#[cfg(target_pointer_width = "64")]
assert_eq!(size_of::<SmallBinTree<u8>>(), 24);
```
