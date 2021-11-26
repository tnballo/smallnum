// Signed Normalization ------------------------------------------------------------------------------------------------

// TODO: make this const once stabilized: https://github.com/rust-lang/rust/issues/67792
// Then update $val -> $val.isize() so that macros can take any int type as input

/// Convenience trait for signed normalization (e.g. `isize`).
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

/// Return smallest signed type capable of representing input value (positive, i.e. maximum, or negative, i.e. minimum)
///
/// # Example
///
/// ```
/// use smallnum::{small_signed, SmallSigned};
/// use core::mem::size_of_val;
///
/// let val_pos: isize = 5;
/// let small_val_pos: small_signed!(500) = 5;
///
/// assert_eq!(val_pos, small_val_pos.isize());
/// assert!(size_of_val(&val_pos) > size_of_val(&small_val_pos));
///
/// let val_neg: isize = -5;
/// let small_val_neg: small_signed!(-500) = -5;
///
/// assert_eq!(val_neg, small_val_neg.isize());
/// assert!(size_of_val(&val_neg) > size_of_val(&small_val_neg));
/// ```
#[macro_export]
macro_rules! small_signed {
    ( $val:expr $(,)? ) => {
        <() as $crate::ShrinkSigned<
            { (core::i8::MIN as i128 <= ($val as i128)) && (($val as i128) <= (core::i8::MAX as i128)) },
            { (core::i16::MIN as i128 <= ($val as i128)) && (($val as i128) <= (core::i16::MAX as i128)) },
            { (core::i32::MIN as i128 <= ($val as i128)) && (($val as i128) <= (core::i32::MAX as i128)) },
            { (core::i64::MIN as i128 <= ($val as i128)) && (($val as i128) <= (core::i64::MAX as i128)) },
            { (core::i128::MIN as i128 <= ($val as i128)) && (($val as i128) <= (core::i128::MAX as i128)) },
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

    use crate::SmallSigned;
    use core::mem::size_of;
    use static_assertions::assert_type_eq_all;

    const MAX_VAL_SIGNED: isize = 150;
    const MIN_VAL_SIGNED: isize = -150;

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
