//! This module contains functions to convert between `break_infinity` and `break_infinity_extended`

use crate::*;
use break_infinity as bi;

/// Convert from `break_infinity` to `break_infinity_extended` (unsafe, but fast)
///
/// NOTE: This is a pretty dangerous operation (using unsafe transmutation),
/// but `break_infinity` doesn't have a way to get the mantissa and exponent directly;
/// for a safer alternative, see `from_bi_str`
pub fn from_bi(bi: &bi::Decimal) -> Decimal {
    // SAFETY: `break_infinity` and `break_infinity_extended` use identical
    // layout of their Decimal structs (same field names and types, in the same order);
    // theoretically rustc could reorder fields, let's cross fingers that it won't;
    // ideally both crates use repr(C) to guarantee that the layout is indeed identical
    unsafe { std::mem::transmute_copy(bi) }
}

/// Convert from `break_infinity` to `break_infinity_extended` (safe, but slow)
///
/// NOTE: This is the slow but probably safer way to convert from `break_infinity`
pub fn from_bi_str(bi: &bi::Decimal) -> Decimal {
    Decimal::from(bi.to_string().as_str())
}

/// Convert from `break_infinity_extended` to `break_infinity`
///
/// We have access to our fields, so we can just copy them over to `break_infinity`;
/// normalization is not needed, because our values are usually already normalized
pub fn to_bi(bie: Decimal) -> bi::Decimal {
    dbg!(bi::from_mantissa_exponent_no_normalize(
        bie.mantissa,
        bie.exponent
    ))
}
