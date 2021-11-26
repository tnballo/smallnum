// Unsigned Normalization ----------------------------------------------------------------------------------------------

// TODO: make this const once stabilized: https://github.com/rust-lang/rust/issues/67792
// Then update $val -> $val.usize() so that macros can take any int type as input

/// Convenience trait for unsigned normalization (e.g. `usize`).
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

// Compile-time Bound Mapping ------------------------------------------------------------------------------------------

/// Return smallest unsigned type capable of representing input value (positive, i.e. maximum)
///
/// # Example
///
/// ```
/// use smallnum::{small_unsigned, SmallUnsigned};
/// use core::mem::size_of_val;
///
/// let idx: usize = 5;
/// let small_idx: small_unsigned!(500) = 5;
///
/// assert_eq!(idx, small_idx.usize());
/// assert!(size_of_val(&idx) > size_of_val(&small_idx));
/// ```
#[macro_export]
macro_rules! small_unsigned {
    ( $max:expr $(,)? ) => {
        <() as $crate::ShrinkUnsigned<
            { ($max as u128) <= (core::u8::MAX as u128) },
            { ($max as u128) <= (core::u16::MAX as u128) },
            { ($max as u128) <= (core::u32::MAX as u128) },
            { ($max as u128) <= (core::u64::MAX as u128) },
            { ($max as u128) <= (core::u128::MAX as u128) },
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

// Test ----------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {

    use crate::SmallUnsigned;
    use core::mem::size_of;

    const MAX_VAL_UNSIGNED: usize = 512;

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
}
