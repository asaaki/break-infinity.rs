use crate::{consts, impl_from, utils::*, Decimal};

impl From<&str> for Decimal {
    /// Creates a new instance of Decimal from the given string.
    fn from(string: &str) -> Decimal {
        if string == "NaN" {
            return consts::NAN;
        };
        if let Some((mantissa, exponent)) = string.split_once('e') {
            return normalize_mantissa_and_exponent(
                mantissa.parse().unwrap(),
                exponent.parse().unwrap(),
            );
        };
        Decimal::new(string.parse().unwrap())
    }
}

// This allows converting virtually any number to a Decimal.

impl_from!(i8);
impl_from!(i16);
impl_from!(i32);
impl_from!(i64);
impl_from!(i128);
impl_from!(isize);
impl_from!(u8);
impl_from!(u16);
impl_from!(u32);
impl_from!(u64);
impl_from!(u128);
impl_from!(usize);
impl_from!(f32);
impl_from!(f64);
