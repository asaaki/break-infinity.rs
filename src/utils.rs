use crate::{consts::*, Decimal};

/// Pads the given string with the fill string to the given max length.
pub fn pad_end(string: String, max_length: u32, fill_string: String) -> String {
    if f32::is_nan(max_length as f32) || f32::is_infinite(max_length as f32) {
        return string;
    }

    let length = string.chars().count() as u32;
    if length >= max_length {
        return string;
    }

    let mut filled = fill_string;
    if filled.is_empty() {
        filled = String::from(" ");
    }

    let fill_len = max_length - length;
    while filled.chars().count() < fill_len as usize {
        filled = format!("{}{}", filled, filled);
    }

    let truncated = if filled.chars().count() > fill_len as usize {
        String::from(&filled.as_str()[0..(fill_len as usize)])
    } else {
        filled
    };

    return string + truncated.as_str();
}

/// Formats the given number to the given number of significant digits.
pub fn to_fixed(num: f64, places: u32) -> String {
    format!("{:.*}", places as usize, num)
}

/// Formats the given number to the given number of significant digits and parses it back to a number.
pub fn to_fixed_num(num: f64, places: u32) -> f64 {
    to_fixed(num, places).parse::<f64>().unwrap()
}

/// Returns the power of 10 with the given exponent from the cache.
#[inline]
pub(crate) fn power_of_10(power: i32) -> f64 {
    CACHED_POWERS[(power - NUMBER_EXP_MIN) as usize]
}

/// Creates a new instance of Decimal with the given mantissa and exponent without normalizing them.
#[inline]
pub fn from_mantissa_exponent_no_normalize(mantissa: f64, exponent: f64) -> Decimal {
    Decimal { mantissa, exponent }
}

/// Creates a new instance of Decimal with the given mantissa and exponent with normalizing them.
#[inline]
pub fn from_mantissa_exponent(mantissa: f64, exponent: f64) -> Decimal {
    if !f64::is_finite(mantissa) || !f64::is_finite(exponent) {
        return NAN;
    }
    (Decimal { mantissa, exponent }).normalize()
}
