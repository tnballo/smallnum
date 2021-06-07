#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

// TODO: add f32 and f64 support

//! Integer optimization: macros return the smallest integer type capable of fitting a static bounds.
//! Both unsigned (e.g. macro input is maximum) and signed (e.g. macro input is maximum or minimum) numbers supported.
//! Saves memory on embedded devices.
//! `!#[no_std]`, `#![forbid(unsafe_code)]`, zero-cost.
//!
//! ### Example: Statically-sized Collection Index
//!
//! When the size of a collection is known at compile-time, the variable used to index it can be size-optimized.
//!
//! ```
//! use smallnum::{small_unsigned, SmallUnsigned};
//! use core::mem::size_of_val;
//!
//! const MAX_CAPACITY: usize = 500;
//! let my_array: [u8; MAX_CAPACITY] = [0; MAX_CAPACITY];
//!
//! let idx: usize = 5;
//! let small_idx: small_unsigned!(MAX_CAPACITY) = 5;
//!
//! assert_eq!(idx, small_idx.usize());                     // Equivalent values
//! assert_eq!(my_array[idx], my_array[small_idx.usize()]); // Equivalent collection indexing
//! assert!(size_of_val(&idx) > size_of_val(&small_idx));   // Memory savings (6 bytes on 64-bit)
//!
//! #[cfg(target_pointer_width = "64")]
//! assert_eq!(size_of_val(&idx), 8);
//!
//! #[cfg(target_pointer_width = "64")]
//! assert_eq!(size_of_val(&small_idx), 2);
//! ```
//! ### Example: Tree Node Metadata
//!
//! When the maximum capacity of a tree is known at compile-time, metadata stored in every node can be size-optimized.
//!
//! ```
//! use smallnum::small_unsigned;
//! use core::mem::size_of;
//!
//! const MAX_CAPACITY: usize = 500;
//!
//! // Regular node in a binary tree
//! struct BinTree<T> {
//!     value: T,
//!     left_child: Option<Box<BinTree<T>>>,
//!     right_child: Option<Box<BinTree<T>>>,
//!     subtree_size: Option<usize>,
//! }
//!
//! // Node with size-optimized metadata
//! struct SmallBinTree<T> {
//!     value: T,
//!     left_child: Option<Box<SmallBinTree<T>>>,
//!     right_child: Option<Box<SmallBinTree<T>>>,
//!     subtree_size: Option<small_unsigned!(MAX_CAPACITY)>,
//! }
//!
//! // Per-node memory savings (16 bytes on 64-bit)
//! assert!(size_of::<BinTree<u8>>() > size_of::<SmallBinTree<u8>>());
//!
//! #[cfg(target_pointer_width = "64")]
//! assert_eq!(size_of::<BinTree<u8>>(), 40);
//!
//! #[cfg(target_pointer_width = "64")]
//! assert_eq!(size_of::<SmallBinTree<u8>>(), 24);
//! ```

// Unsigned Normalization ----------------------------------------------------------------------------------------------

/// Convenience trait for unsigned normalization (e.g. `usize`)
pub trait SmallUnsigned {
    /// Get value of small unsigned as host register-width unsigned (e.g. `usize`)
    fn usize(&self) -> usize;
}

impl SmallUnsigned for u8 {
    fn usize(&self) -> usize {
        *self as usize
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
impl SmallUnsigned for u16 {
    fn usize(&self) -> usize {
        *self as usize
    }
}

#[cfg(any(
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
impl SmallUnsigned for u32 {
    fn usize(&self) -> usize {
        *self as usize
    }
}

#[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
impl SmallUnsigned for u64 {
    fn usize(&self) -> usize {
        *self as usize
    }
}

#[cfg(target_pointer_width = "128")]
impl SmallUnsigned for u128 {
    fn usize(&self) -> usize {
        *self as usize
    }
}

// Signed Normalization ------------------------------------------------------------------------------------------------

/// Convenience trait for signed normalization (e.g. `isize`)
pub trait SmallSigned {
    /// Get value of small signed as host register-width signed (e.g. `isize`)
    fn isize(&self) -> isize;
}

impl SmallSigned for i8 {
    fn isize(&self) -> isize {
        *self as isize
    }
}

#[cfg(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
impl SmallSigned for i16 {
    fn isize(&self) -> isize {
        *self as isize
    }
}

#[cfg(any(
    target_pointer_width = "32",
    target_pointer_width = "64",
    target_pointer_width = "128",
))]
impl SmallSigned for i32 {
    fn isize(&self) -> isize {
        *self as isize
    }
}

#[cfg(any(target_pointer_width = "64", target_pointer_width = "128",))]
impl SmallSigned for i64 {
    fn isize(&self) -> isize {
        *self as isize
    }
}

#[cfg(target_pointer_width = "128")]
impl SmallSigned for i128 {
    fn isize(&self) -> isize {
        *self as isize
    }
}

// Compile-time Bound Mapping ------------------------------------------------------------------------------------------

/// Return smallest unsigned type capable to representing input value (positive, i.e. maximum)
#[macro_export]
macro_rules! small_unsigned {
    ( $max:expr $(,)? ) => {
        <() as $crate::ShrinkUnsigned<
            { $max <= (core::u8::MAX as usize) },
            {
                if cfg!(any(
                    target_pointer_width = "16",
                    target_pointer_width = "32",
                    target_pointer_width = "64",
                    target_pointer_width = "128"
                )) {
                    $max <= (core::u16::MAX as usize)
                } else {
                    true
                }
            },
            {
                if cfg!(any(
                    target_pointer_width = "32",
                    target_pointer_width = "64",
                    target_pointer_width = "128"
                )) {
                    $max <= (core::u32::MAX as usize)
                } else {
                    true
                }
            },
            {
                if cfg!(any(
                    target_pointer_width = "64",
                    target_pointer_width = "128"
                )) {
                    $max <= (core::u64::MAX as usize)
                } else {
                    true
                }
            },
            {
                if cfg!(target_pointer_width = "128") {
                    $max <= (core::u128::MAX as usize)
                } else {
                    true
                }
            },
        >>::UnsignedType
    };
}

/// Helper trait for unsigned type mapping. Internal use only.
pub trait ShrinkUnsigned<
    const FITS_U8: bool,
    const FITS_U16: bool,
    const FITS_U32: bool,
    const FITS_U64: bool,
    const FITS_U128: bool,
>
{
    /// Smallest primitive type that can represent a bounded unsigned value
    type UnsignedType;
}

impl ShrinkUnsigned<true, true, true, true, true> for () {
    type UnsignedType = u8;
}

impl ShrinkUnsigned<false, true, true, true, true> for () {
    type UnsignedType = u16;
}

impl ShrinkUnsigned<false, false, true, true, true> for () {
    type UnsignedType = u32;
}

impl ShrinkUnsigned<false, false, false, true, true> for () {
    type UnsignedType = u64;
}

impl ShrinkUnsigned<false, false, false, false, true> for () {
    type UnsignedType = u128;
}

/// Return smallest signed type capable to representing input value (positive, i.e. maximum, or negative, i.e. minimum)
#[macro_export]
macro_rules! small_signed {
    ( $val:expr $(,)? ) => {
        <() as $crate::ShrinkSigned<
            { (core::i8::MIN as isize <= $val) && ($val <= (core::i8::MAX as isize)) },
            {
                if cfg!(any(
                    target_pointer_width = "16",
                    target_pointer_width = "32",
                    target_pointer_width = "64",
                    target_pointer_width = "128"
                )) {
                    (core::i16::MIN as isize <= $val) && ($val <= (core::i16::MAX as isize))
                } else {
                    true
                }
            },
            {
                if cfg!(any(
                    target_pointer_width = "32",
                    target_pointer_width = "64",
                    target_pointer_width = "128"
                )) {
                    (core::i32::MIN as isize <= $val) && ($val <= (core::i32::MAX as isize))
                } else {
                    true
                }
            },
            {
                if cfg!(any(
                    target_pointer_width = "64",
                    target_pointer_width = "128"
                )) {
                    (core::i64::MIN as isize <= $val) && ($val <= (core::i64::MAX as isize))
                } else {
                    true
                }
            },
            {
                if cfg!(target_pointer_width = "128") {
                    (core::i128::MIN as isize <= $val) && ($val <= (core::i128::MAX as isize))
                } else {
                    true
                }
            },
        >>::SmallSigned
    };
}

/// Helper trait for signed type mapping. Internal use only.
pub trait ShrinkSigned<
    const FITS_I8: bool,
    const FITS_I16: bool,
    const FITS_I32: bool,
    const FITS_I64: bool,
    const FITS_I128: bool,
>
{
    /// Smallest primitive type that can represent a bounded signed value
    type SmallSigned;
}

impl ShrinkSigned<true, true, true, true, true> for () {
    type SmallSigned = i8;
}

impl ShrinkSigned<false, true, true, true, true> for () {
    type SmallSigned = i16;
}

impl ShrinkSigned<false, false, true, true, true> for () {
    type SmallSigned = i32;
}

impl ShrinkSigned<false, false, false, true, true> for () {
    type SmallSigned = i64;
}

impl ShrinkSigned<false, false, false, false, true> for () {
    type SmallSigned = i128;
}

// Test ----------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::{SmallSigned, SmallUnsigned};
    use core::mem::size_of;
    use static_assertions::assert_type_eq_all;

    const MAX_VAL_UNSIGNED: usize = 512;
    const MAX_VAL_SIGNED: isize = 150;
    const MIN_VAL_SIGNED: isize = -150;

    #[test]
    fn unsigned_macro() {
        // Type mapping ------------------------------------------------------------------------------------------------

        type MaxType = small_unsigned!(MAX_VAL_UNSIGNED);
        type U8Type = small_unsigned!(200);
        type U16Type = small_unsigned!(500);
        type U32Type = small_unsigned!(100_000);

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        type U64Type = small_unsigned!(4_300_000_000);

        #[cfg(target_pointer_width = "128")]
        type U128Type = small_unsigned!(18_500_000_000_000_000_000);

        // Len Check ---------------------------------------------------------------------------------------------------

        assert_eq!(size_of::<MaxType>(), 2);
        assert_eq!(size_of::<U8Type>(), 1);
        assert_eq!(size_of::<U16Type>(), 2);
        assert_eq!(size_of::<U32Type>(), 4);

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        assert_eq!(size_of::<U64Type>(), 8);

        #[cfg(target_pointer_width = "128")]
        assert_eq!(size_of::<U128Type>(), 16);

        // Normalization Check -----------------------------------------------------------------------------------------

        let u8_num: U8Type = 200;
        let u16_num: U16Type = 500;
        let u32_num: U32Type = 100_000;

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        let u64_num: U64Type = 4_300_000_000;

        #[cfg(target_pointer_width = "128")]
        let u128_num: U128Type = 18_500_000_000_000_000_000;

        assert_eq!(u8_num.usize(), 200 as usize);
        assert_eq!(u16_num.usize(), 500 as usize);
        assert_eq!(u32_num.usize(), 100_000 as usize);

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        assert_eq!(u64_num.usize(), 4_300_000_000 as usize);

        #[cfg(target_pointer_width = "128")]
        assert_eq!(u128_num.usize(), 18_500_000_000_000_000_000 as usize);
    }

    #[test]
    fn signed_macro() {
        // Type mapping ------------------------------------------------------------------------------------------------

        type MaxType = small_signed!(MAX_VAL_SIGNED);
        type I8TypePos = small_signed!(100);
        type I16TypePos = small_signed!(500);
        type I32TypePos = small_signed!(50_000);

        type MinType = small_signed!(MIN_VAL_SIGNED);
        type I8TypeNeg = small_signed!(-100);
        type I16TypeNeg = small_signed!(-500);
        type I32TypeNeg = small_signed!(-50_000);

        assert_type_eq_all!(MaxType, MinType);
        assert_type_eq_all!(I8TypePos, I8TypeNeg);
        assert_type_eq_all!(I16TypePos, I16TypeNeg);
        assert_type_eq_all!(I32TypePos, I32TypeNeg);

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        type I64TypePos = small_signed!(2_200_000_000);

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        type I64TypeNeg = small_signed!(-2_200_000_000);

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        assert_type_eq_all!(I64TypePos, I64TypeNeg);

        #[cfg(target_pointer_width = "128")]
        type I128TypePos = small_signed!(9_300_000_000_000_000_000);

        #[cfg(target_pointer_width = "128")]
        type I128TypeNeg = small_signed!(-9_300_000_000_000_000_000);

        #[cfg(target_pointer_width = "128")]
        assert_type_eq_all!(I128TypePos, I128TypeNeg);

        // Len Check ---------------------------------------------------------------------------------------------------

        assert_eq!(size_of::<MaxType>(), 2);
        assert_eq!(size_of::<I8TypePos>(), 1);
        assert_eq!(size_of::<I16TypePos>(), 2);
        assert_eq!(size_of::<I32TypePos>(), 4);

        assert_eq!(size_of::<MaxType>(), size_of::<MinType>());
        assert_eq!(size_of::<I8TypePos>(), size_of::<I8TypeNeg>());
        assert_eq!(size_of::<I16TypePos>(), size_of::<I16TypeNeg>());
        assert_eq!(size_of::<I32TypePos>(), size_of::<I32TypeNeg>());

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        assert_eq!(size_of::<I64TypePos>(), 8);

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        assert_eq!(size_of::<I64TypePos>(), size_of::<I64TypeNeg>());

        #[cfg(target_pointer_width = "128")]
        assert_eq!(size_of::<I128TypePos>(), 16);

        #[cfg(target_pointer_width = "128")]
        assert_eq!(size_of::<I128TypePos>(), size_of()::<128TypeNeg>());

        // Normalization Check -----------------------------------------------------------------------------------------

        let i8_num_pos: I8TypePos = 100;
        let i16_num_pos: I16TypePos = 500;
        let i32_num_pos: I32TypePos = 50_000;

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        let i64_num_pos: I64TypePos = 2_200_000_000;

        #[cfg(target_pointer_width = "128")]
        let i128_num_pos: I128TypePos = 9_300_000_000_000_000_000;

        assert_eq!(i8_num_pos.isize(), 100 as isize);
        assert_eq!(i16_num_pos.isize(), 500 as isize);
        assert_eq!(i32_num_pos.isize(), 50_000 as isize);

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        assert_eq!(i64_num_pos.isize(), 2_200_000_000 as isize);

        #[cfg(target_pointer_width = "128")]
        assert_eq!(i128_num_pos.isize(), 9_300_000_000_000_000_000 as isize);

        let i8_num_neg: I8TypeNeg = -100;
        let i16_num_neg: I16TypeNeg = -500;
        let i32_num_neg: I32TypeNeg = -50_000;

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        let i64_num_neg: I64TypeNeg = -2_200_000_000;

        #[cfg(target_pointer_width = "128")]
        let i128_num_neg: I128TypeNeg = -9_300_000_000_000_000_000;

        assert_eq!(i8_num_neg.isize(), -100 as isize);
        assert_eq!(i16_num_neg.isize(), -500 as isize);
        assert_eq!(i32_num_neg.isize(), -50_000 as isize);

        #[cfg(any(target_pointer_width = "64", target_pointer_width = "128"))]
        assert_eq!(i64_num_neg.isize(), -2_200_000_000 as isize);

        #[cfg(target_pointer_width = "128")]
        assert_eq!(i128_num_neg.isize(), -9_300_000_000_000_000_000 as isize);
    }
}
