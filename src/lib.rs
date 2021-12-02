#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
// TODO: add f32 and f64 support (see `std::num::flt2dec`, maybe?)

/*!
Compile-time size optimization for numeric primitives.
Macros return smallest numeric type capable of fitting a static bounds.
For unsigned integers, macro input is a maximum.
For signed integers, macro input may be a maximum or a minimum.

* Can save memory at zero runtime cost.
* Embedded-friendly: `!#[no_std]`.
* Safe: `#![forbid(unsafe_code)]`.

### What is this for?

Aiding the compiler in memory layout optimization (aka ["struct packing"](http://www.catb.org/esr/structure-packing/)).
For an example usecase where `smallnum` cuts RAM usage by 50%, please see [this part](https://github.com/tnballo/scapegoat#the-high_assurance-feature) of the `scapegoat` crate's documentation.

### Doesn't `#[repr(packed)]` already do that?

Not safely. The difference is subtle but important:

* `#[repr(packed)]` removes all padding between struct fields.
This incurs a performance penalty for misaligned accesses at best, and [causes undefined behavior](https://doc.rust-lang.org/nomicon/other-reprs.html#reprpacked) at worst.
It's something you generally want to avoid.

* `smallnum` aids packing while maintaining the target's native alignment, without removing padding.
It can actually improve [data cache] performance while being fully safe.

For extreme size optimization, you're free to combine `smallnum` with `#[repr(packed)]`.

### Example: Collection Index

When the size of a collection is known at compile-time, the variable used to index it can be size-optimized.

* **Target:** Value for collection/container index operator
* **Yield:**  `x * 1` where:
    * `x < size_of<usize>()`

```
use smallnum::{small_unsigned, SmallUnsigned};
use core::mem::size_of_val;

const MAX_SIZE: usize = 500;
let mut my_array: [u8; MAX_SIZE] = [0x00; MAX_SIZE];

let idx: usize = 5;
let small_idx: small_unsigned!(MAX_SIZE) = 5;

// Equivalent values
my_array[idx] = 0xff;
assert_eq!(my_array[idx], my_array[small_idx.usize()]);

// Memory savings (6 bytes on a 64-bit system)
#[cfg(target_pointer_width = "64")]
assert_eq!(size_of_val(&idx) - size_of_val(&small_idx), 6);
```

Notice that having the trait `SmallUnsigned` in scope allows `small_idx.usize()` to be called.
This function returns a `usize` for convenient indexing, regardless of which type the macro selected (`u16` in the above example, hence the 6 byte savings over a 64-bit host's `u64`).

### Example: Tree Node Metadata

When the maximum capacity of a tree is known at compile time, metadata stored in every node can be size-optimized.

* **Target:** Internal metatdata
* **Yield:**  `x * n` where:
    * `x <= size_of<usize>()`
    * `n == node_cnt`

```
use smallnum::small_unsigned;
use core::mem::size_of;

const MAX_CAPACITY: usize = 50_000;

// Regular node in a binary tree
pub struct BinTree<T> {
    value: T,
    left_child: Option<Box<BinTree<T>>>,
    right_child: Option<Box<BinTree<T>>>,
    subtree_size: usize,
}

// Node with size-optimized metadata
pub struct SmallBinTree<T> {
    value: T,
    left_child: Option<Box<SmallBinTree<T>>>,
    right_child: Option<Box<SmallBinTree<T>>>,
    subtree_size: small_unsigned!(MAX_CAPACITY),
}

// Per-node memory savings (8 bytes on a 64-bit system)
#[cfg(target_pointer_width = "64")]
assert_eq!(size_of::<BinTree<i16>>() - size_of::<SmallBinTree<i16>>(), 8);
```

### Example: Index-based Graphs

When implementing an [{index,arena}-based graph](http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/) whose maximum capacity is known at compile-time, indexes stored in every structure (edge or node) can be size-optimized.

* **Target:** Internal "pointer" representation
* **Yield:**  `(x + y) * n` where:
    * `x <= size_of<usize>()`
    * `y <= size_of<Option<usize>>()`
    * `n == edge_cnt`

```
use smallnum::small_unsigned;
use core::mem::size_of;

const MAX_CAPACITY: usize = 50_000;

// Based on "Modeling graphs in Rust using vector indices" by Niko Matsakis (April 2015)
// http://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/

// Unoptimized indexes
pub type NodeIdx = usize;
pub type EdgeIdx = usize;

pub struct EdgeData {
    target: NodeIdx,
    next_outgoing_edge: Option<EdgeIdx>
}

// Optimized indexes
pub type SmallNodeIdx = small_unsigned!(MAX_CAPACITY);
pub type SmallEdgeIdx = small_unsigned!(MAX_CAPACITY);

pub struct SmallEdgeData {
    target: SmallNodeIdx,
    next_outgoing_edge: Option<SmallEdgeIdx>
}

// Per-edge memory savings (18 bytes on a 64-bit system)
#[cfg(target_pointer_width = "64")]
assert_eq!(size_of::<EdgeData>() - size_of::<SmallEdgeData>(), 18);
```

### Advanced Examples

See [`examples/`](https://github.com/tnballo/smallnum/tree/master/examples) directory, `cargo run --example <file_name>`.

### Macro <-> Type Selection Set

* [`small_unsigned!`](crate::small_unsigned) <-> (`u8`, `u16`, `u32`, `u64`, `u128`)
* [`small_signed!`](crate::small_signed) <-> (`i8`, `i16`, `i32`, `i64`, `i128`)

### License and Contributing

Licensed under the [MIT license](https://github.com/tnballo/smallnum/blob/master/LICENSE).
Contributions are welcome!
*/

mod unsigned;
pub use crate::unsigned::{ShrinkUnsigned, SmallUnsigned, SmallUnsignedLabel};

mod signed;
pub use crate::signed::{ShrinkSigned, SmallSigned, SmallSignedLabel};